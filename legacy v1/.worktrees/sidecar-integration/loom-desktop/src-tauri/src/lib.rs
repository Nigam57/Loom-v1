pub mod pty;
pub mod adapter;
pub mod router;
pub mod session;
pub mod commands;
pub mod mcp;
pub mod room_manager;

use std::sync::{Arc, Mutex};
use pty::manager::PtyManager;
use pty::commands::{spawn_pty, write_pty, resize_pty, kill_pty, kill_all, list_pty, PtyState};
use session::SessionManager;
use room_manager::RoomManager;
use commands::{AppState, detect_agents, spawn_agent, kill_agent, run_handoff, get_adapter_capabilities};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let pty_manager = Arc::new(Mutex::new(PtyManager::new()));
  let session_manager = Arc::new(SessionManager::new(pty_manager.clone()));
  let broadcaster = Arc::new(Mutex::new(std::collections::HashMap::new()));

  let server_state = pty::server::ServerState {
      pty_manager: pty_manager.clone(),
      broadcaster: broadcaster.clone(),
  };

  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .manage(PtyState {
      manager: pty_manager,
      broadcaster,
    })
    .manage(AppState {
      session_manager,
      room_manager: Arc::new(RoomManager::new()),
    })
    .invoke_handler(tauri::generate_handler![
        spawn_pty,
        write_pty,
        resize_pty,
        kill_pty,
        kill_all,
        list_pty,
        detect_agents,
        spawn_agent,
        kill_agent,
        run_handoff,
        commands::get_room_transcript,
        commands::create_room,
        commands::start_room,
        commands::pause_room,
        commands::stop_room,
        commands::send_manual_reply,
        commands::get_settings,
        commands::update_settings,
        get_adapter_capabilities
    ])
    .setup(move |app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }

      tokio::spawn(async move {
          // We can use a fixed port like 3030 or random, wait, the prompt says http://127.0.0.1:<port>
          // Let's use 3030 for now or load from env.
          pty::server::start_server(3030, server_state).await;
      });

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
