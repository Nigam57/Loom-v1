# Dynamic CLI Agents Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Refactor the codebase to support dynamically loaded CLI agents via JSON manifests instead of hardcoded Typescript and Rust strings, and gracefully handle immediate agent crashes.

**Architecture:** 
1. The backend will expose a new API `get_available_agents` that reads JSON manifests from a `.loom/agents/` directory (with a fallback to built-in agents if none exist). 
2. The frontend will replace its static `BASE_TERMINALS` with this dynamic list.
3. The `semantic_worker` and `prompt_block` will read the prompt marker character defined in the manifest instead of hardcoding `❯`.
4. `TerminalManager` will monitor PTY exit events and emit a Tauri UI event on failure instead of silently deadlocking the batcher queue.

**Tech Stack:** Rust (Tauri), Vue 3, Typescript.

**Spec:** `D:\Loom\Loom v1\obsidian-docs\10_CLI_Agent_Integration_Plan.md`

## Global Constraints

- No code must be broken during intermediate steps.
- Existing agents in the database should map successfully.

---

### Task 1: Define Backend Manifest System and `get_available_agents` IPC

**Files:**
- Create: `src-tauri/src/application/agents.rs`
- Modify: `src-tauri/src/lib.rs`
- Modify: `src-tauri/src/ui_gateway/mod.rs` (if necessary to route the IPC command)

**Interfaces:**
- Produces: `get_available_agents()` Tauri command returning `Vec<AgentManifest>`

- [x] **Step 1: Write `AgentManifest` Struct and IPC command**

```rust
// In src-tauri/src/application/agents.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentManifestParsing {
    pub prompt_marker: String,
    pub bullet_marker: String,
    pub ready_signal_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentManifest {
    pub id: String,
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub parsing: AgentManifestParsing,
}

#[tauri::command]
pub fn get_available_agents() -> Result<Vec<AgentManifest>, String> {
    // For now, return the hardcoded default replacement: Antigravity CLI
    Ok(vec![
        AgentManifest {
            id: "antigravity-cli".to_string(),
            name: "Antigravity".to_string(),
            command: "agy".to_string(),
            args: vec![],
            parsing: AgentManifestParsing {
                prompt_marker: "❯".to_string(),
                bullet_marker: "•".to_string(),
                ready_signal_ms: 500,
            }
        },
        AgentManifest {
            id: "shell".to_string(),
            name: "Shell".to_string(),
            command: "shell".to_string(),
            args: vec![],
            parsing: AgentManifestParsing {
                prompt_marker: "❯".to_string(),
                bullet_marker: "•".to_string(),
                ready_signal_ms: 0,
            }
        }
    ])
}
```

- [x] **Step 2: Register command in `lib.rs`**

```rust
// inside tauri::Builder::default().invoke_handler(...)
// add application::agents::get_available_agents
```

### Task 2: Frontend Migration to Dynamic Agents

**Files:**
- Modify: `src/shared/constants/terminalCatalog.ts`
- Modify: `src/features/terminal/terminalMemberStore.ts`

**Interfaces:**
- Consumes: `get_available_agents` IPC command.

- [x] **Step 1: Fetch dynamic agents from backend**

```typescript
import { invoke } from '@tauri-apps/api/core';

export interface AgentManifest {
  id: string;
  name: string;
  command: string;
  args: string[];
  parsing: {
    promptMarker: string;
    bulletMarker: string;
    readySignalMs: number;
  };
}

export const fetchAvailableAgents = async (): Promise<AgentManifest[]> => {
  try {
    return await invoke<AgentManifest[]>('get_available_agents');
  } catch (error) {
    console.error('Failed to fetch agents:', error);
    return [];
  }
};
```

- [x] **Step 2: Remove hardcoded terminals**
Replace usages of `BASE_TERMINALS` with reactive state fetched from `fetchAvailableAgents` inside the Vue components or Pinia stores that require it.

### Task 3: Dynamic Prompt Parsing

**Files:**
- Modify: `src-tauri/src/terminal_engine/filters/rules/prompt_block.rs`
- Modify: `src-tauri/src/terminal_engine/session/semantic_worker.rs`

**Interfaces:**
- Produces: Dynamic prompt markers in `is_non_empty_prompt`

- [x] **Step 1: Inject Prompt Marker into `prompt_block.rs`**

```rust
fn is_non_empty_prompt(line: &str, prompt_marker: &str) -> bool {
  let trimmed = line.trim_start();
  if !trimmed.starts_with(prompt_marker) {
    return false;
  }
  let rest = trimmed.trim_start_matches(prompt_marker).trim();
  !rest.is_empty()
}
```
*(Update all surrounding functions in `prompt_block.rs` to accept and pass `prompt_marker`)*.

### Task 4: Catch Immediate PTY Exits

**Files:**
- Modify: `src-tauri/src/runtime/pty.rs` or `emulator.rs`
- Modify: `src-tauri/src/terminal_engine/session/mod.rs`

- [x] **Step 1: Emit crash event**
When `portable-pty` exits, if it exited with a non-zero code immediately, emit `terminal-crash` to the frontend.

- [x] **Step 2: Clear Batcher Queue**
Hook into the exit event to wipe `ChatDispatchBatcher`'s pending queue for that terminal to prevent deadlocks.
