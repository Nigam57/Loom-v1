---
id: ADR-003
type: adr
status: provisional
updated: 2026-09-20
tags: [architecture, budget, metering]
spike: S5
---

# ADR-003: Budget Metering

## Decision
Local API proxy interception for cost tracking.

## Context
Loom must enforce spend caps on agent runs. When agents call LLM APIs, Loom needs to track cumulative cost and halt execution when a budget is exceeded.

## Options Evaluated

| Option | Score | Notes |
|--------|-------|-------|
| **Local API Proxy** | **4.50** | Intercepts API calls, precise metering at request time |
| JSON Output Parse | 4.05 | Parse `total_cost_usd` from agent output; depends on agent support |
| File Log Parse | 3.10 | Parse cost from agent log files; unreliable, delayed |

## Evidence
- Claude Code outputs `total_cost_usd` in stream-json — UNVERIFIED (needs S5 spike)
- Router currently uses `cost_cents: u32` in message schema (P0.5 implementation)

## Status: PROVISIONAL
Waiting for S5 spike to verify cost extraction from real CLI outputs. The local proxy approach adds complexity (must redirect API URLs) and may break if CLIs hardcode endpoints.

## Counter-Argument
Proxies fail if the CLI hardcodes the provider URL. JSON output parsing is simpler and sufficient for non-strict metering.

## Change My Mind If
All agents consistently output cost in `stream-json` mid-turn, making proxy unnecessary.

## Related
- [[ADR-002-agent-communication]]
- [[ADR-005-github-access]]
