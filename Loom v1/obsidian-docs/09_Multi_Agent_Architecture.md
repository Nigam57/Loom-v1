---
title: Multi-Agent CLI Architecture
tags: [architecture, agents, terminal, rust, pty]
---

# 🤖 Multi-Agent CLI Architecture

Loom v1 achieves its "Multi-Agent" environment by wrapping standard CLI tools inside invisible Pseudoterminals (PTYs), managing them in a thread-safe registry, and rendering them as interactive avatars in the frontend.

## 1. The Execution Layer: Pseudoterminals
Every time an AI agent (or "Member") is invoked, it is not just an API call. It is a full operating system process running inside a headless terminal.
- **Proof in Code:** `src-tauri/src/runtime/pty.rs`. The backend uses the `portable-pty` crate to spawn native terminal pairs (PTY master/slave) across Windows/macOS/Linux.
- **How it Works:** When an agent is added to a workspace, the system spawns its executable (e.g., `npx claude-code` or `powershell`) inside one of these PTYs. The agent believes it is interacting with a normal human in a terminal.

## 2. The Management Layer: `TerminalManager`
To support *multiple* agents running simultaneously, the system uses a centralized session registry.
- **Proof in Code:** `src-tauri/src/terminal_engine/session/state.rs`. The `TerminalManager` struct holds a thread-safe `SessionRegistry` (using `Arc<Mutex<...>>`).
- **How it Works:** Each PTY session is assigned a unique `session_id`. `TerminalManager` maps `member_ids` (the identities of your AI friends in the Chat UI) directly to these `session_id`s, allowing the app to juggle dozens of parallel background CLI agents.

## 3. The Orchestration Layer: Chat to CLI Bridge
The illusion of a "Multiplayer Chat" is created by intercepting your chat messages and routing them to the correct agent's terminal `stdin`.
- **Proof in Code:** `src-tauri/src/orchestration/dispatch.rs`. 
- **How it Works:** 
  1. When you hit "Send", the SQLite `chat_outbox_worker` picks up the message and calls `orchestrate_chat_dispatch()`.
  2. The function parses your `@mentions`. (If there are no mentions, but only one AI agent exists in the channel, it smartly auto-targets them via a fallback mechanism).
  3. It calls `ensure_backend_member_session()` to wake up or spawn the targeted agent's PTY.
  4. It calls `batcher.enqueue_for_terminal()` to queue the message, which waits for the `semantic_worker` to confirm the terminal is ready, and then effectively "types" your message into the CLI agent's `stdin`.

## 4. The Visual Layer: Emulation & xterm.js
Because raw PTYs output a messy stream of ANSI escape codes (colors, cursor movements), Loom must parse them to keep the UI clean.
- **Proof in Code (Backend):** `src-tauri/src/terminal_engine/emulator.rs`. The `WeztermEmulator` parses the raw byte stream into a structured 2D grid in Rust before it reaches the frontend.
- **Proof in Code (Frontend):** `src/features/terminal/TerminalPane.vue`. The frontend uses `@xterm/xterm` (xterm.js) and its WebGL addons to draw the terminal canvas.
- **How it Works in UI:** By splitting the terminal grid (using drag-and-drop on the terminal tabs), you can watch multiple CLI agents typing, thinking, and executing commands in real-time, side-by-side.
