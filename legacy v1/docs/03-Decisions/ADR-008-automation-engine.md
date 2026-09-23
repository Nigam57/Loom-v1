---
id: ADR-008
type: adr
status: provisional
updated: 2026-09-20
tags: [architecture, automation]
---

# ADR-008: Automation Engine

## Decision
Custom Rust node-graph engine.

## Context
Loom needs a visual automation engine for unattended agent runs — triggers, conditions, and actions. Post-MVP feature.

## Options Evaluated

| Option | Score | Notes |
|--------|-------|-------|
| **Custom Rust Node-Graph** | **4.20** | Full control, no dependencies, type-safe |
| n8n embedding | 3.40 | Feature-rich but heavy, Node.js dependency |
| Temporal/Inngest | 3.15 | Server-side workflow, wrong paradigm |
| YAML workflow files | 3.80 | Simple but no visual editor |

## Status: PROVISIONAL
Post-MVP. No implementation work until M1-M3 complete.

## Counter-Argument
YAML workflow files are simpler and sufficient for a CLI-oriented tool. Visual editor adds UI complexity.

## Change My Mind If
User feedback shows YAML workflows are sufficient and visual editor is unwanted.

## Related
- [[ADR-002-agent-communication]]
