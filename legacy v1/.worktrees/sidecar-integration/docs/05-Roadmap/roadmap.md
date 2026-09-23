---
id: ROADMAP
type: milestone
status: in-progress
updated: 2026-09-21
tags: [roadmap, planning]
---

# Loom Roadmap

## Resolved Decisions (Impact on Roadmap)
- **Auth:** All modes allowed — users bring their own (D-01)
- **CLIs:** All major CLIs supported: Claude Code, Codex, OpenCode, agy, Hermes, Odysseus (D-04)
- **Signing:** Unsigned alpha, no spending (D-02)
- **License:** AGPL v3 provisional (D-03)
- **Package Manager:** npm (D-05)
- **VS Code parity:** Non-goal (ASSUMPTION D-06, awaiting confirmation)

## Assumptions
- 1 developer, ~20 hrs/week
- Developer based in India (IST timezone) — confirmed from locale
- Windows-first development
- All auth modes allowed per D-01
- Handoff between real PTY agents not available until HeadlessJson adapter (MVP-Coordinate)

## Phase Overview

| Phase | Description | Est Hours | Status |
|-------|-------------|-----------|--------|
| **P0** | Walking skeleton | 40-60 | 🔧 In Progress — revision |
| **MVP-Coordinate** | Flow view, N-agent routing, Arena Mode | 28-63 | ⏳ |
| **MVP-Extend** | GitHub connector, extensions, skills sync, tools | 48-110 | ⏳ |
| **MVP-Remember** | Memory graph, vault notes, graph view, MCP | 36-77 | ⏳ |
| **MVP-Run** | Multi-Pane UI + Built-in Tools | 18-40 | ⏳ |
| **MVP-Automate** | Node-graph engine, triggers, approval gates | 25-54 | ⏳ |
| **MVP-Observe** | Traces, diffs, cost, audit, approval inbox | 15-36 | ⏳ |
| **MVP-Secure** | Budget proxy, credential scoping, AppContainer | 12-26 | ⏳ |
| **MVP-Package** | CI, E2E, release pipeline, README | 16-34 | ⏳ |
| **Later** | Candidates to validate, macOS/Linux | TBD | ⏳ |

*Ranges rolled up from task-level estimates. P0 is build-order only.*

## P0: Walking Skeleton — 🔧 IN PROGRESS (revision)

| Item | Description | Status | Evidence |
|------|-------------|--------|----------|
| P0.1 | Tauri v2 + React + TS + NSIS installer | ✅ | [[evidence/P0.1-tauri-build.txt]] |
| P0.2 | Terminal panes spawn selected agent CLI (not cmd.exe) | ✅ | AdapterFactory routes to PtyAdapter/HeadlessJson |
| P0.3 | Job Object kill + Kill All + Drop cleanup | ✅ | commit c0481a2 |
| P0.4 | AgentAdapter trait + FakeCli (2 tests pass) | ✅ (unit only) | [[evidence/P0.4-P0.5-tests.txt]] |
| P0.5 | A2A router with guards accessible via IPC | ✅ | Conversation Rooms wired to IPC — [[evidence/P0-ipc-handoff.txt]] |
| P0.6 | No fake data in UI; unbuilt labeled "not implemented yet" | ✅ | detect_agents uses where.exe; unbuilt views show placeholders |
| P0.7 | Skin engine with 4-5 original skins | ⏳ | Not started |
| P0.8 | spawn_agent goes through AgentAdapter | ✅ | Tested via IPC |
| P0.9 | Conversation Room via IPC (FakeCLI only) with guards | ✅ | Tested via IPC |
| P0.10 | detect_agents resolves CLIs including .cmd shims | ✅ | Tested via IPC |
| P0.11 | PTY resize functional (automated test) | ⏳ | Not started |
| P0.12 | Dual-stream data flow (xterm + router) | ✅ | Dual stream tested |

### P0 Defect Log

#### P0-defect-1
Terminal.tsx hardcodes `cmd.exe` at line 86 regardless of which agent is selected. The `cmd` prop from agent config is never used. (FIXED)

