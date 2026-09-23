---
id: HOME
type: guide
status: in-progress
updated: 2026-09-20
tags: [navigation, index]
---

# Loom — Project Home

> **Loom** is the complete workbench for AI agents. An open-source, local-first, native Windows `.exe` IDE (Tauri v2) that gives agent CLIs everything an agent needs today, in one place. The CLIs provide the agent loop; Loom provides the environment around them.

## Current Status

| Metric | Value |
|--------|-------|
| Branch | `prototype/p0` |
| Commits | 7 |
| Build | ✅ Passing (NSIS + MSI) |
| Tests | ✅ 4/4 passing |
| Vault | ✅ 24 files, 0 errors |
| Installer | `Loom_0.1.0_x64-setup.exe` (2 MB) |

## Installed CLIs (Verified)

| CLI | Version |
|-----|---------|
| Claude Code | 2.1.228 |
| OpenCode | 1.18.24 |
| AGY | 1.1.23 |

## Vault Map

| Folder | Contents | Files |
|--------|----------|-------|
| [01-Vision/](file:///D:/Loom/docs/01-Vision) | Goals, features, glossary | 1 |
| [02-Research/](file:///D:/Loom/docs/02-Research) | CLI agents table (verified) | 1 |
| [03-Decisions/](file:///D:/Loom/docs/03-Decisions) | ADR-001 through ADR-010 | 8 |
| [04-Spec/](file:///D:/Loom/docs/04-Spec) | Adapter, Router, IPC, Security | 4 |
| [05-Roadmap/](file:///D:/Loom/docs/05-Roadmap) | Roadmap, checklist, decisions | 3 |
| [08-Reviews/](file:///D:/Loom/docs/08-Reviews) | Inventory, v1 review | 2 |
| [09-Agent-Guides/](file:///D:/Loom/docs/09-Agent-Guides) | Tooling log | 1 |
| [_archive/v1/](file:///D:/Loom/docs/_archive/v1) | Original untouched files | — |
| [evidence/](file:///D:/Loom/docs/evidence) | Build + test output | 3 |

## Quick Start

```powershell
cd D:\Loom\loom-desktop

# Dev server (launches the app)
npm run tauri dev

# Run Rust tests
cargo test --manifest-path src-tauri/Cargo.toml

# Full release build
npm run tauri build

# Validate docs vault
python ..\tools\validate_vault.py --vault-root ..\docs
```

## Key Documents

- [[AGENT_START_HERE]] — agent onboarding
- [[05-Roadmap/checklist]] — master progress tracker
- [[05-Roadmap/open-decisions]] — resolved decisions
- [[02-Research/cli-agents]] — verified CLI data
- [[08-Reviews/01-review-of-v1]] — v1 review findings

## Related

- [[AGENT_START_HERE]]
- [[05-Roadmap/checklist]]
