//! OpenCode (`opencode run … --format json`) line parser.

use serde_json::Value;

use super::{str_field, u64_field};
use crate::agent_runtime::events::{AgentEvent, AgentUsage};

pub(crate) fn parse_line(line: &str) -> Vec<AgentEvent> {
  let Ok(value) = serde_json::from_str::<Value>(line.trim()) else {
    return Vec::new();
  };
  let part = value.get("part").cloned().unwrap_or(Value::Null);
  let session_id = value.get("sessionID").and_then(Value::as_str).map(str::to_string);
  match value.get("type").and_then(Value::as_str) {
    Some("step_start") => session_id
      .map(|session_id| vec![AgentEvent::SessionStarted { session_id }])
      .unwrap_or_default(),
    Some("text") => part
      .get("text")
      .and_then(Value::as_str)
      .filter(|text| !text.is_empty())
      .map(|text| vec![AgentEvent::TextDelta { text: text.to_string() }])
      .unwrap_or_default(),
    // OpenCode reports a tool once it has finished, so start and finish arrive together.
    Some("tool_use") => {
      let id = str_field(&part, "callID");
      let status = part.pointer("/state/status").and_then(Value::as_str).unwrap_or("");
      vec![
        AgentEvent::ToolStarted {
          id: id.clone(),
          name: str_field(&part, "tool"),
          input: part.pointer("/state/input").cloned().unwrap_or(Value::Null),
        },
        AgentEvent::ToolFinished { id, is_error: status == "error" },
      ]
    }
    Some("step_finish") if part.get("reason").and_then(Value::as_str) == Some("stop") => {
      let tokens = part.get("tokens").cloned().unwrap_or(Value::Null);
      vec![AgentEvent::TurnCompleted {
        session_id,
        // Empty: the sink uses the streamed text parts as the final answer.
        text: String::new(),
        usage: Some(AgentUsage {
          input_tokens: u64_field(&tokens, "input"),
          output_tokens: u64_field(&tokens, "output"),
          thinking_tokens: u64_field(&tokens, "reasoning"),
          cache_read_tokens: tokens.pointer("/cache/read").and_then(Value::as_u64).unwrap_or(0),
          cost_usd: part.get("cost").and_then(Value::as_f64),
        }),
      }]
    }
    Some("error") => {
      let message = value
        .pointer("/error/data/message")
        .or_else(|| value.pointer("/error/message"))
        .and_then(Value::as_str)
        .map(str::to_string)
        .unwrap_or_else(|| value.get("error").map(Value::to_string).unwrap_or_else(|| "opencode failed".to_string()));
      vec![AgentEvent::TurnFailed { message }]
    }
    _ => Vec::new(),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const FIXTURE: &str = include_str!("fixtures/opencode_tool_call.jsonl");

  fn events() -> Vec<AgentEvent> {
    FIXTURE.lines().flat_map(parse_line).collect()
  }

  #[test]
  fn maps_tool_text_and_stop() {
    let all = events();
    assert_eq!(all[0], AgentEvent::SessionStarted { session_id: "ses_f31540539ffeOWqIQQzbdIT0c1".into() });
    assert!(all.contains(&AgentEvent::ToolStarted {
      id: "call-f577fdbd".into(),
      name: "glob".into(),
      input: serde_json::json!({ "pattern": "**/*" }),
    }));
    assert!(all.contains(&AgentEvent::ToolFinished { id: "call-f577fdbd".into(), is_error: false }));
    assert!(all.contains(&AgentEvent::TextDelta { text: "Done. The directory has one file: notes.txt.".into() }));
    match all.last().unwrap() {
      AgentEvent::TurnCompleted { usage: Some(usage), .. } => {
        assert_eq!(usage.cache_read_tokens, 168480);
        assert_eq!(usage.cost_usd, Some(0.0042));
      }
      other => panic!("unexpected last event: {other:?}"),
    }
  }

  #[test]
  fn tool_calls_step_finish_is_not_terminal() {
    let line = r#"{"type":"step_finish","sessionID":"s","part":{"reason":"tool-calls"}}"#;
    assert!(parse_line(line).is_empty());
  }

  #[test]
  fn error_event_fails_the_turn() {
    let line = r#"{"type":"error","sessionID":"s","error":{"name":"ProviderAuthError","data":{"message":"No API key"}}}"#;
    assert_eq!(parse_line(line), vec![AgentEvent::TurnFailed { message: "No API key".into() }]);
  }
}
