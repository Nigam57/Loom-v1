use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;
use tauri::State;
use std::sync::Arc;

use crate::adapter::{factory::AdapterFactory, AdapterCapabilities, SpawnConfig};
use crate::pty::commands::PtyState;
use crate::session::SessionManager;
use crate::room_manager::RoomManager;

pub struct AppState {
    pub session_manager: Arc<SessionManager>,
    pub room_manager: Arc<RoomManager>,
}

/// Settings payload — mirrors the frontend AppSettings type.
/// For now, we return defaults and accept updates without persisting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettingsPayload {
    pub appearance: Option<serde_json::Value>,
    pub agents: Option<serde_json::Value>,
    pub rooms: Option<serde_json::Value>,
    pub extensions: Option<serde_json::Value>,
}

#[tauri::command]
pub fn get_settings() -> Result<serde_json::Value, String> {
    // Return empty object — frontend merges with defaults
    Ok(serde_json::json!({}))
}

#[tauri::command]
pub fn update_settings(settings: serde_json::Value) -> Result<(), String> {
    // TODO: Persist settings to disk (e.g. JSON file in app data dir)
    log::info!("Settings update received: {} keys", settings.as_object().map(|o| o.len()).unwrap_or(0));
    Ok(())
}

#[derive(Serialize)]
pub struct AgentStatus {
    pub id: String,
    pub name: String,
    pub installed: bool,
    pub path: Option<String>,
    pub requires_wsl: bool,
    pub reason: Option<String>,
}

/// Detect installed agent CLIs by searching PATH (using `where.exe` on Windows).
#[tauri::command]
pub fn detect_agents() -> Vec<AgentStatus> {
    let mut agents = Vec::new();

    let clis = vec![
        ("claude", "Claude Code"),
        ("coder", "Codex"),
        ("opencode", "OpenCode"),
        ("agy", "agy"),
        ("hermes", "Hermes Agent"),
        ("odysseus", "Odysseus"),
    ];

    for (cmd, name) in clis {
        let (installed, path, reason) = resolve_cli(cmd);
        agents.push(AgentStatus {
            id: cmd.to_string(),
            name: name.to_string(),
            installed,
            path,
            requires_wsl: false,
            reason,
        });
    }

    agents
}

#[tauri::command]
pub fn get_adapter_capabilities(agent_id: String) -> AdapterCapabilities {
    AdapterFactory::get_capabilities(&agent_id)
}

fn resolve_cli(cmd: &str) -> (bool, Option<String>, Option<String>) {
    #[cfg(windows)]
    {
        // Use where.exe to resolve .cmd shims and exact path on Windows
        match Command::new("where.exe").arg(cmd).output() {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    
                    let mut fallback_path = None;
                    
                    for line in stdout.lines() {
                        let path = line.trim().to_string();
                        if path.to_lowercase().ends_with(".exe") || path.to_lowercase().ends_with(".cmd") || path.to_lowercase().ends_with(".bat") {
                            return (true, Some(path), None);
                        }
                        if fallback_path.is_none() && !path.is_empty() {
                            fallback_path = Some(path);
                        }
                    }
                    
                    if let Some(path) = fallback_path {
                        return (true, Some(path), None);
                    }
                }
                (false, None, Some(format!("{} not found in PATH", cmd)))
            }
            Err(e) => (false, None, Some(format!("Failed to execute where.exe: {}", e))),
        }
    }
    #[cfg(not(windows))]
    {
        match Command::new("which").arg(cmd).output() {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let path = stdout.trim().to_string();
                    if !path.is_empty() {
                        return (true, Some(path), None);
                    }
                }
                (false, None, Some(format!("{} not found in PATH", cmd)))
            }
            Err(e) => (false, None, Some(format!("Failed to execute which: {}", e))),
        }
    }
}

