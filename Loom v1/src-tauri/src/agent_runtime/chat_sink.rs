//! Chat sink: feeds structured agent events into the existing stream/final chat pipeline.

use std::sync::Arc;

use super::events::{AgentEvent, AgentUsage};
use super::runner::AgentEventSink;
use crate::contracts::terminal_message::{TerminalMessageMeta, TerminalMessagePayload};
use crate::now_millis;
use crate::ports::terminal_message::TerminalMessagePipeline;

#[derive(Clone, Debug)]
pub(crate) struct ChatTurnContext {
  pub(crate) member_id: String,
  pub(crate) workspace_id: String,
  pub(crate) conversation_id: String,
  pub(crate) conversation_type: String,
  pub(crate) sender_id: String,
  pub(crate) sender_name: String,
}

#[derive(Debug, Default, PartialEq)]
pub(crate) struct TurnOutcome {
  pub(crate) session_id: Option<String>,
  pub(crate) usage: Option<AgentUsage>,
  pub(crate) failed: bool,
  /// Text posted as the final message (the answer, or the error).
  pub(crate) final_text: String,
}

/// Sees every event before the sink handles it (used to emit `agent-event` to the UI).
pub(crate) type EventObserver = Box<dyn FnMut(&ChatTurnContext, &str, &AgentEvent) + Send>;

pub(crate) struct ChatSink {
  context: ChatTurnContext,
  span_id: String,
  pipeline: Arc<dyn TerminalMessagePipeline>,
  observer: Option<EventObserver>,
  seq: u64,
  streamed: String,
  break_before_next_text: bool,
  pub(crate) outcome: TurnOutcome,
}

impl ChatSink {
  pub(crate) fn new(
    context: ChatTurnContext,
    span_id: String,
    pipeline: Arc<dyn TerminalMessagePipeline>,
    observer: Option<EventObserver>,
  ) -> Self {
    Self {
      context,
      span_id,
      pipeline,
      observer,
      seq: 0,
      streamed: String::new(),
      break_before_next_text: false,
      outcome: TurnOutcome::default(),
    }
  }

  fn payload(&mut self, content: String, mode: &str) -> TerminalMessagePayload {
    self.seq += 1;
    TerminalMessagePayload {
      terminal_id: format!("agent:{}", self.context.member_id),
      member_id: Some(self.context.member_id.clone()),
      workspace_id: Some(self.context.workspace_id.clone()),
      conversation_id: Some(self.context.conversation_id.clone()),
      conversation_type: Some(self.context.conversation_type.clone()),
      sender_id: Some(self.context.sender_id.clone()),
      sender_name: Some(self.context.sender_name.clone()),
      seq: self.seq,
      timestamp: now_millis().unwrap_or(0),
      content,
      message_type: "info".to_string(),
      source: "chat".to_string(),
      mode: mode.to_string(),
      span_id: Some(self.span_id.clone()),
      meta: Some(TerminalMessageMeta { command: None, line_count: None, cursor: None, start_row: None, end_row: None }),
    }
  }

  /// Channel replies must mention the sender or the chat side ignores them (same rule as `build_semantic_payload`).
  fn with_channel_mention(&self, content: String) -> String {
    let sender = self.context.sender_name.trim();
    if self.context.conversation_type != "channel" || sender.is_empty() {
      return content;
    }
    let mention = format!("@{sender}");
    if content.trim_start().starts_with(&mention) {
      content
    } else {
      format!("{mention} {content}")
    }
  }

  fn post_final(&mut self, content: String) {
    let content = if content.trim().is_empty() { "(no reply)".to_string() } else { content };
    self.outcome.final_text = content.clone();
    let content = self.with_channel_mention(content);
    let payload = self.payload(content, "final");
    if let Err(err) = self.pipeline.process_final(payload) {
      log::warn!("agent chat append failed member_id={} err={err}", self.context.member_id);
    }
  }
}

