# Loom: Build Specification

## 1. Pinned Tech Stack
- **Backend:** Rust 1.80+, Tauri v2 (`tauri` and `tauri-build`).
- **PTY/Process Control:** `portable-pty` v0.8, Windows API (`windows-sys` crate for Job Objects).
- **Frontend:** React 18 (TypeScript), Vite 5, TailwindCSS.
- **Terminal Rendering:** `xterm.js` v5.3 + `xterm-addon-webgl`.
- **Visual Node Editor:** Rete.js v2.
- **Local DB (Memory/Config):** SQLite via `rusqlite` + FTS5.

## 2. Component Diagram
```mermaid
graph TD
    subgraph Frontend (React/Webview)
        UI[Main UI Layout]
        T1[Terminal Pane 1 - xterm.js]
        T2[Terminal Pane 2 - xterm.js]
        NodeUI[Rete.js Node Editor]
        UI --> T1 & T2 & NodeUI
    end

    subgraph Backend (Tauri / Rust)
        IPC[Tauri IPC Router]
        PM[Process Manager & Job Objects]
        PTY1[portable-pty 1]
        PTY2[portable-pty 2]
        MemDB[(SQLite / FTS5 Vault)]
        MCPServer[In-Memory MCP Server]
        Adapter[CLI Adapter Layer]
        
        IPC <--> PM
        PM --> PTY1 & PTY2
        PTY1 <--> Adapter
        PTY2 <--> Adapter
        Adapter <--> MemDB
        Adapter <--> MCPServer
    end
    
    Frontend <-->|JSON over IPC| IPC
```

## 3. Module Boundaries & Responsibilities
- `src-tauri/src/pty/`: Manages `portable-pty` spawning and OS-level Job Objects for Windows. Responsible for killing process trees.
- `src-tauri/src/adapter/`: Translates raw stdout/stdin from PTY into structured `AgentEvent` structs.
- `src-tauri/src/mcp/`: An embedded stdio MCP server that Loom exposes to agents to facilitate agent-to-agent memory reads and GitHub token injections.
- `src-tauri/src/vault/`: Reads/writes `.md` files to `.loom/vault/` and syncs them with `rusqlite`.

## 4. Schemas & Interfaces

### CLI Adapter Interface
```rust
trait CliAdapter {
    fn spawn(&self, cmd: &str) -> Result<ChildId, Error>;
    fn parse_stream(&self, chunk: &[u8]) -> Option<AgentEvent>;
    fn capabilities(&self) -> CliCapabilities;
}

struct CliCapabilities {
    pub supports_headless_json: bool,
    pub supports_mcp: bool,
    pub supports_acp: bool,
}
```

### Agent-to-Agent Message/Event Schema (JSON)
```json
{
  "event_id": "uuid",
  "source_agent": "claude-code-1",
  "target_agent": "opencode-2",
  "type": "WORKTREE_MERGE_REQUEST",
  "payload": {
    "branch": "feature/ui",
    "diff_summary": "+45 -12"
  },
  "guards": {
    "loop_count": 2, // Maximum 5 before auto-kill
    "budget_spent": 1.25 // USD
  }
}
```

### SQLite Schema Draft
```sql
CREATE TABLE memory_nodes (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    path TEXT UNIQUE NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE VIRTUAL TABLE memory_fts USING fts5(
    title, content, content='memory_nodes', content_rowid='id'
);
```

### Extension Manifest (loom.json)
```json
{
  "name": "gh-issue-creator",
  "type": "mcp-server",
  "entrypoint": "node dist/index.js",
  "isolation_level": "appcontainer",
  "required_permissions": ["network"]
}
```

## 5. Worktree Lifecycle
1. **Init:** User triggers Arena Mode. Loom runs `git worktree add .loom-trees/arena-claude` and `git worktree add .loom-trees/arena-opencode`.
2. **Execute:** Agents run in isolated folders. Loom binds `cwd` for PTYs to these folders.
3. **Merge/Conflict:** If diffs conflict, Loom halts agents and summons the `Conflict-Aware Merge Agent` (Subagent).
4. **Cleanup:** On UI "Accept", Loom runs `git merge`, then `git worktree remove --force`.

## 6. IPC Design
- Frontend -> Backend: Tauri `invoke` commands (`spawn_agent`, `kill_agent`, `write_memory`).
- Backend -> Frontend: Tauri `emit` for streaming PTY chunks (`pty_data_${id}`), which the frontend directly feeds into `xterm.js.write()`.

## 7. Testing Strategy
- **Unit:** Rust `cargo test` for SQLite Vault and Adapter regex parsing.
- **Fake-CLI Integration:** A mock Rust binary (`mock-claude.exe`) that emits `stream-json` to test PTY loop guards.
- **Windows E2E:** Tauri Webdriver (Playwright) running against the compiled `.exe`.

## 8. MVP Definition & Milestones
*Team Assumption: 1 developer, 20 hrs/week.*

- **Milestone 1: Core Orchestrator (Weeks 1-3)**
  - Tauri setup, 2-pane `xterm.js`, `portable-pty` spawning Windows `cmd.exe` and `claude`. Job Object kill switch.
  - *Acceptance:* Can launch Claude Code, interact via UI, and kill it cleanly.
- **Milestone 2: Agent-to-Agent Memory (Weeks 4-5)**
  - SQLite Vault + local MCP server injected into CLI env.
  - *Acceptance:* Claude writes a note to memory; OpenCode reads the note via MCP.
- **Milestone 3: Arena Mode (Weeks 6-8)**
  - Git worktree automation, side-by-side execution.
  - *Acceptance:* Two agents attempt the same prompt in different worktrees without file conflicts.

*Cut First if short on time:* Extension installer (revert to manual `loom.json` authoring).
