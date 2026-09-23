---
id: OPEN-DECISIONS
type: decision
status: in-progress
updated: 2026-09-20
tags: [decisions, resolved]
---

# Open Decisions

## Resolved Decisions

### D-01: Auth Stance — RESOLVED

**Decision:** Allow all auth modes. Let users decide.
- API keys for automation
- Subscription login for attended CLI use
- No money spent by Loom itself — users bring their own auth

### D-02: Signing Route — RESOLVED

**Decision:** Unsigned alpha for P0/MVP. No spending. SmartScreen warnings documented. OV certificate deferred to post-MVP if needed.

### D-03: License — PROVISIONAL (AGPL v3)

**Decision:** AGPL v3 stays as default. Can be revisited later.

### D-04: First Supported CLIs — RESOLVED

**Decision:** Support all major CLIs and harnesses:
- Claude Code (headless JSON + PTY)
- Codex (JSONL + PTY)
- OpenCode (JSON + PTY)
- agy (JSON + PTY)
- Hermes Agent (streaming JSON + PTY)
- Odysseus (NDJSON + PTY)
- Any CLI via PTY fallback adapter

### D-05: Package Manager — RESOLVED

**Decision:** npm (already installed, no cost).

### D-06: VS Code Editor Parity — ASSUMPTION (awaiting confirmation)

**Decision (assumed):** Full VS Code editor parity is a non-goal. Loom is an agent workbench, not a general-purpose code editor. Agents use their own editor integrations.

**Status:** ASSUMPTION — recorded per vision.md. Awaiting developer confirmation.

## Related

- [[checklist]]
- [[roadmap]]