impl AgentEventSink for ChatSink {
  fn on_event(&mut self, event: AgentEvent) {
    if let Some(observer) = self.observer.as_mut() {
      observer(&self.context, &self.span_id, &event);
    }
    match event {
      AgentEvent::SessionStarted { session_id } => self.outcome.session_id = Some(session_id),
      AgentEvent::TextDelta { text } => {
        let delta = if self.break_before_next_text && !self.streamed.is_empty() { format!("\n\n{text}") } else { text };
        self.break_before_next_text = false;
        self.streamed.push_str(&delta);
        let payload = self.payload(delta, "delta");
        if let Err(err) = self.pipeline.process_stream(payload) {
          log::warn!("agent stream emit failed member_id={} err={err}", self.context.member_id);
        }
      }
      AgentEvent::ToolStarted { .. } => self.break_before_next_text = true,
      AgentEvent::ToolFinished { .. } => {}
      AgentEvent::TurnCompleted { session_id, text, usage } => {
        if session_id.is_some() {
          self.outcome.session_id = session_id;
        }
        self.outcome.usage = usage;
        let content = if text.trim().is_empty() { std::mem::take(&mut self.streamed) } else { text };
        self.post_final(content.trim_end().to_string());
      }
      AgentEvent::TurnFailed { message } => {
        self.outcome.failed = true;
        self.post_final(format!("Agent error: {message}"));
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use std::sync::Mutex;

  use super::*;
  use crate::agent_runtime::adapters::agy;
  use crate::ports::message_service::TerminalMessageAppendResult;

  const FIXTURE: &str = include_str!("adapters/fixtures/agy_tool_call.jsonl");

  #[derive(Default)]
  struct RecordingPipeline {
    calls: Mutex<Vec<(String, String)>>,
  }

  impl TerminalMessagePipeline for RecordingPipeline {
    fn process_stream(&self, payload: TerminalMessagePayload) -> Result<(), String> {
      self.calls.lock().unwrap().push((payload.mode, payload.content));
      Ok(())
    }
    fn process_final(&self, payload: TerminalMessagePayload) -> Result<TerminalMessageAppendResult, String> {
      self.calls.lock().unwrap().push((payload.mode, payload.content));
      Ok(TerminalMessageAppendResult::skipped())
    }
  }

  fn context(conversation_type: &str) -> ChatTurnContext {
    ChatTurnContext {
      member_id: "m1".into(),
      workspace_id: "w1".into(),
      conversation_id: "c1".into(),
      conversation_type: conversation_type.into(),
      sender_id: "u1".into(),
      sender_name: "Mira".into(),
    }
  }

  fn run(conversation_type: &str, lines: &str) -> (Arc<RecordingPipeline>, ChatSink) {
    let pipeline = Arc::new(RecordingPipeline::default());
    let mut sink = ChatSink::new(context(conversation_type), "span-1".into(), pipeline.clone(), None);
    for event in lines.lines().flat_map(agy::parse_line) {
      sink.on_event(event);
    }
    (pipeline, sink)
  }

  #[test]
  fn streams_deltas_then_posts_clean_final() {
    let (pipeline, sink) = run("dm", FIXTURE);
    let calls = pipeline.calls.lock().unwrap();
    assert!(calls[..calls.len() - 1].iter().all(|(mode, _)| mode == "delta"));
    let expected = "I have listed the files in the current directory (which is currently empty). \n\nDone.";
    assert_eq!(calls.last().unwrap(), &("final".to_string(), expected.to_string()));
    assert_eq!(sink.outcome.final_text, expected);
    assert_eq!(sink.outcome.session_id.as_deref(), Some("e9beba6d-92d2-43bc-bc0d-600f70c939fc"));
    assert!(!sink.outcome.failed);
  }

  #[test]
  fn channel_final_mentions_sender() {
    let (pipeline, _) = run("channel", FIXTURE);
    let calls = pipeline.calls.lock().unwrap();
    assert!(calls.last().unwrap().1.starts_with("@Mira I have listed"));
  }

  #[test]
  fn failure_posts_error_message() {
    let pipeline = Arc::new(RecordingPipeline::default());
    let mut sink = ChatSink::new(context("dm"), "span-1".into(), pipeline.clone(), None);
    sink.on_event(AgentEvent::TurnFailed { message: "boom".into() });
    assert_eq!(pipeline.calls.lock().unwrap().last().unwrap(), &("final".to_string(), "Agent error: boom".to_string()));
    assert!(sink.outcome.failed);
  }
}