/// Spawn an agent through the appropriate adapter
#[tauri::command]
pub async fn spawn_agent(
    agent_id: String,
    cwd: String,
    app_state: State<'_, AppState>,
    pty_state: State<'_, PtyState>,
) -> Result<(u32, String), String> {
    // 1. Validate cwd (Amendment 12)
    let safe_cwd = SessionManager::validate_cwd(&cwd)?;

    // 2. Select Adapter
    // If FakeCLI is requested (e.g. for handoff test), use FakeCliAdapter
    let adapter = AdapterFactory::create_adapter(&agent_id, pty_state.manager.clone());
    
    // Ensure the real CLI is actually installed if it's not fake
    if !agent_id.starts_with("fake-") {
        let (installed, _path, reason) = resolve_cli(&agent_id);
        if !installed {
            return Err(format!("CLI {} is not installed: {}", agent_id, reason.unwrap_or_default()));
        }
    }

    // 3. Register in session manager
    let session_id = format!("{}-{}", agent_id, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
    app_state.session_manager.register(session_id.clone(), adapter.clone());

    // 4. Spawn it
    let resolved_cmd = if agent_id.starts_with("fake-") {
        "fake".to_string()
    } else {
        resolve_cli(&agent_id).1.unwrap_or(agent_id) // Use absolute path if possible
    };
    
    // Check if it's a .cmd shim on Windows
    let (final_cmd, args) = if resolved_cmd.to_lowercase().ends_with(".cmd") || resolved_cmd.to_lowercase().ends_with(".bat") {
        ("cmd.exe".to_string(), vec!["/c".to_string(), resolved_cmd])
    } else {
        (resolved_cmd, vec![])
    };

    let config = SpawnConfig {
        cmd: final_cmd,
        args,
        env: HashMap::new(),
        cwd: safe_cwd,
    };

    let pid = adapter.spawn(config).await?;
    Ok((pid, session_id))
}

#[tauri::command]
pub async fn kill_agent(
    session_id: String,
    app_state: State<'_, AppState>,
) -> Result<(), String> {
    if let Some(adapter) = app_state.session_manager.get(&session_id) {
        adapter.cancel().await?;
        app_state.session_manager.remove(&session_id);
        Ok(())
    } else {
        Err("Session not found".to_string())
    }
}

use crate::router::{Router, RouterConfig, RouterResult};

#[tauri::command]
pub async fn run_handoff(
    session_a: String,
    session_b: String,
    prompt: String,
    config: RouterConfig,
    app_state: State<'_, AppState>,
) -> Result<RouterResult, String> {
    let agent_a = app_state.session_manager.get(&session_a).ok_or("Session A not found")?;
    let agent_b = app_state.session_manager.get(&session_b).ok_or("Session B not found")?;

    // As per amendment 4, handoff is FakeCLI only right now
    if !agent_a.name().starts_with("fake-") || !agent_b.name().starts_with("fake-") {
        return Err("Handoff is currently only supported between FakeCLI agents (until HeadlessJson adapter M2)".to_string());
    }

    let mut router = Router::new(config);
    
    // For fake CLI, we don't need real spawn config paths
    let spawn_config = SpawnConfig {
        cmd: "fake".to_string(),
        args: vec![],
        env: HashMap::new(),
        cwd: ".".to_string(),
    };

    let result = router.run_handoff(
        agent_a.as_ref(),
        agent_b.as_ref(),
        &prompt,
        spawn_config.clone(),
        spawn_config,
    ).await;

    Ok(result)
}

use crate::router::conversation::{Message, ConversationGuards, Participant};

#[tauri::command]
pub async fn get_room_transcript(
    room_id: String,
    app_state: State<'_, AppState>,
) -> Result<Vec<Message>, String> {
    app_state.room_manager.get_transcript(&room_id)
}

#[tauri::command]
pub async fn create_room(
    goal: String,
    agent_ids: Vec<String>,
    cwd: Option<String>,
    max_turns: u32,
    timeout_seconds: u64,
    budget_cents: u32,
    app_state: State<'_, AppState>,
    pty_state: State<'_, PtyState>,
) -> Result<String, String> {
    let guards = ConversationGuards {
        max_turns,
        timeout_seconds,
        budget_cents,
    };
    let room_id = app_state.room_manager.create_room(goal, guards);

    let resolved_cwd = cwd.unwrap_or_else(|| {
        std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| ".".to_string())
    });

    for agent_id in agent_ids {
        let adapter = AdapterFactory::create_adapter(&agent_id, pty_state.manager.clone());
        
        let resolved_cmd = if agent_id.starts_with("fake-") {
            "fake".to_string()
        } else {
            resolve_cli(&agent_id).1.unwrap_or(agent_id.clone())
        };
        
        let (final_cmd, args) = if resolved_cmd.to_lowercase().ends_with(".cmd") || resolved_cmd.to_lowercase().ends_with(".bat") {
            ("cmd.exe".to_string(), vec!["/c".to_string(), resolved_cmd])
        } else {
            (resolved_cmd, vec![])
        };

        let mut env_map = HashMap::new();
        if let Some(mcp_config) = crate::mcp::detect_mcp_config(&resolved_cwd) {
            env_map.insert("MCP_SERVERS_CONFIG".to_string(), mcp_config);
        }

        let spawn_config = SpawnConfig {
            cmd: final_cmd,
            args,
            env: env_map,
            cwd: resolved_cwd.clone(),
        };
        let _ = app_state.room_manager.add_participant(&room_id, Participant {
            agent: adapter,
            role_prompt: "".to_string(),
            is_writer: false,
            spawn_config,
        });
    }

    Ok(room_id)
}

#[tauri::command]
pub async fn start_room(
    room_id: String,
    app_state: State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    if let Some(room_arc) = app_state.room_manager.get_room(&room_id) {
        {
            let mut room = room_arc.lock().unwrap();
            room.status = crate::router::conversation::RoomStatus::Running;
        }

        let room_arc_clone = room_arc.clone();
        let guards = {
            let room = room_arc.lock().unwrap();
            room.guards.clone()
        };
        tokio::spawn(async move {
            let config = RouterConfig {
                max_turns: guards.max_turns,
                timeout_seconds: guards.timeout_seconds,
                budget_cents: guards.budget_cents,
            };
            let mut router = Router::new(config);
            let result = router.run_conversation(room_arc_clone, Some(app_handle)).await;
            
            // Mark as stopped or paused based on result
            let mut room = room_arc.lock().unwrap();
            match result.stop_reason {
                crate::router::StopReason::NeedsApproval { .. } => {
                    room.status = crate::router::conversation::RoomStatus::Paused;
                }
                _ => {
                    room.status = crate::router::conversation::RoomStatus::Stopped;
                }
            }
        });
        Ok(())
    } else {
        Err(format!("Room {} not found", room_id))
    }
}

