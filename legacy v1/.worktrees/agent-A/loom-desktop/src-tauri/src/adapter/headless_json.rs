use super::{AdapterCapabilities, AgentAdapter, AgentEvent, AgentMessage, ContextStrategy, SpawnConfig, UsageStats};
use serde_json::Value;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, Command};
use tokio::sync::{mpsc, Mutex};
use std::process::Stdio;

#[cfg(windows)]
use windows_sys::Win32::System::JobObjects::{
    AssignProcessToJobObject, SetInformationJobObject, JobObjectExtendedLimitInformation,
    JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
};
#[cfg(windows)]
use windows_sys::Win32::System::Threading::{OpenProcess, PROCESS_ALL_ACCESS};
#[cfg(windows)]
use windows_sys::Win32::Foundation::CloseHandle;

#[cfg(windows)]
extern "system" {
    pub fn CreateJobObjectW(
        lpjobattributes: *const std::ffi::c_void,
        lpname: *const u16,
    ) -> *mut std::ffi::c_void;
}

#[cfg(windows)]
struct JobHandle(*mut std::ffi::c_void);
#[cfg(windows)]
unsafe impl Send for JobHandle {}
#[cfg(windows)]
unsafe impl Sync for JobHandle {}

pub struct HeadlessJsonAdapter {
    name: String,
    child: Mutex<Option<Child>>,
    stdin: Mutex<Option<ChildStdin>>,
    event_rx: Mutex<Option<mpsc::Receiver<AgentEvent>>>,
    #[cfg(windows)]
    job_handle: Mutex<Option<JobHandle>>,
}

// SAFETY: Windows HANDLE used for cleanup.
#[cfg(windows)]
unsafe impl Send for HeadlessJsonAdapter {}
#[cfg(windows)]
unsafe impl Sync for HeadlessJsonAdapter {}

impl HeadlessJsonAdapter {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            child: Mutex::new(None),
            stdin: Mutex::new(None),
            event_rx: Mutex::new(None),
            #[cfg(windows)]
            job_handle: Mutex::new(None),
        }
    }
}

impl Drop for HeadlessJsonAdapter {
    fn drop(&mut self) {
        #[cfg(windows)]
        unsafe {
            if let Some(handle) = self.job_handle.blocking_lock().take() {
                if !handle.0.is_null() {
                    CloseHandle(handle.0 as _);
                }
            }
        }
    }
}

#[async_trait::async_trait]
impl AgentAdapter for HeadlessJsonAdapter {
    async fn spawn(&self, mut config: SpawnConfig) -> Result<u32, String> {
        // Ensure --output-format stream-json is added if not present (heuristic for claude/agy)
        if !config.args.iter().any(|a| a == "--output-format") {
            if self.name.starts_with("claude") || self.name.starts_with("agy") {
                config.args.push("--output-format".to_string());
                config.args.push("stream-json".to_string());
            }
        }

        let mut cmd = Command::new(&config.cmd);
        cmd.args(&config.args);
        cmd.envs(&config.env);
        cmd.current_dir(&config.cwd);
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped()); // ignore or log stderr

        let mut child = cmd.spawn().map_err(|e: std::io::Error| e.to_string())?;
        let pid = child.id().unwrap_or(0);

