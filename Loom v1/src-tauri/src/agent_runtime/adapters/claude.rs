//! Claude Code (`claude -p … --output-format stream-json --verbose`) line parser.

use serde_json::Value;

use super::{str_field, u64_field};
use crate::agent_runtime::events::{AgentEvent, AgentUsage};

pub(crate) fn parse_line(line: &str) -> Vec<AgentEvent> {
  let Ok(value) = serde_json::from_str::<Value>(line.trim()) else {
    return Vec::new();
  };
  match value.get("type").and_then(Value::as_str) {
    Some("system") if value.get("subtype").and_then(Value::as_str) == Some("init") => value
      .get("session_id")
      .and_then(Value::as_str)
      .map(|id| vec![AgentEvent::SessionStarted { session_id: id.to_string() }])
      .unwrap_or_default(),
    Some("assistant") => content_blocks(&value)
      .iter()
      .filter_map(|block| match block.get("type").and_then(Value::as_str) {
        Some("text") => block
          .get("text")
          .and_then(Value::as_str)
          .filter(|text| !text.is_empty())
          .map(|text| AgentEvent::TextDelta { text: text.to_string() }),
        Some("tool_use") => Some(AgentEvent::ToolStarted {
          id: str_field(block, "id"),
          name: str_field(block, "name"),
          input: block.get("input").cloned().unwrap_or(Value::Null),
        }),
        _ => None,
      })
      .collect(),
    Some("user") => content_blocks(&value)
      .iter()
      .filter(|block| block.get("type").and_then(Value::as_str) == Some("tool_result"))
      .map(|block| AgentEvent::ToolFinished {
        id: str_field(block, "tool_use_id"),
        is_error: block.get("is_error").and_then(Value::as_bool).unwrap_or(false),
      })
      .collect(),
    Some("result") => parse_result(&value),
    _ => Vec::new(),
  }
}

fn content_blocks(value: &Value) -> &[Value] {
  value
    .pointer("/message/content")
    .and_then(Value::as_array)
    .map(Vec::as_slice)
    .unwrap_or(&[])
}

fn parse_result(value: &Value) -> Vec<AgentEvent> {
  let subtype = value.get("subtype").and_then(Value::as_str).unwrap_or("");
  let is_error = value.get("is_error").and_then(Value::as_bool).unwrap_or(false) || subtype != "success";
  if is_error {
    let message = value
      .get("result")
      .and_then(Value::as_str)
      .filter(|text| !text.trim().is_empty())
      .map(str::to_string)
      .unwrap_or_else(|| format!("claude finished with {subtype}"));
    return vec![AgentEvent::TurnFailed { message }];
  }
  vec![AgentEvent::TurnCompleted {
    session_id: value.get("session_id").and_then(Value::as_str).map(str::to_string),
    text: str_field(value, "result"),
    usage: value.get("usage").map(|usage| AgentUsage {
      input_tokens: u64_field(usage, "input_tokens"),
      output_tokens: u64_field(usage, "output_tokens"),
      thinking_tokens: usage
        .pointer("/output_tokens_details/thinking_tokens")
        .and_then(Value::as_u64)
        .unwrap_or(0),
      cache_read_tokens: u64_field(usage, "cache_read_input_tokens"),
      cost_usd: value.get("total_cost_usd").and_then(Value::as_f64),
    }),
  }]
}

#[cfg(test)]
mod tests {
  use super::*;

  const FIXTURE: &str = include_str!("fixtures/claude_tool_call.jsonl");

  fn events() -> Vec<AgentEvent> {
    FIXTURE.lines().flat_map(parse_line).collect()
  }

  #[test]
  fn maps_full_turn() {
    assert_eq!(
      events(),
      vec![
        AgentEvent::SessionStarted { session_id: "sess-1".into() },
        AgentEvent::TextDelta { text: "Let me look.".into() },
        AgentEvent::ToolStarted {
          id: "toolu_1".into(),
          name: "Bash".into(),
          input: serde_json::json!({ "command": "ls" }),
        },
        AgentEvent::ToolFinished { id: "toolu_1".into(), is_error: false },
        AgentEvent::TextDelta { text: "There is one file: a.txt.".into() },
        AgentEvent::TurnCompleted {
          session_id: Some("sess-1".into()),
          text: "There is one file: a.txt.".into(),
          usage: Some(AgentUsage {
            input_tokens: 120,
            output_tokens: 30,
            thinking_tokens: 7,
            cache_read_tokens: 4000,
            cost_usd: Some(0.0123),
          }),
        },
      ]
    );
  }

  #[test]
  fn error_result_is_failure() {
    let line = r#"{"type":"result","subtype":"error_max_turns","is_error":true,"session_id":"s"}"#;
    assert_eq!(
      parse_line(line),
      vec![AgentEvent::TurnFailed { message: "claude finished with error_max_turns".into() }]
    );
  }
}
