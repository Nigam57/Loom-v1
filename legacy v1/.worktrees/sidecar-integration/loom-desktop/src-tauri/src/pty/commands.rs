use tauri::{State, ipc::Channel};
use std::sync::{Arc, Mutex};
use crate::pty::manager::PtyManager;
use std::io::Read;
use serde::Serialize;
use std::thread;
use std::collections::HashMap;

pub struct PtyState {
    pub manager: Arc<Mutex<PtyManager>>,
    pub broadcaster: Arc<Mutex<HashMap<u32, tokio::sync::broadcast::Sender<PtyEvent>>>>,
}

#[derive(Clone, Serialize)]
#[serde(tag = "type")]
pub enum PtyEvent {
    #[serde(rename = "data")]
    Data { pid: u32, data: String },
    #[serde(rename = "exit")]
    Exit { pid: u32, code: Option<u32> },
    #[serde(rename = "error")]
    Error { pid: u32, message: String },
}

#[tauri::command]
pub fn spawn_pty(
    state: State<'_, PtyState>,
    cmd: String,
    args: Option<Vec<String>>,
    env: Option<HashMap<String, String>>,
    cwd: Option<String>,
    on_event: Channel<PtyEvent>,
) -> Result<u32, String> {
    let manager = state.manager.lock().unwrap();
    let args_vec = args.unwrap_or_default();
    let mut env_map = env.unwrap_or_default();
    
    let resolved_cwd = cwd.clone().unwrap_or_else(|| {
        std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| ".".to_string())
    });

    if let Some(mcp_config) = crate::mcp::detect_mcp_config(&resolved_cwd) {
        env_map.insert("MCP_SERVERS_CONFIG".to_string(), mcp_config);
    }

    let (pid, mut reader) = manager.spawn_pty(
        &cmd,
        &args_vec,
        &env_map,
        Some(&resolved_cwd),
    )?;

    let tx = {
        let mut map = state.broadcaster.lock().unwrap();
        if let Some(tx) = map.get(&pid) {
            tx.clone()
        } else {
            let (tx, _) = tokio::sync::broadcast::channel(100);
            map.insert(pid, tx.clone());
            tx
        }
    };

    // Spawn background thread to read PTY output and send via channel
    let channel = on_event;
    thread::spawn(move || {
        let mut buffer = [0u8; 4096];
        loop {
            match reader.read(&mut buffer) {
                Ok(0) => {
                    let ev = PtyEvent::Exit { pid, code: Some(0) };
                    let _ = channel.send(ev.clone());
                    let _ = tx.send(ev);
                    break;
                }
                Ok(n) => {
                    let data = String::from_utf8_lossy(&buffer[..n]).to_string();
                    let ev = PtyEvent::Data { pid, data };
                    let c_res = channel.send(ev.clone());
                    let _ = tx.send(ev);
                    if c_res.is_err() && tx.receiver_count() == 0 {
                        // Keep reading even if channel fails, unless there are no subscribers at all
                        // Actually, just don't break on channel error so sidecar can still read
                    }
                }
                Err(e) => {
                    let ev = PtyEvent::Error {
                        pid,
                        message: e.to_string(),
                    };
                    let _ = channel.send(ev.clone());
                    let _ = tx.send(ev);
                    break;
                }
            }
        }
    });

    Ok(pid)
}

#[tauri::command]
pub fn write_pty(state: State<'_, PtyState>, pid: u32, data: String) -> Result<(), String> {
    let manager = state.manager.lock().unwrap();
    manager.write_pty(pid, &data)
}

#[tauri::command]
pub fn resize_pty(state: State<'_, PtyState>, pid: u32, rows: u16, cols: u16) -> Result<(), String> {
    let manager = state.manager.lock().unwrap();
    manager.resize_pty(pid, rows, cols)
}

#[tauri::command]
pub fn kill_pty(state: State<'_, PtyState>, pid: u32) -> Result<(), String> {
    let manager = state.manager.lock().unwrap();
    manager.kill_process_tree(pid)
}

#[tauri::command]
pub fn kill_all(state: State<'_, PtyState>) -> Result<(), String> {
    let manager = state.manager.lock().unwrap();
    manager.kill_all_processes()
}

#[tauri::command]
pub fn list_pty(state: State<'_, PtyState>) -> Vec<u32> {
    let manager = state.manager.lock().unwrap();
    manager.list_pids()
}