        #[cfg(windows)]
        {
            let job = unsafe { CreateJobObjectW(std::ptr::null(), std::ptr::null()) };
            if !job.is_null() {
                let mut info: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = unsafe { std::mem::zeroed() };
                info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;

                unsafe {
                    SetInformationJobObject(
                        job,
                        JobObjectExtendedLimitInformation,
                        &info as *const _ as *const _,
                        std::mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
                    );
                }

                let process_handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, 0, pid) };
                if process_handle != 0 as _ {
                    unsafe {
                        AssignProcessToJobObject(job, process_handle);
                        CloseHandle(process_handle);
                    }
                }
            }
            *self.job_handle.lock().await = Some(JobHandle(job));
        }

        let stdin = child.stdin.take().ok_or("Failed to open stdin")?;
        let stdout = child.stdout.take().ok_or("Failed to open stdout")?;

        *self.stdin.lock().await = Some(stdin);
        *self.child.lock().await = Some(child);

        let (tx, rx) = mpsc::channel(100);
        *self.event_rx.lock().await = Some(rx);

        // Reader task
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                let line_str: String = line;
                // Try to parse NDJSON line
                if let Ok(value) = serde_json::from_str::<Value>(&line_str) {
                    // Extract message structure depending on format
                    if let Some(msg_type) = value.get("type").and_then(|v| v.as_str()) {
                        match msg_type {
                            "message_start" | "message_delta" | "content_block_delta" => {
                                // Accumulate or emit output
                                // Claude Code stream-json emits `content_block_delta` with text
                                if let Some(delta) = value.get("delta") {
                                    if let Some(text) = delta.get("text").and_then(|t| t.as_str()) {
                                        let _ = tx.send(AgentEvent::Output { text: text.to_string() }).await;
                                    }
                                }
                            }
                            "message_stop" | "message_done" => {
                                // Extract cost if available, or just send Done
                                let cost_cents = 0; // Parse this if available
                                let _ = tx.send(AgentEvent::Done { cost_cents }).await;
                            }
                            "result" => {
                                // Some tools might emit a flat "result"
                                if let Some(content) = value.get("content").and_then(|v| v.as_str()) {
                                    let _ = tx.send(AgentEvent::Result { content: content.to_string(), cost_cents: 0 }).await;
                                }
                            }
                            "error" => {
                                let msg = value.get("message").and_then(|v| v.as_str()).unwrap_or("Unknown error");
                                let _ = tx.send(AgentEvent::Error { message: msg.to_string() }).await;
                            }
                            _ => {
                                // Unknown json event, ignore or log
                            }
                        }
                    } else if let Some(content) = value.get("text").and_then(|v| v.as_str()) {
                         // Simple NDJSON structure
                         let _ = tx.send(AgentEvent::Output { text: content.to_string() }).await;
                    }
                } else {
                    // Not JSON, just emit as text if it has something useful? Or ignore.
                    // Let's emit as output for safety
                    let _ = tx.send(AgentEvent::Output { text: format!("{}\n", line_str) }).await;
                }
            }
            // Stream ended
            let _ = tx.send(AgentEvent::Done { cost_cents: 0 }).await;
        });

        Ok(pid)
    }

    async fn send_message(&self, msg: AgentMessage) -> Result<(), String> {
        let mut stdin_lock = self.stdin.lock().await;
        if let Some(stdin) = stdin_lock.as_mut().map(|s| s as &mut tokio::process::ChildStdin) {
            // For stateful JSON adapters, we might send JSON or just text.
            // Claude Code expects JSON on stdin when using stream-json.
            let payload = serde_json::json!({
                "type": "user",
                "text": msg.content
            });
            let line = format!("{}\n", serde_json::to_string(&payload).unwrap());
            stdin.write_all(line.as_bytes()).await.map_err(|e: std::io::Error| e.to_string())?;
            stdin.flush().await.map_err(|e: std::io::Error| e.to_string())?;
            Ok(())
        } else {
            Err("Stdin not available".to_string())
        }
    }

    async fn recv_event(&self) -> Result<AgentEvent, String> {
        let mut rx_lock = self.event_rx.lock().await;
        if let Some(rx) = rx_lock.as_mut() {
            match rx.recv().await {
                Some(event) => Ok(event),
                None => Err("Channel closed".to_string()),
            }
        } else {
            Err("Receiver not available".to_string())
        }
    }

    async fn cancel(&self) -> Result<(), String> {
        let mut child_lock = self.child.lock().await;
        if let Some(child) = child_lock.as_mut().map(|c| c as &mut tokio::process::Child) {
            let _ = child.kill().await;
            #[cfg(windows)]
            {
                let mut job = self.job_handle.lock().await;
                if let Some(handle) = job.take() {
                    if !handle.0.is_null() {
                        unsafe { CloseHandle(handle.0 as _) };
                    }
                }
            }
        }
        Ok(())
    }

    fn usage(&self) -> Option<UsageStats> {
        None
    }

    fn capabilities(&self) -> AdapterCapabilities {
        AdapterCapabilities {
            supports_headless_json: true,
            supports_acp: false,
            supports_streaming: true,
            supports_approval: true,
            supports_usage_extraction: false,
            context_strategy: ContextStrategy::Stateful, // Keep process alive
        }
    }

    fn name(&self) -> &str {
        &self.name
    }
}
