//! Structured agent events: the one shape every adapter produces and chat consumes.

use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AgentUsage {
  pub(crate) input_tokens: u64,
  pub(crate) output_tokens: u64,
  pub(crate) thinking_tokens: u64,
  pub(crate) cache_read_tokens: u64,
  pub(crate) cost_usd: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub(crate) enum AgentEvent {
  SessionStarted { session_id: String },
  TextDelta { text: String },
  ToolStarted { id: String, name: String, input: Value },
  ToolFinished { id: String, is_error: bool },
  TurnCompleted { session_id: Option<String>, text: String, usage: Option<AgentUsage> },
  TurnFailed { message: String },
}

impl AgentEvent {
  /// A turn ends on exactly one of these.
  pub(crate) fn is_terminal(&self) -> bool {
    matches!(self, AgentEvent::TurnCompleted { .. } | AgentEvent::TurnFailed { .. })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn serializes_with_kind_tag_and_camel_case_fields() {
    let event = AgentEvent::SessionStarted { session_id: "abc".to_string() };
    let json = serde_json::to_value(&event).unwrap();
    assert_eq!(json, serde_json::json!({ "kind": "sessionStarted", "sessionId": "abc" }));
  }

  #[test]
  fn only_completed_and_failed_are_terminal() {
    assert!(AgentEvent::TurnFailed { message: "x".into() }.is_terminal());
    assert!(!AgentEvent::TextDelta { text: "x".into() }.is_terminal());
  }
}
