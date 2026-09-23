use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod fake_cli;
pub mod pty_adapter;
pub mod headless_json;
pub mod factory;

/// Events emitted by an agent adapter
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AgentEvent {
    /// Agent produced text output
    #[serde(rename = "output")]
    Output { text: String },
    /// Agent produced a structured result
    #[serde(rename = "result")]
    Result { content: String, cost_cents: u32 },
    /// Agent requests approval for an action
    #[serde(rename = "approval_request")]
    ApprovalRequest { id: String, tool: String, args: String },
    /// Agent finished
    #[serde(rename = "done")]
    Done { cost_cents: u32 },
    /// Agent errored
    #[serde(rename = "error")]
    Error { message: String },
}

/// Message sent to an agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    pub content: String,
    pub sender: String,
}

/// How the adapter manages conversation history
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContextStrategy {
    Stateless, // Router must send the full transcript every turn
    Stateful,  // Router only sends the newest message; session remains alive
}

impl Default for ContextStrategy {
    fn default() -> Self {
        ContextStrategy::Stateless
    }
}

/// Capabilities of an adapter
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AdapterCapabilities {
    pub supports_headless_json: bool,
    pub supports_acp: bool,
    pub supports_streaming: bool,
    pub supports_approval: bool,
    pub supports_usage_extraction: bool,
    pub context_strategy: ContextStrategy,
}

/// Spawn configuration for an agent
#[derive(Debug, Clone)]
pub struct SpawnConfig {
    pub cmd: String,
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
    pub cwd: String,
}

/// Usage statistics from an agent run
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UsageStats {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cost_cents: u32,
}

/// Transport-agnostic agent adapter trait (async)
#[async_trait::async_trait]
pub trait AgentAdapter: Send + Sync {
    /// Spawn the agent process
    async fn spawn(&self, config: SpawnConfig) -> Result<u32, String>;

    /// Send a message/prompt to the agent
    async fn send_message(&self, msg: AgentMessage) -> Result<(), String>;

    /// Receive the next event from the agent (blocks until available)
    async fn recv_event(&self) -> Result<AgentEvent, String>;

    /// Cancel the running agent
    async fn cancel(&self) -> Result<(), String>;

    /// Get usage statistics
    fn usage(&self) -> Option<UsageStats>;

    /// Get adapter capabilities
    fn capabilities(&self) -> AdapterCapabilities;

    /// Adapter name for display
    fn name(&self) -> &str;
}
