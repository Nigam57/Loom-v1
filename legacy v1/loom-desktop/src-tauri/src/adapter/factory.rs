use super::fake_cli::FakeCliAdapter;
use super::headless_json::HeadlessJsonAdapter;
use super::pty_adapter::PtyAdapter;
use super::{AdapterCapabilities, AgentAdapter};
use crate::pty::manager::PtyManager;
use std::sync::{Arc, Mutex};

pub struct AdapterFactory;

impl AdapterFactory {
    pub fn create_adapter(
        agent_id: &str,
        pty_manager: Arc<Mutex<PtyManager>>,
    ) -> Arc<dyn AgentAdapter + Send + Sync> {
        match agent_id.to_lowercase().as_str() {
            "claude" | "claude-code" | "agy" => Arc::new(HeadlessJsonAdapter::new(agent_id)),
            id if id.starts_with("fake-") => Arc::new(FakeCliAdapter::echo(id)),
            _ => Arc::new(PtyAdapter::new(agent_id, pty_manager)),
        }
    }

    pub fn get_capabilities(agent_id: &str) -> AdapterCapabilities {
        // We create a temporary dummy adapter to query its capabilities
        // A real implementation might just return hardcoded caps based on ID
        match agent_id.to_lowercase().as_str() {
            "claude" | "claude-code" | "agy" => HeadlessJsonAdapter::new(agent_id).capabilities(),
            id if id.starts_with("fake-") => FakeCliAdapter::echo(id).capabilities(),
            _ => {
                // Return default PTY capabilities without initializing the PtyManager
                AdapterCapabilities {
                    supports_headless_json: false,
                    supports_acp: false,
                    supports_streaming: true,
                    supports_approval: false,
                    supports_usage_extraction: false,
                    context_strategy: super::ContextStrategy::Stateless,
                }
            }
        }
    }
}
