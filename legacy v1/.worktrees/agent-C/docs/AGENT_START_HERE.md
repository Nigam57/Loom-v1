---
id: AGENT-START
type: guide
status: done
updated: 2026-09-20
tags: [guide, onboarding, agent]
---

# Agent Start Here

> If you are an AI coding agent (Claude Code, Codex, OpenCode, Antigravity, Hermes, Odysseus), read this entire file before doing anything.

## Reading Order

1. **This file** — conventions, rules, how to pick work
2. [[HOME]] — vault map and current status
3. [[05-Roadmap/checklist]] — what's done, what's next
4. [[05-Roadmap/open-decisions]] — decisions pending human review
5. The specific ADR/spec/task notes for your chosen work item

## Status Vocabulary

| Status | Meaning |
|--------|---------|
| `draft` | Initial content, not reviewed |
| `proposed` | Ready for review |
| `provisional` | Decision made but spike/evidence pending. **Do NOT implement anything depending on a PROVISIONAL ADR.** |
| `confirmed` | Decision verified with evidence (spike passed, source verified) |
| `in-progress` | Work actively being done |
| `done` | Complete with evidence |
| `blocked` | Cannot proceed (document why) |
| `superseded` | Replaced by another decision |

## Note Conventions

### Frontmatter (Required)
Every note has YAML frontmatter:
```yaml
---
id: ADR-001          # Stable ID
type: adr            # adr | spike | task | spec | research | review | guide | decision | milestone
status: provisional  # See vocabulary above
updated: 2026-09-20  # ISO date
tags: [tag1, tag2]
# For tasks:
milestone: M1
depends_on: [T-001]
estimate_hours: 4
feature: process-control
---
```

### Links
- Use `[[wikilinks]]` for internal references
- Every note ends with a `## Related` section
- ADRs link to spikes and tasks
- Tasks link to ADRs and spec notes

### Evidence Rules
1. A checkbox may be ticked ONLY when an evidence link exists: raw command output in `docs/evidence/`, a commit hash, or a primary-source URL with access date
2. Sources: primary = official docs, legal pages, source code, changelogs, issue trackers. Tag `[P]`. Secondary = news/blogs. Tag `[S]`. Include URL and access date.
3. Anything unverifiable is tagged `UNVERIFIED`
4. Never invent numbers or statistics
5. "Should work" does not count as evidence

## How to Pick the Next Task

1. Check [[05-Roadmap/checklist]] for the first unchecked item
2. Verify its dependencies are done (check `depends_on` in frontmatter)
3. Read the linked ADR and spec notes
4. If the task depends on a PROVISIONAL ADR, skip it — pick the next one
5. Mark the task `in-progress`, do the work, save evidence, mark `done`

## Definition of Done

A task is done when:
- [ ] Code compiles without errors
- [ ] Tests pass (cargo test, npm run lint)
- [ ] Evidence saved to `docs/evidence/` (command output, screenshot, etc.)
- [ ] Related notes updated (ADR status, spec, traceability, checklist)
- [ ] Vault validation passes (`python tools/validate_vault.py --vault-root docs` with 0 errors)
- [ ] Committed with a clear message

## Agent Handover & Doc Sync Protocol

1. **Continuous Doc Sync:** After completing any code change or pass, you **MUST** update all corresponding documentation (specs, checklist, ADR statuses, test evidence) after verifying tests.
2. **Remediation on Takeover:** If an incoming agent detects that the previous agent failed to update the docs or left specs/checklists stale, the incoming agent must **verify the real codebase state and synchronize the docs before starting new work**.

## Exact Commands

```powershell
# Dev server (launches app with hot reload)
cd loom-desktop; npm run tauri:dev
# OR: cd loom-desktop; npx tauri dev

# Rust tests
cargo test --manifest-path loom-desktop/src-tauri/Cargo.toml

# Frontend lint
cd loom-desktop; npm run lint

# Full build (NSIS + MSI installers)
cd loom-desktop; npm run tauri:build
# OR: cd loom-desktop; npx tauri build

# Vault validation
python tools/validate_vault.py --vault-root docs
```

## What NOT to Touch

1. `docs/_archive/v1/` and `archive/v1/` — untouched originals, never modify
2. Do not commit secrets, API keys, or tokens
3. Do not use `std::process::Command` without Job Object binding
4. Do not implement features depending on PROVISIONAL ADRs

## Recommended Obsidian Plugins

These are optional but helpful:
- **Dataview** — query tasks and ADR statuses
- **Tasks** — checkbox management
- **Kanban** — visual board from backlog

## Related

- [[HOME]]
- [[05-Roadmap/checklist]]
- [[05-Roadmap/open-decisions]]
