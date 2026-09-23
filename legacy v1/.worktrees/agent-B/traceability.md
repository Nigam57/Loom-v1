# Loom: Traceability Matrix

| Feature / Requirement | ADR Decision | Spec Module | Backlog Tasks | Milestone / Test |
|---|---|---|---|---|
| Native Windows `.exe` | Q9: Tauri v2 + NSIS | Release Pipeline | T1, T32, T34, T35 | M1 (Launch), M5 (E2E Install) |
| Process Control (Kill) | Q4: Job Objects | Process Manager | T4, T5, T8 | M1 (Clean Kill Test) |
| A2A Collaboration | Q2.1: Agnostic Transport | Router & Adapter | T9, T10, T12, T15 | M2 (A2A Handoff Test) |
| Strict Budget Limits | Q4.1: API Proxy | Message Router | T14, T15, T31 | M2 (Budget Guard Halt) |
| GitHub Auth & Sync | Q5: OAuth Device Flow | Config Layout | T13, T29 | M2 (Token Generation) |
| Arena Mode (Parallel) | Q11: Top Differentiator | Worktree Lifecycle | T22, T23 | M3 (No-Conflict Parallel Edit) |
| Semantic Memory Graph | Q7: SQLite FTS5+vec | SQLite Schema | T18, T19, T20, T21 | M3 (Agent Read/Write Vault) |
| Ext. Install & Isolation| Q6: AppContainer | Security Model | T25, T26, T27, T28 | M4 (Prompt-Isolated Run) |
| Approval UI | Q6: Permission Prompts | Security Model | T16 | M2 (UI Block Test) |
| Automation Engine | Q8: Custom Rust | Automation Engine| T24 | M4 (Node-Graph Run) |
| CI & Release Pipeline | Q9: Tauri v2 + NSIS | Release Pipeline | T32, T33, T34 | M5 (CI Build + Signed NSIS) |
