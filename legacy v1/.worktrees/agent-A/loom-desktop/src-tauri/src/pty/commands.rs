use tauri::{State, ipc::Channel};
use std::sync::{Arc, Mutex};
use crate::pty::manager::PtyManager;
use std::io::Read;
use serde::Serialize;
use std::thread;
use std::collections::HashMap;

pub struct PtyState {
    pub manager: Arc<Mutex<PtyManager>>,
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

    // Spawn background thread to read PTY output and send via channel
    let channel = on_event;
    thread::spawn(move || {
        let mut buffer = [0u8; 4096];
        loop {
            match reader.read(&mut buffer) {
                Ok(0) => {
                    let _ = channel.send(PtyEvent::Exit { pid, code: Some(0) });
                    break;
                }
                Ok(n) => {
                    let data = String::from_utf8_lossy(&buffer[..n]).to_string();
                    if channel.send(PtyEvent::Data { pid, data }).is_err() {
                        break;
                    }
                }
                Err(e) => {
                    let _ = channel.send(PtyEvent::Error {
                        pid,
                        message: e.to_string(),
                    });
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