#### P0-defect-2
lib.rs registers only PTY commands (spawn_pty, write_pty, etc). The Router and AgentAdapter modules exist in Rust and pass unit tests but are unreachable from the frontend via IPC. P0.5 was ticked without evidence of IPC-level handoff. (FIXED: Conversation Rooms exposed)

#### P0-defect-3
The sidebar shows all agents with colored status dots as if they are connected. No agent detection runs. Budget/turn numbers are not metered. The UI simulates functionality that does not exist.

## MVP-Coordinate (Core Feature 1)

Estimate: 4-12h

- [x] T-030: Conversation Flow data model (Transcript)
- [x] T-031: Conversation Flow React component
- [x] T-032: Conversation Rooms (N-agent routing)
- [x] T-033: HeadlessJsonAdapter
- [x] T-034: Adapter factory
- [x] T-035: Approval UI (HITL pause)
- [ ] T-036: Arena Mode compare-and-merge
- [x] T-037: PtyAdapter turn detection

## MVP-Extend (Core Features 2+3)

Estimate: 48-110h

- [ ] T-038: GitHub OAuth Device Flow
- [ ] T-039: One-click repo clone + worktree
- [ ] T-040: Per-agent GitHub credential injection
- [ ] T-041: GitHub connection status UI
- [ ] T-042: DPAPI secret storage
- [ ] T-043: Agent credential scoping
- [ ] T-044: Deterministic extension classifier
- [ ] T-045: Extension installer
- [ ] T-046: Extension registry and lifecycle
- [ ] T-047: Extension exposure to agents
- [ ] T-048: Extension management UI
- [ ] T-049: README prompt-injection scrub
- [ ] T-050: Permission prompts for extension tools
- [ ] T-051: Per-CLI config fan-out
- [ ] T-052: AppContainer isolation wrapper

## MVP-Remember (Core Feature 4)

Estimate: 36-77h

- [ ] T-053: SQLite + FTS5 + sqlite-vec
- [ ] T-054: Markdown vault filesystem watcher
- [ ] T-055: Graph view UI — node layout
- [ ] T-056: Graph view UI — edges and interactions
- [ ] T-057: MCP server for memory
- [ ] T-058: Note editor with wikilinks
- [ ] T-059: Memory search API
- [ ] T-060: Embedding pipeline
- [ ] T-061: Provenance tracking
- [ ] T-062: Write contention handling

## MVP-Run (Core Feature 5)

Estimate: 18-40h

- [ ] T-063: Dockable panel system
- [ ] T-064: Panel serialization
- [ ] T-065: Tool registry and panel
- [ ] T-066: Scrapling MCP integration
- [ ] T-067: Config file support
- [ ] T-068: WSL bridge for WSL-preferred CLIs

## MVP-Automate (Core Feature 6)

Estimate: 25-54h

- [ ] T-069: Node-graph data model
- [ ] T-070: Node-graph visual editor — canvas
- [ ] T-071: Node-graph editor — edge wiring
- [ ] T-072: Trigger bindings
- [ ] T-073: Unattended run engine
- [ ] T-074: Automation testing harness

## MVP-Observe

Estimate: 15-36h

- [ ] T-075: Audit log NDJSON generator
- [ ] T-076: Live trace viewer UI
- [ ] T-077: Per-agent diff view
- [ ] T-078: Usage/cost dashboard
- [ ] T-079: Approval inbox UI

## MVP-Secure

Estimate: 12-26h

- [ ] T-080: Budget metering proxy
- [ ] T-081: Write-once skills sync
- [ ] T-082: Git worktree management
- [ ] T-083: Skin engine

## MVP-Package

Estimate: 16-34h

- [ ] T-084: GitHub Actions CI
- [ ] T-085: NSIS installer generation
- [ ] T-086: Code signing
- [ ] T-087: tauri-driver E2E test suite
- [ ] T-088: README with screenshots
- [ ] T-089: PTY resize IPC + test

## Related

- [[checklist]]
- [[open-decisions]]
