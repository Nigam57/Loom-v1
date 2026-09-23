---
id: SPEC-AUTOMATION
type: spec
status: in-progress
updated: '2026-09-21'
tags:
- spec
features:
- 6
---

# Spec: Visual Automation Engine

## Related

- [[ADR-008]]
- [[T-069]]
- [[T-073]]

## Scope

Executes node-graph based workflows where nodes are agents, tools, or logical conditions. Supports unattended triggers like webhooks and cron schedules.

## Interfaces/schemas

```json
{
  "nodes": [{"id": "1", "type": "agent"}],
  "edges": [{"source": "1", "target": "2"}]
}
```

## Data

Workflows stored as JSON graphs. Execution state tracked in memory. Emits trace events upon node completion.

## Security

- Threat 1: Unintended destructive actions while unattended. Mitigation: Approval gates pause execution.
- Threat 2: Workflow infinite loops. Mitigation: Maximum depth execution limit.
- Threat 3: Trigger spam. Mitigation: Rate limit webhook endpoints.

## UI

React Flow canvas for drawing nodes and wiring edges. Execution monitor overlay.

## Acceptance tests

- Verify graph executes topologically.
- Verify approval gate successfully pauses engine.
- Verify cycle detection prevents graph execution.

## Open questions

- How do we handle agent state continuity across nodes in a complex graph?
