---
id: ADR-002
type: adr
status: confirmed
updated: 2026-09-21
tags: [decision, architecture, backend, adapter, communication]
spike: S2
---

# ADR-002: Agent Communication Adapter

## Decision
Transport-agnostic adapter engine with priority: Headless JSON > ACP > PTY parsing.

## Context
Loom must communicate with multiple CLI agents (Claude Code, Codex, OpenCode, agy, Hermes, Odysseus). Each CLI has different output formats and control surfaces. The adapter layer must abstract these differences.

## Options Evaluated

| Option | Score | Notes |
|--------|-------|-------|
| **Headless JSON** | **4.70** | Universal (15/15 agents per harness-test), structured output |
| ACP Native | 4.35 | Standardized protocol but only 12/15 adoption |
| PTY Parsing | 2.65 | Works everywhere but fragile, no structure |
| Filesystem Mailbox | 1.85 | Simple but slow, no streaming |
| MCP-as-Bus | 3.60 | Overkill for agent-to-agent routing |

## Evidence
- Claude Code 2.1.228 accepts `--output-format stream-json` [P] (version confirmed in spike runner)
- harness-test conformance suite [P] https://github.com/belt-sh/harness-test (2026-09-19) — UNVERIFIED exact numbers
- [[evidence/S2-claude-code.ndjson]] - Validated headless JSON stream from Claude Code
- [[evidence/S2-agy.ndjson]] - Validated headless JSON stream from agy

## Counter-Argument
ACP is a standardized protocol with growing adoption. If all CLIs adopt ACP natively, the headless JSON priority should flip.

## Change My Mind If
All target CLIs fully adopt ACP without needing wrappers.

## Supported CLIs (per user decision D-04)
All major CLIs: Claude Code, Codex, OpenCode, agy, Hermes Agent, Odysseus. PTY fallback for any CLI.

## Related
- [[ADR-001-framework]]
- [[ADR-003-budget-metering]]
