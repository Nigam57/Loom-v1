use crate::adapter::{
    AgentAdapter, AgentEvent, AgentMessage, AdapterCapabilities, SpawnConfig, UsageStats,
};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

/// A fake CLI adapter for testing that emits scripted events
pub struct FakeCliAdapter {
    name: String,
    events: Arc<Mutex<Vec<AgentEvent>>>,
    rx: Mutex<Option<mpsc::Receiver<AgentEvent>>>,
    tx: mpsc::Sender<AgentEvent>,
    usage: Mutex<UsageStats>,
}

impl FakeCliAdapter {
    pub fn new(name: &str, scripted_events: Vec<AgentEvent>) -> Self {
        let (tx, rx) = mpsc::channel(100);
        Self {
            name: name.to_string(),
            events: Arc::new(Mutex::new(scripted_events)),
            rx: Mutex::new(Some(rx)),
            tx,
            usage: Mutex::new(UsageStats::default()),
        }
    }

    /// Create a simple echo adapter that responds with the input
    pub fn echo(name: &str) -> Self {
        Self::new(name, vec![])
    }

    /// Create an adapter that emits a sequence then a Done event
    pub fn scripted(name: &str, outputs: Vec<String>, cost: u32) -> Self {
        let mut events: Vec<AgentEvent> = outputs
            .into_iter()
            .map(|text| AgentEvent::Output { text })
            .collect();
        events.push(AgentEvent::Done { cost_cents: cost });
        Self::new(name, events)
    }
}

#[async_trait::async_trait]
impl AgentAdapter for FakeCliAdapter {
    async fn spawn(&self, _config: SpawnConfig) -> Result<u32, String> {
        // Emit all scripted events
        let events = self.events.lock().unwrap().drain(..).collect::<Vec<_>>();
        let tx = self.tx.clone();
        tokio::spawn(async move {
            for event in events {
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                if tx.send(event).await.is_err() {
                    break;
                }
            }
        });
        Ok(0) // fake PID
    }

    async fn send_message(&self, msg: AgentMessage) -> Result<(), String> {
        // Echo adapter: respond with the message content
        let response = AgentEvent::Result {
            content: format!("[{}] echo: {}", self.name, msg.content),
            cost_cents: 1,
        };
        self.tx.send(response).await.map_err(|e| e.to_string())?;
        self.tx
            .send(AgentEvent::Done { cost_cents: 1 })
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn recv_event(&self) -> Result<AgentEvent, String> {
        let mut rx_guard = self.rx.lock().unwrap();
        if let Some(_rx) = rx_guard.as_mut() {
            // We need to drop the mutex before awaiting
            // This is a simplified version - in production use tokio::sync::Mutex
            drop(rx_guard);

            // Re-acquire - this is safe because we're single-threaded per adapter
            let mut rx_guard = self.rx.lock().unwrap();
            if let Some(rx) = rx_guard.as_mut() {
                match rx.try_recv() {
                    Ok(event) => {
                        if let AgentEvent::Done { cost_cents } | AgentEvent::Result { cost_cents, .. } = &event {
                            let mut usage = self.usage.lock().unwrap();
                            usage.cost_cents += cost_cents;
                        }
                        Ok(event)
                    }
                    Err(mpsc::error::TryRecvError::Empty) => {
                        Err("no events available".to_string())
                    }
                    Err(mpsc::error::TryRecvError::Disconnected) => {
                        Ok(AgentEvent::Done { cost_cents: 0 })
                    }
                }
            } else {
                Err("adapter not started".to_string())
            }
        } else {
            Err("adapter not started".to_string())
        }
    }

    async fn cancel(&self) -> Result<(), String> {
        Ok(()) // Nothing to cancel for fake CLI
    }

    fn usage(&self) -> Option<UsageStats> {
        Some(self.usage.lock().unwrap().clone())
    }

    fn capabilities(&self) -> AdapterCapabilities {
        AdapterCapabilities {
            supports_headless_json: true,
            supports_acp: false,
            supports_streaming: true,
            supports_approval: false,
            supports_usage_extraction: true,
            context_strategy: crate::adapter::ContextStrategy::Stateless,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fake_cli_scripted() {
        let adapter = FakeCliAdapter::scripted(
            "test-agent",
            vec!["hello".to_string(), "world".to_string()],
            10,
        );

        adapter
            .spawn(SpawnConfig {
                cmd: "fake".to_string(),
                args: vec![],
                env: std::collections::HashMap::new(),
                cwd: ".".to_string(),
            })
            .await
            .unwrap();

        // Give scripted events time to emit
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        let mut events = vec![];
        loop {
            match adapter.recv_event().await {
                Ok(event) => {
                    let is_done = matches!(&event, AgentEvent::Done { .. });
                    events.push(event);
                    if is_done {
                        break;
                    }
                }
                Err(_) => break,
            }
        }

        assert!(events.len() >= 2, "Expected at least 2 events, got {}", events.len());
        assert!(matches!(&events.last().unwrap(), AgentEvent::Done { .. }));

        let usage = adapter.usage().unwrap();
        assert!(usage.cost_cents > 0);
    }

    #[tokio::test]
    async fn test_fake_cli_echo() {
        let adapter = FakeCliAdapter::echo("echo-agent");

        adapter
            .spawn(SpawnConfig {
                cmd: "fake".to_string(),
                args: vec![],
                env: std::collections::HashMap::new(),
                cwd: ".".to_string(),
            })
            .await
            .unwrap();

        adapter
            .send_message(AgentMessage {
                content: "test message".to_string(),
                sender: "router".to_string(),
            })
            .await
            .unwrap();

        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        let event = adapter.recv_event().await.unwrap();
        match event {
            AgentEvent::Result { content, .. } => {
                assert!(content.contains("test message"));
            }
            _ => panic!("Expected Result event"),
        }
    }
}
