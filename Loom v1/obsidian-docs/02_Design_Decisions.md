---
title: System Design Decisions
tags: [decisions, architecture, logic]
---

# 🧠 Design Decisions

Why is Loom v1 designed this way? Here is the rationale behind the core architectural choices discovered in the graph.

## 1. Why PTYs (Pseudoterminals) instead of direct APIs?
**Decision:** All AI agents are spawned as full OS processes running inside a `portable-pty` rather than simple HTTP/API wrappers.
**Rationale:** 
By forcing agents to run in a PTY, Loom v1 gains complete environment capture. It doesn't need to know the specific API or integration of the CLI tool. If a tool works in a terminal (like `npm`, `git`, or an AI CLI), it works in Loom. This provides universal compatibility and allows Loom to intercept `stdout` to render rich UI components over it later.

## 2. Why Tauri over Electron?
**Decision:** Using Rust + Tauri instead of Node.js + Electron.
**Rationale:** 
Loom is deeply system-level. It needs to spawn Job Objects, track memory allocations of sub-processes, parse raw ANSI streams in real-time, and run embedded SQLite. Rust provides the necessary safety and performance for this, while Tauri keeps the memory footprint low compared to bundling a full Chromium instance.

## 3. Why the "Multiplayer Chat" abstraction?
**Decision:** UI is designed like Discord/Slack instead of a standard Terminal multiplexer like Tmux.
**Rationale:** 
Terminal windows are intimidating and hard to manage for long-running LLM agents. By treating an agent as a "Member" in a "Chat," users can mentally map the interaction to talking to a coworker. The underlying PTY ensures the agent can still run commands, but the user experiences it as simply sending a chat message.

## 4. Why Wezterm Emulator in Headless Mode?
**Decision:** `WeztermEmulator` (`emulator.rs`) runs headlessly in Rust, parsing the PTY before sending it to the frontend.
**Rationale:**
Normally, a terminal app pipes raw bytes directly to `xterm.js` in the frontend. Loom intercepts it in Rust first to take **Snapshots** (`SnapshotMetrics`). This allows the backend to perform "Semantic Extraction" (e.g., figuring out if the AI is currently typing a block of code, waiting for a prompt, or crashing) without the frontend needing to be active.

## 5. Local-First SQLite (`ChatDbManager`)
**Decision:** All state is local.
**Rationale:**
Privacy and speed. CLI AI tools often have access to sensitive codebase files. Loom v1 ensures no telemetry or chat histories leave the local machine by using local `rusqlite` files (`.loom/workspace.json`).
