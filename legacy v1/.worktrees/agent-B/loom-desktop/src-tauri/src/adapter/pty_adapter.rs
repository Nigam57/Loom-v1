use crate::adapter::{
    AgentAdapter, AgentEvent, AgentMessage, AdapterCapabilities, SpawnConfig, UsageStats,
};
use crate::pty::manager::PtyManager;
use std::io::Read;
use std::sync::{Arc, Mutex as StdMutex};
use tokio::sync::{broadcast, mpsc, Mutex};

/// PtyAdapter wraps PtyManager as an AgentAdapter.
///
/// Data flow (dual-stream, amendment 3):
/// - Raw bytes from PTY → broadcast channel → xterm (UI) AND parsed events
/// - Parsed events → mpsc channel → Router
/// - User keystrokes → raw write path (write_raw)
/// - Router prompts → send_message (structured)
///
/// Neither consumer drains a shared queue; the reader thread clones bytes to both.
pub struct PtyAdapter {
    name: String,
    manager: Arc<StdMutex<PtyManager>>,
    pid: Mutex<Option<u32>>,
    /// Raw byte broadcast — UI subscribes for xterm rendering
    raw_tx: broadcast::Sender<Vec<u8>>,
    /// Parsed event channel — Router subscribes
    event_tx: mpsc::Sender<AgentEvent>,
    event_rx: Mutex<Option<mpsc::Receiver<AgentEvent>>>,
    #[allow(dead_code)] // Reserved for future cost tracking
    usage: StdMutex<UsageStats>,
}

impl PtyAdapter {
    pub fn new(name: &str, manager: Arc<StdMutex<PtyManager>>) -> Self {
        let (raw_tx, _) = broadcast::channel(256);
        let (event_tx, event_rx) = mpsc::channel(256);
        Self {
            name: name.to_string(),
            manager,
            pid: Mutex::new(None),
            raw_tx,
            event_tx,
            event_rx: Mutex::new(Some(event_rx)),
            usage: StdMutex::new(UsageStats::default()),
        }
    }

    /// Subscribe to the raw byte broadcast (for xterm UI rendering).
    /// Multiple subscribers can coexist without draining each other.
    pub fn subscribe_raw(&self) -> broadcast::Receiver<Vec<u8>> {
        self.raw_tx.subscribe()
    }

    /// Write raw bytes to the PTY stdin (user keystrokes).
    /// This is the raw write path — distinct from send_message.
    pub fn write_raw(&self, data: &str) -> Result<(), String> {
        let pid = {
            // We can't hold the tokio Mutex synchronously, so use try_lock
            // This is safe because write_raw is called from sync IPC handlers
            match self.pid.try_lock() {
                Ok(guard) => match *guard {
                    Some(pid) => pid,
                    None => return Err("adapter not spawned".to_string()),
                },
                Err(_) => return Err("pid lock contention".to_string()),
            }
        };
        let mgr = self.manager.lock().unwrap();
        mgr.write_pty(pid, data)
    }

    /// Get the PID of the spawned process, if any.
    pub async fn get_pid(&self) -> Option<u32> {
        *self.pid.lock().await
    }
}

