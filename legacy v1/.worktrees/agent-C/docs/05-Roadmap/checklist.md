---
id: CHECKLIST
type: guide
status: in-progress
updated: 2026-09-21
tags: [checklist, tracking]
---

# Master Checklist

> Only tick a box when evidence exists (command output in `docs/evidence/`, commit hash, or verified URL with date).

## Phase 0: Inventory & Baseline
- [x] F-00.1 Git repo initialized on branch `prototype/p0` | commit: b9c93cd
- [x] F-00.2 Vault directory structure created | commit: d1a326e
- [x] F-00.3 Originals archived to `docs/_archive/v1/` | commit: d1a326e
- [x] F-00.4 Inventory document written | [[08-Reviews/00-inventory]]

## Phase 1: Review
- [x] F-01.1 Independent review of v1 deliverables | [[08-Reviews/01-review-of-v1]]

## Phase 2: Obsidian Vault
- [x] F-02.1 HOME.md created | [[HOME]]
- [x] F-02.2 AGENT_START_HERE.md created | [[AGENT_START_HERE]]
- [x] F-02.3 Vision document | [[01-Vision/vision]] (replaced 2026-09-21)
- [x] F-02.4 ADR notes (8 ADRs: 001-010) | [[03-Decisions/]]
- [ ] F-02.5 Spec notes (one per module) — partial: 4 exist, 6 core features need specs
- [ ] F-02.6 Task notes generated — T-### tasks for all 6 core features needed
- [ ] F-02.7 Templates created
- [x] F-02.8 .obsidian config created | commit: d1a326e
- [x] F-02.9 CLAUDE.md and GEMINI.md at repo root | commit: d1a326e
- [ ] F-02.10 build_index.py created

## Phase 3: Roadmap & Checklist
- [x] F-03.1 Roadmap document (corrected 2026-09-21) | [[05-Roadmap/roadmap]]
- [x] F-03.2 This checklist | this file
- [x] F-03.3 Open decisions resolved (D-06 pending) | [[05-Roadmap/open-decisions]]
- [ ] F-03.4 Milestones document

## Phase 4: Tooling
- [ ] F-04.1 WebView2 runtime verified
- [ ] F-04.2 VS C++ Build Tools verified
- [x] F-04.3 validate_vault.py created | `tools/validate_vault.py` passes (24 files, 0 errors)
- [x] F-04.4 Tooling log created | [[09-Agent-Guides/tooling-log]]
- [ ] F-04.5 Project-local skills created
- [ ] F-04.6 just or npm scripts command surface

## Phase 5: Doc Fixes
- [x] F-05.1 Missing backlog tasks and traceability rows added | 55+ tasks generated, roadmap updated
- [ ] F-05.2 Provider policy matrix rebuilt from primary sources
- [x] F-05.3 Signing plan with fallback documented | [[03-Decisions/ADR-009-distribution]]
- [x] F-05.4 Estimates recomputed with ranges — roadmap now has task-based ranges | rollup matches exactly
- [x] F-05.5 ADR statuses corrected (PROVISIONAL where no evidence) | 8 ADRs created with correct statuses
- [ ] F-05.6 CLI table corrected with versions and sources
- [ ] F-05.7 Discovery queries and competitor rows verified — competitor claims need correction
- [x] F-05.8 Spec corrections applied | 10 strict feature specs generated

## Phase 6: Prototype (P0) — 🔧 IN PROGRESS

