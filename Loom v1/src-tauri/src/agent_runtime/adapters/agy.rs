//! Antigravity CLI (`agy -p … --output-format stream-json`) line parser.

use serde_json::Value;

use super::{str_field, u64_field};
use crate::agent_runtime::events::{AgentEvent, AgentUsage};

pub(crate) fn parse_line(line: &str) -> Vec<AgentEvent> {
  let Ok(value) = serde_json::from_str::<Value>(line.trim()) else {
    return Vec::new();
  };
  match value.get("event").and_then(Value::as_str) {
    Some("init") => value
      .get("conversation_id")
      .and_then(Value::as_str)
      .map(|id| vec![AgentEvent::SessionStarted { session_id: id.to_string() }])
      .unwrap_or_default(),
    Some("step_update") => value.get("step_update").map(parse_step).unwrap_or_default(),
    Some("result") => value.get("result").map(parse_result).unwrap_or_default(),
    _ => Vec::new(),
  }
}

fn parse_step(step: &Value) -> Vec<AgentEvent> {
  let state = step.get("state").and_then(Value::as_str).unwrap_or("");
  match step.get("step_type").and_then(Value::as_str) {
    Some("agent_response") => step
      .get("text_delta")
      .and_then(Value::as_str)
      .filter(|text| !text.is_empty())
      .map(|text| vec![AgentEvent::TextDelta { text: text.to_string() }])
      .unwrap_or_default(),
    // A long tool may report ACTIVE more than once; consumers treat ToolStarted as idempotent by id.
    Some("tool") => {
      let id = format!("step-{}", u64_field(step, "step_index"));
      match state {
        "ACTIVE" => vec![AgentEvent::ToolStarted {
          id,
          name: str_field(step, "tool_name"),
          input: step.pointer("/tool_info/parameters").cloned().unwrap_or(Value::Null),
        }],
        "DONE" => vec![AgentEvent::ToolFinished { id, is_error: false }],
        "ERROR" => vec![AgentEvent::ToolFinished { id, is_error: true }],
        _ => Vec::new(),
      }
    }
    _ => Vec::new(),
  }
}

fn parse_result(result: &Value) -> Vec<AgentEvent> {
  let status = result.get("status").and_then(Value::as_str).unwrap_or("");
  if status != "SUCCESS" {
    let message = match result.get("error") {
      Some(Value::String(text)) => text.clone(),
      Some(other) => other.to_string(),
      None => format!("agy finished with status {status}"),
    };
    return vec![AgentEvent::TurnFailed { message }];
  }
  vec![AgentEvent::TurnCompleted {
    session_id: result.get("conversation_id").and_then(Value::as_str).map(str::to_string),
    text: str_field(result, "response"),
    usage: result.get("usage").map(|usage| AgentUsage {
      input_tokens: u64_field(usage, "input_tokens"),
      output_tokens: u64_field(usage, "output_tokens"),
      thinking_tokens: u64_field(usage, "thinking_tokens"),
      cache_read_tokens: u64_field(usage, "cache_read_tokens"),
      cost_usd: None,
    }),
  }]
}

#[cfg(test)]
mod tests {
  use super::*;

  const FIXTURE: &str = include_str!("fixtures/agy_tool_call.jsonl");
  const SESSION: &str = "e9beba6d-92d2-43bc-bc0d-600f70c939fc";
  const REPLY: &str = "I have listed the files in the current directory (which is currently empty). \n\nDone.\n";

  fn events() -> Vec<AgentEvent> {
    FIXTURE.lines().flat_map(parse_line).collect()
  }

  #[test]
  fn starts_with_session() {
    assert_eq!(events()[0], AgentEvent::SessionStarted { session_id: SESSION.to_string() });
  }

  #[test]
  fn text_deltas_join_into_reply() {
    let text: String = events()
      .iter()
      .filter_map(|event| match event {
        AgentEvent::TextDelta { text } => Some(text.as_str()),
        _ => None,
      })
      .collect();
    assert_eq!(text, REPLY);
  }

  #[test]
  fn reports_tool_start_with_parameters_then_finish() {
    let tools: Vec<AgentEvent> = events()
      .into_iter()
      .filter(|event| matches!(event, AgentEvent::ToolStarted { .. } | AgentEvent::ToolFinished { .. }))
      .collect();
    assert_eq!(
      tools,
      vec![
        AgentEvent::ToolStarted {
          id: "step-2".into(),
          name: "run_command".into(),
          input: serde_json::json!({ "CommandLine": "Get-ChildItem" }),
        },
        AgentEvent::ToolFinished { id: "step-2".into(), is_error: false },
      ]
    );
  }

  #[test]
  fn ends_with_completed_turn_and_usage() {
    assert_eq!(
      events().last().unwrap(),
      &AgentEvent::TurnCompleted {
        session_id: Some(SESSION.to_string()),
        text: REPLY.to_string(),
        usage: Some(AgentUsage {
          input_tokens: 10459,
          output_tokens: 891,
          thinking_tokens: 803,
          cache_read_tokens: 16183,
          cost_usd: None,
        }),
      }
    );
  }

  #[test]
  fn tool_error_state_is_a_failed_finish() {
    let line = r#"{"event":"step_update","step_update":{"step_index":4,"state":"ERROR","step_type":"tool","tool_name":"call_mcp_tool"}}"#;
    assert_eq!(parse_line(line), vec![AgentEvent::ToolFinished { id: "step-4".into(), is_error: true }]);
  }

  #[test]
  fn non_success_status_is_failure() {
    let line = r#"{"event":"result","result":{"conversation_id":"c","status":"ERROR","error":"auth required"}}"#;
    assert_eq!(parse_line(line), vec![AgentEvent::TurnFailed { message: "auth required".into() }]);
  }

  #[test]
  fn ignores_blank_and_non_json_lines() {
    assert!(parse_line("").is_empty());
    assert!(parse_line("Loaded cached credentials.").is_empty());
  }
}