#[async_trait::async_trait]
impl AgentAdapter for PtyAdapter {
    async fn spawn(&self, config: SpawnConfig) -> Result<u32, String> {
        let (pid, mut reader) = {
            let mgr = self.manager.lock().unwrap();
            mgr.spawn_pty(
                &config.cmd,
                &config.args,
                &config.env,
                Some(&config.cwd),
            )?
        };

        *self.pid.lock().await = Some(pid);

        *self.pid.lock().await = Some(pid);

        // Spawn reader thread: reads raw bytes and sends to an mpsc channel
        let (raw_bytes_tx, mut raw_bytes_rx) = tokio::sync::mpsc::channel::<Vec<u8>>(1024);
        std::thread::spawn(move || {
            let mut buffer = [0u8; 4096];
            loop {
                match reader.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(n) => {
                        if raw_bytes_tx.blocking_send(buffer[..n].to_vec()).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        let raw_tx = self.raw_tx.clone();
        let event_tx = self.event_tx.clone();
        
        tokio::spawn(async move {
            let mut turn_buffer = String::new();
            loop {
                match tokio::time::timeout(std::time::Duration::from_secs(5), raw_bytes_rx.recv()).await {
                    Ok(Some(bytes)) => {
                        let text = String::from_utf8_lossy(&bytes).to_string();
                        let _ = raw_tx.send(bytes);
                        let _ = event_tx.send(AgentEvent::Output { text: text.clone() }).await;
                        
                        turn_buffer.push_str(&text);
                        // Regex/Prompt check
                        let trimmed = turn_buffer.trim_end();
                        if trimmed.ends_with('>') || trimmed.ends_with('$') {
                            let _ = event_tx.send(AgentEvent::Result { content: turn_buffer.clone(), cost_cents: 0 }).await;
                            turn_buffer.clear();
                        }
                    }
                    Ok(None) => {
                        // EOF
                        if !turn_buffer.is_empty() {
                            let _ = event_tx.send(AgentEvent::Result { content: turn_buffer.clone(), cost_cents: 0 }).await;
                        }
                        let _ = event_tx.send(AgentEvent::Done { cost_cents: 0 }).await;
                        break;
                    }
                    Err(_) => {
                        // Timeout (Idle for 5s)
                        if !turn_buffer.is_empty() {
                            let _ = event_tx.send(AgentEvent::Result { content: turn_buffer.clone(), cost_cents: 0 }).await;
                            turn_buffer.clear();
                        }
                    }
                }
            }
        });

        Ok(pid)
    }

    async fn send_message(&self, msg: AgentMessage) -> Result<(), String> {
        // Router prompts go through send_message — writes to PTY stdin
        // with a newline to simulate pressing Enter
        let data = format!("{}\n", msg.content);
        self.write_raw(&data)
    }

    async fn recv_event(&self) -> Result<AgentEvent, String> {
        let mut rx_guard = self.event_rx.lock().await;
        if let Some(rx) = rx_guard.as_mut() {
            match rx.try_recv() {
                Ok(event) => Ok(event),
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
    }

    async fn cancel(&self) -> Result<(), String> {
        let pid = self.pid.lock().await;
        if let Some(pid) = *pid {
            let mgr = self.manager.lock().unwrap();
            mgr.kill_process_tree(pid)?;
        }
        Ok(())
    }

    fn usage(&self) -> Option<UsageStats> {
        // Real CLIs: usage not metered via PTY adapter
        None
    }

    fn capabilities(&self) -> AdapterCapabilities {
        AdapterCapabilities {
            supports_headless_json: false,
            supports_acp: false,
            supports_streaming: true,
            supports_approval: false,
            supports_usage_extraction: false,
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
    use std::collections::HashMap;

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_pty_adapter_spawn_and_output() {
        let manager = Arc::new(StdMutex::new(PtyManager::new()));
        let adapter = PtyAdapter::new("test-pty", manager);

        // Subscribe to raw stream BEFORE spawn
        let mut raw_rx = adapter.subscribe_raw();

        // Collect raw bytes in a background task
        let raw_handle = tokio::spawn(async move {
            let mut collected = Vec::new();
            // Collect for up to 2 seconds
            let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(2);
            loop {
                let remaining = deadline.saturating_duration_since(tokio::time::Instant::now());
                if remaining.is_zero() {
                    break;
                }
                match tokio::time::timeout(remaining, raw_rx.recv()).await {
                    Ok(Ok(bytes)) => collected.extend_from_slice(&bytes),
                    _ => break,
                }
            }
            collected
        });

        let pid = adapter
            .spawn(SpawnConfig {
                cmd: "cmd.exe".to_string(),
                args: vec!["/c".to_string(), "echo hello_loom & ping 127.0.0.1 -n 2 > nul".to_string()],
                env: HashMap::new(),
                cwd: ".".to_string(),
            })
            .await
            .unwrap();

        assert!(pid > 0);

        // Wait for process to complete
        tokio::time::sleep(std::time::Duration::from_millis(2500)).await;

        let raw_bytes = raw_handle.await.unwrap();
        let raw_text = String::from_utf8_lossy(&raw_bytes);

        // Check parsed event stream received data
        let mut event_text = String::new();
        loop {
            match adapter.recv_event().await {
                Ok(AgentEvent::Output { text }) => event_text.push_str(&text),
                Ok(AgentEvent::Done { .. }) => break,
                Err(_) => break,
                _ => {}
            }
        }

        // Both streams must receive output (amendment 3: dual-stream)
        assert!(
            !raw_bytes.is_empty(),
            "Raw byte stream received nothing"
        );
        assert!(
            !event_text.is_empty(),
            "Parsed event stream received nothing"
        );
        // Both should contain the echo output
        assert!(
            raw_text.contains("hello_loom"),
            "Raw stream missing 'hello_loom'. Got: {:?}", raw_text
        );
        assert!(
            event_text.contains("hello_loom"),
            "Event stream missing 'hello_loom'. Got: {:?}", event_text
        );
    }

    #[tokio::test]
    async fn test_pty_adapter_cancel() {
        let manager = Arc::new(StdMutex::new(PtyManager::new()));
        let adapter = PtyAdapter::new("cancel-test", manager);

        let pid = adapter
            .spawn(SpawnConfig {
                cmd: "cmd.exe".to_string(),
                args: vec![],
                env: HashMap::new(),
                cwd: ".".to_string(),
            })
            .await
            .unwrap();

        assert!(pid > 0);

        // Cancel should not error
        adapter.cancel().await.unwrap();
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn test_dual_subscriber_same_output() {
        let manager = Arc::new(StdMutex::new(PtyManager::new()));
        let adapter = PtyAdapter::new("dual-sub-test", manager);

        let mut raw_rx1 = adapter.subscribe_raw();
        let mut raw_rx2 = adapter.subscribe_raw();

        let pid = adapter
            .spawn(SpawnConfig {
                cmd: "cmd.exe".to_string(),
                args: vec!["/c".to_string(), "echo dual_stream_test_data".to_string()],
                env: HashMap::new(),
                cwd: ".".to_string(),
            })
            .await
            .unwrap();
        assert!(pid > 0);

        let h1 = tokio::spawn(async move {
            let mut out = String::new();
            while let Ok(bytes) = raw_rx1.recv().await {
                out.push_str(&String::from_utf8_lossy(&bytes));
                if out.contains("dual_stream_test_data") { break; }
            }
            out
        });

        let h2 = tokio::spawn(async move {
            let mut out = String::new();
            while let Ok(bytes) = raw_rx2.recv().await {
                out.push_str(&String::from_utf8_lossy(&bytes));
                if out.contains("dual_stream_test_data") { break; }
            }
            out
        });

        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        let mut event_text = String::new();
        while let Ok(event) = adapter.recv_event().await {
            if let AgentEvent::Output { text } = event {
                event_text.push_str(&text);
            } else if let AgentEvent::Done { .. } = event {
                break;
            }
        }

        let out1 = h1.await.unwrap();
        let out2 = h2.await.unwrap();

        assert!(out1.contains("dual_stream_test_data"));
        assert!(out2.contains("dual_stream_test_data"));
        assert!(event_text.contains("dual_stream_test_data"));
    }

    #[tokio::test]
    async fn test_slow_consumer_does_not_block() {
        let manager = Arc::new(StdMutex::new(PtyManager::new()));
        let adapter = PtyAdapter::new("slow-sub-test", manager);

        let mut raw_rx = adapter.subscribe_raw();

        let pid = adapter
            .spawn(SpawnConfig {
                cmd: "cmd.exe".to_string(),
                args: vec!["/c".to_string(), "for /L %i in (1,1,500) do @echo spam_spam_spam_spam_spam".to_string()],
                env: HashMap::new(),
                cwd: ".".to_string(),
            })
            .await
            .unwrap();
        assert!(pid > 0);

        tokio::time::sleep(std::time::Duration::from_millis(1500)).await;

        let mut event_count = 0;
        let mut event_text = String::new();
        while let Ok(event) = adapter.recv_event().await {
            if let AgentEvent::Output { text } = event {
                event_count += 1;
                event_text.push_str(&text);
            } else if let AgentEvent::Done { .. } = event {
                break;
            }
        }

        assert!(event_count > 0, "Event channel should receive data despite lagging raw channel");
        assert!(event_text.contains("spam_spam_spam"), "Event channel should contain output");

        let err = raw_rx.try_recv().unwrap_err();
        assert!(
            matches!(err, tokio::sync::broadcast::error::TryRecvError::Lagged(_)),
            "Expected Lagged error for slow consumer, got {:?}", err
        );
    }
}