#[tauri::command]
pub async fn pause_room(
    room_id: String,
    app_state: State<'_, AppState>,
) -> Result<(), String> {
    if let Some(room_arc) = app_state.room_manager.get_room(&room_id) {
        let mut room = room_arc.lock().unwrap();
        room.status = crate::router::conversation::RoomStatus::Paused;
        Ok(())
    } else {
        Err(format!("Room {} not found", room_id))
    }
}

#[tauri::command]
pub async fn stop_room(
    room_id: String,
    app_state: State<'_, AppState>,
) -> Result<(), String> {
    if let Some(room_arc) = app_state.room_manager.get_room(&room_id) {
        let mut room = room_arc.lock().unwrap();
        room.status = crate::router::conversation::RoomStatus::Stopped;
        Ok(())
    } else {
        Err(format!("Room {} not found", room_id))
    }
}

#[tauri::command]
pub async fn send_manual_reply(
    room_id: String,
    text: String,
    app_state: State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    if let Some(room_arc) = app_state.room_manager.get_room(&room_id) {
        let msg = {
            let mut room = room_arc.lock().unwrap();
            room.append_message("User".to_string(), text, 0)
        };
        use tauri::Emitter;
        let _ = app_handle.emit("room://transcript-update", serde_json::json!({
            "room_id": room_id,
            "message": msg
        }));
        Ok(())
    } else {
        Err(format!("Room {} not found", room_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapter::{fake_cli::FakeCliAdapter, AgentAdapter, SpawnConfig};
    use crate::pty::manager::PtyManager;
    use std::sync::Mutex as StdMutex;

    #[tokio::test]
    async fn test_spawn_and_kill_fakecli() {
        let pty_manager = Arc::new(StdMutex::new(PtyManager::new()));
        let session_manager = Arc::new(SessionManager::new(pty_manager.clone()));
        
        let agent_id = "fake-test".to_string();
        let safe_cwd = ".".to_string();
        
        let adapter: Arc<dyn AgentAdapter + Send + Sync> = Arc::new(FakeCliAdapter::echo(&agent_id));
        
        let session_id = format!("{}-123", agent_id);
        session_manager.register(session_id.clone(), adapter.clone());
        
        let config = SpawnConfig {
            cmd: "fake".to_string(),
            args: vec![],
            env: HashMap::new(),
            cwd: safe_cwd,
        };
        
        let pid = adapter.spawn(config).await.unwrap();
        assert_eq!(pid, 0); // Fake CLI returns PID 0
        
        assert!(session_manager.get(&session_id).is_some());
        
        // Now kill
        let killed = if let Some(a) = session_manager.get(&session_id) {
            a.cancel().await.unwrap();
            session_manager.remove(&session_id);
            true
        } else {
            false
        };
        
        assert!(killed);
        assert!(session_manager.get(&session_id).is_none());
    }

    #[tokio::test]
    async fn test_run_handoff_ipc() {
        let pty_manager = Arc::new(StdMutex::new(PtyManager::new()));
        let session_manager = Arc::new(SessionManager::new(pty_manager.clone()));
        
        let agent_a_id = "fake-a".to_string();
        let agent_b_id = "fake-b".to_string();
        
        // Use FakeCliAdapter::echo which just echoes messages back
        let adapter_a: Arc<dyn AgentAdapter + Send + Sync> = Arc::new(FakeCliAdapter::echo(&agent_a_id));
        let adapter_b: Arc<dyn AgentAdapter + Send + Sync> = Arc::new(FakeCliAdapter::echo(&agent_b_id));
        
        let session_a = format!("{}-123", agent_a_id);
        let session_b = format!("{}-456", agent_b_id);
        
        session_manager.register(session_a.clone(), adapter_a);
        session_manager.register(session_b.clone(), adapter_b);
        
        // Use a strict guard: max 3 turns
        let config = RouterConfig {
            max_turns: 3,
            timeout_seconds: 300,
            budget_cents: 100,
        };
        
        let mut router = crate::router::Router::new(config);
        
        let spawn_config = SpawnConfig {
            cmd: "fake".to_string(),
            args: vec![],
            env: HashMap::new(),
            cwd: ".".to_string(),
        };

        let result = router.run_handoff(
            session_manager.get(&session_a).unwrap().as_ref(),
            session_manager.get(&session_b).unwrap().as_ref(),
            "ping",
            spawn_config.clone(),
            spawn_config,
        ).await;
        
        match result.stop_reason {
            crate::router::StopReason::MaxTurns { limit } => {
                assert_eq!(limit, 3);
            },
            _ => panic!("Expected max turns guard to fire, got {:?}", result.stop_reason),
        }
        
        assert_eq!(result.turns_used, 3);
    }
}
