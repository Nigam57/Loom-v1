# Loom: Ordered Task Backlog (MVP)

## Pre-Requisites & Scaffold (Tasks 1-5)
1. Initialize Tauri v2 project with React/TypeScript/Vite.
2. Configure TailwindCSS and setup base multi-pane UI layout.
3. Integrate `xterm.js` and `xterm-addon-webgl` in a reusable React component.
4. Setup Rust backend: add `portable-pty` and `windows-sys` dependencies.
5. Create `JobObject` wrapper in Rust for process-tree management on Windows.

## Core Process Management (Tasks 6-12)
6. Implement `spawn_pty` command in Rust that creates a PTY and attaches it to a Job Object.
7. Implement IPC channel to stream PTY stdout bytes to the React frontend.
8. Wire frontend `xterm.js.onData` to send stdin keystrokes via IPC back to Rust.
9. Implement `kill_pty` command to violently terminate the Job Object.
10. Build the `Adapter` trait layout for parsing CLI-specific outputs.
11. Implement fallback Regex parser for Claude Code `(Idle|Thinking|Tool Use)`.
12. Create the "Kill All" emergency stop button in the UI header.

## Semantic Memory Vault (Tasks 13-18)
13. Setup `rusqlite` + FTS5 in `src-tauri` and run initial migrations.
14. Create file-watcher logic (`notify` crate) to sync `.loom/vault/*.md` files into SQLite.
15. Implement `search_memory` IPC command for the frontend.
16. Build the left-hand Sidebar UI to display the Markdown vault tree.
17. Scaffold the embedded stdio MCP server (`src-tauri/src/mcp`).
18. Expose `read_memory` and `write_memory` MCP tools to the injected environment of spawned agents.

## Arena Mode & Git Worktrees (Tasks 19-24)
19. Implement Rust functions for `git worktree add` and `remove`.
20. Build the UI modal for "Launch Arena Session".
21. Modify PTY spawner to accept a custom `cwd` mapping to the newly created worktree.
22. Launch two side-by-side terminal components for Agent A and Agent B.
23. Create a Git diff parser to summarize worktree differences.
24. Implement the UI pane to display Side-by-Side Diffs of the two worktrees.

## Visual Node Automation (Tasks 25-28)
25. Install `rete.js` and render a basic node editor overlay.
26. Create Custom Node Types: `Trigger (Webhook)`, `Agent Task`, `Approval Gate`.
27. Link the execution of an `Agent Task` node to the `spawn_pty` backend command.
28. Implement the `Approval Gate` node to halt execution until the user clicks "Approve" in the UI.

## GitHub Auto-Provisioning (Tasks 29-30)
29. Implement OAuth Device Flow in Rust to fetch a GitHub PAT.
30. Store the PAT securely in Windows DPAPI and expose it via the embedded MCP server as an ephemeral token.

---
### Time Constraints: What to Cut First
If the 20 hr/week schedule falls behind, aggressively cut features in this order to protect the core MVP:
1. **GitHub Auto-Provisioning (Tasks 29-30):** Rely on the user having `gh` CLI installed.
2. **Visual Node Editor (Tasks 25-28):** Revert to a simple JSON config file for automation sequences.
3. **SQLite FTS5 Sync (Tasks 13-14):** Drop SQLite and just let agents read/write raw `.md` files via the file system.
