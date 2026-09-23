//! CLI adapters: turn each CLI's NDJSON line format into `AgentEvent`s and build its headless args.

pub(crate) mod agy;
pub(crate) mod claude;
pub(crate) mod opencode;

use serde_json::Value;

use super::events::AgentEvent;
use crate::application::agents::AgentAdapterKind;

pub(crate) type LineParser = fn(&str) -> Vec<AgentEvent>;

pub(super) fn str_field(value: &Value, key: &str) -> String {
  value.get(key).and_then(Value::as_str).unwrap_or("").to_string()
}

pub(super) fn u64_field(value: &Value, key: &str) -> u64 {
  value.get(key).and_then(Value::as_u64).unwrap_or(0)
}

pub(crate) struct TurnRequest<'a> {
  pub(crate) prompt: &'a str,
  pub(crate) resume_session_id: Option<&'a str>,
  pub(crate) skip_permissions: bool,
  /// MCP config file handed to CLIs that accept one per invocation (claude).
  pub(crate) mcp_config: Option<&'a str>,
}

pub(crate) fn parser_for(kind: AgentAdapterKind) -> Option<LineParser> {
  match kind {
    AgentAdapterKind::AgyStreamJson => Some(agy::parse_line),
    AgentAdapterKind::ClaudeStreamJson => Some(claude::parse_line),
    AgentAdapterKind::OpencodeJson => Some(opencode::parse_line),
    AgentAdapterKind::Pty => None,
  }
}

/// Headless argv for one turn. The prompt travels through argv, so Windows' 32,767-char
/// command-line limit applies; a leading space keeps a prompt starting with `-` from reading as a flag.
pub(crate) fn build_args(kind: AgentAdapterKind, request: &TurnRequest<'_>) -> Option<Vec<String>> {
  let prompt = if request.prompt.starts_with('-') {
    format!(" {}", request.prompt)
  } else {
    request.prompt.to_string()
  };
  let mut args: Vec<String> = match kind {
    AgentAdapterKind::AgyStreamJson => vec!["-p".into(), prompt, "--output-format".into(), "stream-json".into()],
    AgentAdapterKind::ClaudeStreamJson => vec![
      "-p".into(),
      prompt,
      "--output-format".into(),
      "stream-json".into(),
      "--verbose".into(),
    ],
    AgentAdapterKind::OpencodeJson => vec!["run".into(), prompt, "--format".into(), "json".into()],
    AgentAdapterKind::Pty => return None,
  };
  if let Some(session_id) = request.resume_session_id {
    let flag = match kind {
      AgentAdapterKind::AgyStreamJson => "--conversation",
      AgentAdapterKind::OpencodeJson => "--session",
      _ => "--resume",
    };
    args.push(flag.into());
    args.push(session_id.into());
  }
  if request.skip_permissions {
    let flag = if kind == AgentAdapterKind::OpencodeJson { "--auto" } else { "--dangerously-skip-permissions" };
    args.push(flag.into());
  } else if kind == AgentAdapterKind::AgyStreamJson {
    // Without unlimited access, agy may still edit files but asks before anything riskier.
    args.push("--mode".into());
    args.push("accept-edits".into());
  }
  if let (Some(config), AgentAdapterKind::ClaudeStreamJson) = (request.mcp_config, kind) {
    args.push("--mcp-config".into());
    args.push(config.into());
    // Loom's own tools never need a prompt; everything else keeps the member's permission mode.
    args.push("--allowedTools".into());
    args.push("mcp__loom".into());
  }
  Some(args)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
  }

  fn request<'a>(resume: Option<&'a str>, skip: bool) -> TurnRequest<'a> {
    TurnRequest { prompt: "hi", resume_session_id: resume, skip_permissions: skip, mcp_config: None }
  }

  #[test]
  fn agy_fresh_turn_defaults_to_accept_edits() {
    assert_eq!(
      build_args(AgentAdapterKind::AgyStreamJson, &request(None, false)),
      Some(strings(&["-p", "hi", "--output-format", "stream-json", "--mode", "accept-edits"]))
    );
  }

  #[test]
  fn agy_resume_with_skip_permissions() {
    assert_eq!(
      build_args(AgentAdapterKind::AgyStreamJson, &request(Some("c1"), true)),
      Some(strings(&[
        "-p",
        "hi",
        "--output-format",
        "stream-json",
        "--conversation",
        "c1",
        "--dangerously-skip-permissions"
      ]))
    );
  }

  #[test]
  fn claude_resume_with_mcp_config() {
    let request = TurnRequest { prompt: "hi", resume_session_id: Some("s1"), skip_permissions: false, mcp_config: Some("m.json") };
    assert_eq!(
      build_args(AgentAdapterKind::ClaudeStreamJson, &request),
      Some(strings(&[
        "-p",
        "hi",
        "--output-format",
        "stream-json",
        "--verbose",
        "--resume",
        "s1",
        "--mcp-config",
        "m.json",
        "--allowedTools",
        "mcp__loom"
      ]))
    );
  }

  #[test]
  fn dash_prompt_is_not_a_flag() {
    let request = TurnRequest { prompt: "-v please", resume_session_id: None, skip_permissions: true, mcp_config: None };
    let args = build_args(AgentAdapterKind::ClaudeStreamJson, &request).unwrap();
    assert_eq!(args[1], " -v please");
  }

  #[test]
  fn opencode_resume_with_auto_approve() {
    assert_eq!(
      build_args(AgentAdapterKind::OpencodeJson, &request(Some("ses_1"), true)),
      Some(strings(&["run", "hi", "--format", "json", "--session", "ses_1", "--auto"]))
    );
  }

  #[test]
  fn pty_has_no_headless_args() {
    assert_eq!(build_args(AgentAdapterKind::Pty, &request(None, false)), None);
    assert!(parser_for(AgentAdapterKind::Pty).is_none());
  }
}
