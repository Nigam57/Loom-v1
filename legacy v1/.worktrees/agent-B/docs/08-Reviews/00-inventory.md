---
id: INVENTORY-00
type: review
status: done
updated: 2026-09-20
tags: [phase0, inventory]
---

# Phase 0: Inventory & Baseline

## Files Found in Workspace

| File | Size | Description |
|------|------|-------------|
| `loom_research_report.md` | 15.9 KB | Research report with ADRs, competitor matrix, policy matrix, spikes |
| `loom_build_spec.md` | 5.9 KB | Build spec with component diagram, interfaces, schema, milestones |
| `loom_backlog.md` | 3.1 KB | 35 tasks T1-T35, ~250 hrs estimated |
| `traceability.md` | 1.3 KB | 11-row feature→ADR→spec→task mapping |
| `revision_ledger.md` | 4.9 KB | Claims 35 fixed / 0 open across A/B/C/D categories |
| `validate_package.py` | 2.1 KB | Validator with hard-coded D:\Loom path |
| `AGENTS.md` | 2.1 KB | Agent rules (skill requirements, tech stack, verification) |
| `agent.md` | 483 B | Short agent rules pointing to superpowers skills |
| `spikes/` | 10 files | 8 stub scripts + runner + raw_results.txt |
| `archive/v1/` | 4 files | Earlier versions of research, spec, backlog, AGENTS.md |
| `loom-desktop/` | Full scaffold | Tauri v2 + React + xterm.js + PTY manager (has node_modules) |
| `s4_test/`, `s4_tree/` | Spike artifacts | Git worktree test output from S4 spike runner |

## Spike Script States

| Spike | File | Lines | Actual Content | Ran? |
|-------|------|-------|----------------|------|
| S1 ConPTY | `S1_conpty.ps1` | 4 | Comment + UNTESTED marker. No PTY code. | Runner used `Start-Process cmd` (not a PTY) |
| S2 Headless | `S2_headless.sh` | 4 | Comment only. Bash script on Windows. | Runner ran `claude --version` (not stream-json test) |
| S3 Handoff | `S3_handoff.py` | 4 | Comment only. No code. | Not run |
| S4 Worktree | `S4_worktree.ps1` | 4 | Single `git worktree add` command | Runner tested init+commit+worktree+ls. Passed. |
| S5 Usage | `S5_usage.py` | 4 | Comment only. No code. | Not run |
| S6 Tauri | `S6_tauri.ps1` | 4 | Single `npm run tauri build` command | Not run |
| S7 SQLite | `S7_sqlite.sql` | 4 | Single CREATE VIRTUAL TABLE statement | Runner checked `sqlite3.sqlite_version`. Passed (v3.49.1) |
| S8 Concurrency | `S8_concurrency.js` | 4 | Comment only. JS file with `#` comments. | Not run |

**Verdict:** Only S4 (worktree) produced meaningful evidence. S1/S2 ran but tested the wrong things. S3/S5/S6/S8 never ran. All spike scripts are stubs.

## Tools Installed on This Machine

| Tool | Version | Status |
|------|---------|--------|
| git | 2.53.0.windows.1 | ✅ Installed |
| rustup | 1.29.0 | ✅ Installed |
| rustc | 1.94.1 (MSVC) | ✅ Installed |
| cargo | 1.94.1 | ✅ Installed |
| node | v22.19.0 (LTS) | ✅ Installed |
| npm | 11.14.1 | ✅ Installed |
| pnpm | — | ❌ Not installed |
| gh | 2.89.0 | ✅ Installed |
| winget | 1.29.290 | ✅ Installed |
| WebView2 | Unknown | Needs verification |
| VS C++ Build Tools | Unknown | Needs verification (Rust MSVC works, so likely present) |
| Python | 3.x (sqlite3 v3.49.1) | ✅ Installed (used by spike runner) |

## Existing Scaffolded App (`loom-desktop/`)

- **Frontend:** React 19.2.8, Vite 8.3.0, TypeScript 6.0.2, TailwindCSS 4.3.3, xterm 6.0.0
- **Backend:** Tauri 2.11.3, portable-pty 0.8.1, windows-sys 0.61.2
- **Structure:** 3-pane layout (Vault/Arena/Traces), 2 terminal components, Kill All button
- **PTY Manager:** Has Job Object integration, spawn/write/kill commands
- **Issues:** Uses Tauri events (`app.emit`) instead of channels; `take_writer()` called on every write (will fail after first call); no resize support; no adapter abstraction

## Related

- [[01-review-of-v1]]
- [[HOME]]
