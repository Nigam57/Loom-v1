//! Headless agent runtime: structured CLI adapters instead of PTY screen scraping.

pub(crate) mod adapters;
pub(crate) mod chat_sink;
pub(crate) mod events;
pub(crate) mod queue;
pub(crate) mod registration;
pub(crate) mod runner;
pub(crate) mod service;
pub(crate) mod sessions;

pub(crate) use chat_sink::ChatTurnContext;
pub(crate) use service::{AgentRuntime, McpBridge, TurnJob};
