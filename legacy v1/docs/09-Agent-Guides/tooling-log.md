---
id: TOOLING-LOG
type: guide
status: done
updated: 2026-09-21
tags: [tooling, log]
---

# Tooling Log

## Skills Used

| Skill | Purpose | When |
|-------|---------|------|
| `using-superpowers` | Established skill invocation requirements | Session start |
| `brainstorming` | Explored scope before planning | Session 1 |
| `writing-plans` | Created implementation plan | Session 1 |
| `executing-plans` | Executing approved plan | Session 1-2 |
| `verification-before-completion` | Evidence-first completion claims | Ongoing |

## Tools & Software Verified

| Tool | Version | Purpose | Installed? |
|------|---------|---------|------------|
| **Node.js** | ≥18 | Vite frontend | ✅ Yes |
| **npm** | ≥9 | Package manager | ✅ Yes |
| **Rust/Cargo** | stable | Tauri backend | ✅ Yes |
| **Tauri CLI** | 2.11.4 | Build + dev | ✅ (devDep) |
| **Python** | 3.13 | Vault validator | ✅ Yes |
| **PyYAML** | latest | Frontmatter parsing | ✅ Installed |
| **oxlint** | 1.81.0 | Frontend linting | ✅ (devDep) |
| **NSIS** | — | Windows installer | ✅ (Tauri bundled) |
| **WiX** | — | MSI creation | ✅ (Tauri bundled) |
| **Claude Code** | 2.1.228 | Agent CLI | ✅ Installed |
| **OpenCode** | 1.18.24 | Agent CLI | ✅ Installed |
| **AGY** | 1.1.23 | Agent CLI | ✅ Installed |
| **Codex** | — | Agent CLI | ❌ Not installed |
| **Hermes** | — | Agent CLI | ❌ Not installed |
| **Odysseus** | — | Agent CLI | ❌ Not installed |

## Project Scripts

| Command | Description |
|---------|-------------|
| `npm run tauri:dev` | Launch app with hot reload |
| `npm run tauri:build` | Full release build (NSIS + MSI) |
| `npm run build` | Frontend only (tsc + vite) |
| `npm run lint` | oxlint on 6 frontend files |
| `cargo test --manifest-path src-tauri/Cargo.toml` | 4 Rust unit tests |
| `python tools/validate_vault.py --vault-root docs` | Vault integrity check |

## Files Created This Session

| File | Purpose |
|------|---------|
| `tools/validate_vault.py` | Replaces broken `validate_package.py` |
| `.gitignore` | Build outputs, secrets, IDE, OS files |
| `CLAUDE.md` / `GEMINI.md` | Thin pointers to AGENTS.md |
| 8 ADR notes | Individual vault notes with correct statuses |
| 4 spec modules | Adapter, Router, IPC, Security |
| 1 research doc | CLI agents with verified versions |

## Evidence Files

| File | Contents |
|------|----------|
| `docs/evidence/P0.1-tauri-build.txt` | First Tauri build output |
| `docs/evidence/P0.4-P0.5-tests.txt` | Cargo test results (4 passed) |
| `docs/evidence/full-build-post-ui.txt` | Full build after UI overhaul |
| `docs/evidence/P0.2-P0.3-runtime.txt` | Runtime launch evidence |

## Related
- [[AGENT_START_HERE]]
- [[05-Roadmap/checklist]]
