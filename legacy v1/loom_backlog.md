# Loom: Ordered Task Backlog (MVP)

| ID | Task | Dependency | Est (Hrs) | Milestone |
|---|---|---|---|---|
| T1 | Init Tauri v2 project (React 18.3, Vite 5.4, Tailwind). | None | 2 | M1 |
| T2 | Create base 3-pane UI layout (Vault, Arena, Traces). | T1 | 4 | M1 |
| T3 | Integrate `@xterm/xterm` in React component. | T1 | 4 | M1 |
| T4 | Setup Rust: `portable-pty` & `windows-sys` (Job Objects).| T1 | 2 | M1 |
| T5 | Implement `spawn_pty` with Job Object process-tree wrapper.| T4 | 6 | M1 |
| T6 | Implement fast Tauri IPC channels for terminal streaming. | T5 | 4 | M1 |
| T7 | Wire React `xterm` stdin to IPC channels. | T6 | 2 | M1 |
| T8 | Build `kill_process_tree` emergency stop button. | T5 | 2 | M1 |
| T9 | Scaffold `AgentAdapter` trait (args, cwd, resume, usage). | None | 4 | M2 |
| T10 | Implement `HeadlessJson` adapter. | T9 | 8 | M2 |
| T11 | Implement fallback `PTY Parsing` adapter. | T9 | 4 | M2 |
| T12 | Implement `ACP Native` adapter bridge via JSON-RPC. | T9 | 24 | M2 |
| T13 | Build GitHub OAuth Device Flow connector. | None | 8 | M2 |
| T14 | Build local API Proxy for budget metering (cents). | None | 8 | M2 |
| T15 | Build A2A Message Router with loop & budget guards. | T10, T14 | 20 | M2 |
| T16 | Implement UI for Approval Requests & Permission Prompts. | T15 | 12 | M2 |
| T17 | Create Fake-CLI mock binary for routing/E2E testing. | T15 | 4 | M2 |
| T18 | Setup `rusqlite` + FTS5 + `sqlite-vec` + Sync Triggers. | None | 4 | M3 |
| T19 | Build `.loom/vault/*.md` filesystem watcher to SQLite. | T18 | 6 | M3 |
| T20 | Build Semantic Memory Graph View UI. | T18 | 8 | M3 |
| T21 | Scaffold embedded stdio MCP server for agent DB access. | T19 | 8 | M3 |
| T22 | Implement `git worktree add/remove` bindings in Rust. | None | 4 | M3 |
| T23 | Build Arena Mode Compare-and-Merge visual diff UI. | T22, T2 | 16 | M3 |
| T24 | Custom Rust Automation Engine (Node-Graph Execution). | T15 | 24 | M4 |
| T25 | Extension Installer: Deterministic MCP repo classification.| None | 6 | M4 |
| T26 | Extension Installer: AppContainer isolation wrapper. | T25 | 24 | M4 |
| T27 | Extension Installer: Manifest fan-out to CLI configs. | T26 | 4 | M4 |
| T28 | Extension Installer: README prompt-injection scrubber. | T25 | 8 | M4 |
| T29 | Write-Once Skills Sync: Translate `AGENTS.md` to `CLAUDE.md`. | None | 4 | M4 |
| T30 | Implement Built-In Tool: Scrapling integration. | T21 | 4 | M4 |
| T31 | Audit Log JSONL generator (budget & usage events). | T15 | 4 | M4 |
| T32 | Setup GitHub CI/CD Actions matrix build (x86_64). | T1 | 4 | M5 |
| T33 | Integrate Azure Artifact Signing (or OV Cert fallback). | T32 | 4 | M5 |
| T34 | Setup NSIS installer generation in Tauri config. | T32 | 4 | M5 |
| T35 | Write Tauri WebDriver (tauri-driver) E2E suite. | T17 | 8 | M5 |

---
### Time Constraints: What to Cut First
1. **Extension Installer (T25-T28):** Provide no extension support rather than unsandboxed code.
2. **Built-In Tools (T30):** Agents bring their own tools.
3. **Write-Once Skills Sync (T29):** Users manually configure agents.
