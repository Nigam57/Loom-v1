---
title: System Architecture
tags: [architecture, rust, tauri, vue]
---

# 🏗️ Architecture

Loom v1 is built on a **Tauri** framework, creating a sharp boundary between the high-performance system layer (Rust) and the rich user interface (Vue 3 / TypeScript).

## 🔌 The Backend (Rust / Tauri)
The `src-tauri` directory houses a highly asynchronous, multi-threaded Rust environment. 

### Core Modules:
1. **`pty.rs` (Pseudoterminals):** 
   - Uses `portable-pty` to spawn actual OS-level terminal instances. 
   - Every "Member" (Agent) assigned to a chat runs as a subprocess here.
2. **`session/mod.rs` & `TerminalSession`:** 
   - Manages the lifecycle, input/output streams, and stability checks (`is_output_stable()`) of running PTYs. 
   - Intercepts raw ANSI escape codes and translates them.
3. **`emulator.rs` & `WeztermEmulator`:** 
   - A custom headless terminal emulator built on top of Wezterm's parsing logic.
   - Converts raw byte streams from the PTY into structured `SnapshotMetrics` and `CellAttributes` so the UI knows exactly what text is on the screen without needing to parse ANSI itself.
4. **`chat_dispatch_batcher.rs` & `orchestrate_chat_dispatch`:**
   - Because AIs type fast, sending every keystroke to the UI via IPC is too slow. The batcher queues inputs/outputs and sends them as chunks (`TerminalDispatchEnvelope`) at a fixed tick rate.
5. **`store.rs` & `ChatDbManager`:**
   - Powered by `rusqlite`. Handles all local persistence of chats, workspace members, and terminal metadata. 
6. **`passiveMonitor.ts` (Bridge):**
   - The Rust backend continuously runs a `diagnostics_log_frontend_batch` to ensure the PTY and UI are perfectly synced.

## 🖥️ The Frontend (Vue 3 / TypeScript)
The `src` directory contains a Vite-powered Vue 3 SPA.

### Core Modules:
1. **`TerminalManager` & `TerminalPane.vue`:**
   - The "God Node" of the frontend.
   - Uses `xterm.js` (`@xterm/xterm`, Canvas, WebGL, Fit addons) to render the exact state of the Rust `WeztermEmulator`.
2. **`terminalBridge.ts`:**
   - Handles the WebSocket-like Tauri IPC (Inter-Process Communication) events. 
   - Uses `ackSession()` and `attachSession()` to ensure no data is dropped between Rust and Vue.
3. **`chatStore.ts` & `projectStore.ts`:**
   - Uses Pinia for reactive state management. 
   - Keeps track of all open workspaces, active members, and unread notification counts (`notificationOrchestratorStore.ts`).
