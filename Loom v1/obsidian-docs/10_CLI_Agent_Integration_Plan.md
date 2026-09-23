---
title: CLI Agent Integration Plan
tags: [architecture, clis, agents, migration, dynamic]
---

# 🚀 Dynamic CLI Agent Integration Plan

## The Current State & Root Cause Analysis

As identified during systematic debugging, the multi-agent chat system is currently deadlocking because of a tightly coupled, hardcoded integration with outdated CLI tools (like the deprecated `gemini` CLI).

1. **The Deprecation Failure:** The UI instructs the backend to spawn `gemini`. However, the Gemini CLI has been deprecated in favor of the `agy` (Antigravity) CLI. When spawned, `gemini` immediately crashes with an `IneligibleTierError` and exits.
2. **The Deadlock:** Because the CLI crashes instantly, it never outputs the expected `❯` prompt character or the internal ready signal. The `ChatDispatchBatcher` places your message in a queue waiting for the agent to become "ready" (`shell_ready = true`), but because the agent is dead, the queue stalls indefinitely.
3. **Hardcoded Fragility:** The frontend has a hardcoded `terminalCatalog.ts` listing specific `TerminalType`s (`codex`, `gemini`, `claude`). The backend Rust code (`prompt_block.rs` and `semantic_worker.rs`) has hardcoded assumptions about what the prompt character (`❯`) and bullet markers (`•`) look like.

## The Plan: Data-Driven Agent Manifests

To keep up with rapid CLI updates (such as migrating from `gemini` to `agy`) without needing to recompile the frontend or backend, Loom should migrate to a **Data-Driven Plugin System**.

### 1. Dynamic Agent Manifests
Instead of hardcoding agents in TypeScript, agents should be defined by JSON/YAML manifests. This allows users (and AI) to dynamically patch or add new CLIs.

```json
{
  "id": "antigravity-cli",
  "name": "Antigravity",
  "command": "agy",
  "args": ["--interactive"],
  "parsing": {
    "prompt_marker": "❯",
    "bullet_marker": "•",
    "ready_signal_ms": 500
  }
}
```

### 2. Frontend Decoupling
The Vue frontend (specifically `terminalMemberStore.ts` and the Add Agent UI) will stop importing the static `BASE_TERMINALS` array. Instead, it will make a Tauri IPC call (e.g., `get_available_agents()`) to read the local manifests. This means adding a new CLI is as simple as dropping a JSON file into a `.loom/agents/` folder.

### 3. Dynamic Rust Filtering
The backend `semantic_worker.rs` and `prompt_block.rs` will no longer hardcode `trimmed.starts_with('❯')`. Instead, when the `WeztermEmulator` processes the stream, it will use the `prompt_marker` defined in the agent's active configuration.

### 4. Resilient Lifecycle Management
To prevent silent deadlocks, `TerminalManager` must monitor the PTY's process exit code. If the spawned CLI crashes immediately (like `gemini` currently does), the backend must:
1. Clear the `ChatDispatchBatcher` queue for that agent.
2. Emit an event to the frontend to mark the avatar with a ⚠️ "Failed to Start" badge.
3. Pipe the `stderr` (e.g., "This client is no longer supported...") directly into the chat UI as a system error message, so the user knows exactly why the agent isn't replying.
