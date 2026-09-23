---
id: ADR-005
type: adr
status: provisional
updated: 2026-09-20
tags: [architecture, github, authentication]
---

# ADR-005: GitHub Access

## Decision
OAuth Device Flow for GitHub integration.

## Context
Loom needs to access GitHub repos for Arena Mode worktrees and collaboration features. Must authenticate without storing secrets in the app binary.

## Options Evaluated

| Option | Score | Notes |
|--------|-------|-------|
| **OAuth Device Flow** | **4.05** | No redirect URI, works in desktop apps, standard OAuth |
| GitHub CLI (`gh`) delegation | 3.60 | Simple but adds external dependency |
| PAT manual entry | 3.25 | Simple but poor UX, less secure |
| GitHub App Installation | 3.95 | Rich features but complex for individual developer |

## Evidence
- GitHub Device Flow docs [P] https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/authorizing-oauth-apps#device-flow (2026-09-19)
- Tauri v2 has no built-in OAuth support — must implement via HTTP calls

## Status: PROVISIONAL
No spike implemented. Needs T13 to implement.

## Counter-Argument
If users already have `gh` CLI authenticated, delegating to it is simpler. But adds a dependency.

## Change My Mind If
A Tauri OAuth plugin appears with built-in Device Flow support.

## Related
- [[ADR-002-agent-communication]]