### Wiring (must pass before UI)
- [x] P0.1 Tauri app scaffolded, builds, NSIS installer | [[evidence/P0.1-tauri-build.txt]]
- [x] P0.2 Terminal panes spawn selected agent CLI (not cmd.exe) | [[evidence/P0-frontend-ipc.txt]]
- [x] P0.3 Job Object kill + Kill All button + Drop impl | commit: c0481a2
- [x] P0.4 AgentAdapter + FakeCli (unit tests pass) | [[evidence/P0.4-P0.5-tests.txt]]
- [x] P0.5 Conversation Rooms with guards accessible via Tauri IPC | [[evidence/P0-ipc-handoff.txt]]
- [x] P0.6 No fake data in UI; unbuilt labeled "not implemented yet" | [[evidence/P0-frontend-ipc.txt]]
- [ ] P0.7 Skin engine with 4-5 original skins (Hermes-inspired) | not started
- [x] P0.8 spawn_agent goes through AgentAdapter (PtyAdapter or FakeCliAdapter) | [[evidence/P0-cli-spawn-tests.txt]]
- [x] P0.9 Conversation Rooms via IPC (FakeCLI only) with guards enforced | [[evidence/P0-ipc-handoff.txt]]
- [x] P0.10 detect_agents resolves CLIs including .cmd shims | [[evidence/P0-cli-spawn-tests.txt]]
- [x] P0.11 PTY resize functional (automated PowerShell test) | fix(pty) support resize commit
- [x] P0.12 Dual-stream data flow (raw bytes to xterm + parsed events to router) | fix(pty) dual-stream commit

### Spikes
- [x] S1 ConPTY spawn + Job Object + PTY read loop | [[evidence/S1-conpty.txt]]
- [x] S2 Claude Code headless JSON (stream-json) | [[evidence/S2-claude-code.ndjson]] [[evidence/S2-agy.ndjson]]
- [ ] P0.S3-S8 remaining spikes | not started

### UI (after wiring passes)
- [ ] T-090 Hermes design research | not started — 2h
- [ ] T-091 Skin engine (YAML, live reload, IPC) | not started — 4h
- [ ] T-092 Ship 4-5 original skins | not started — 4h
- [ ] T-093 Component split (Tailwind + shadcn/ui) | not started — 3h
- [ ] T-094 Skin tests (WCAG AA, validation, repaint) | not started — 2h

## Phase 7: Handoff
- [x] F-07.1 Validator passes | `validate_vault.py` 24 files, 0 errors
- [x] F-07.2 Rust tests pass | 4 passed, 0 failed
- [x] F-07.3 Frontend builds | `npm run build` exit code 0
- [x] F-07.4 Evidence saved to docs/evidence/ | 2 files
- [ ] F-07.5 HOME.md updated with final status
- [ ] F-07.6 Walkthrough created

## Core Feature Traceability

| # | Feature | ADR | Spec | Tasks | Roadmap Phase |
|---|---------|-----|------|-------|---------------|
| 1 | Conversation Rooms + flow view | ADR-002, ADR-003 | spec-conversation | T-030–T-032 | MVP-Coordinate |
| 2 | One-click GitHub connector | ADR-005 | spec-github-connector (needed) | T-040–T-042 | MVP-Extend |
| 3 | Install from GitHub as extension | ADR-006, ADR-012 (needed) | spec-extensions (needed) | T-050–T-053 | MVP-Extend |
| 4 | Semantic memory + vault notes | ADR-007 | spec-memory (needed) | T-060–T-064 | MVP-Remember |
| 5 | Multi-pane UI + built-in tools | ADR-001, ADR-011 (needed) | spec-workspace-ui (needed) | T-070–T-072 | MVP-Run |
| 6 | Visual automation + triggers | ADR-008 | spec-automation (needed) | T-080–T-083 | MVP-Automate |

## Decisions Resolved
- [x] D-01 Auth stance → All auth modes, no spending | [[open-decisions]]
- [x] D-02 Signing route → Unsigned alpha | [[ADR-009-distribution]]
- [x] D-03 License → AGPL v3 (provisional) | [[ADR-010-license]]
- [x] D-04 First CLIs → All major CLIs | [[open-decisions]]
- [x] D-05 Package manager → npm | [[open-decisions]]
- [ ] D-06 VS Code parity non-goal → ASSUMPTION, awaiting confirmation | [[open-decisions]]

## Related

- [[roadmap]]
- [[open-decisions]]
- [[HOME]]
