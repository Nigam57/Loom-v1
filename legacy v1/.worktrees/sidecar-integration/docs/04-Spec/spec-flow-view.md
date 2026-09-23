---
id: SPEC-FLOW-VIEW
type: spec
status: in-progress
updated: '2026-09-21'
tags:
- spec
features:
- 1
---

# Spec: Flow View Trace Timeline

## Related

- [[ADR-002]]
- [[T-030]]
- [[T-031]]

## Scope

Visualizes real-time agent activity across multiple sessions using a timeline/swimlane component, rendering traces from the data model.

## Interfaces/schemas

```tsx
<FlowView traces={useTraceStream()} onApprove={handleApprove} />
```

## Data

Consumes `AgentEvent` structs via React context. Persists traces temporarily in memory (or SQLite if enabled).

## Security

- Threat 1: Malicious markdown in trace output. Mitigation: Strict HTML sanitization in React renderer.
- Threat 2: OOM from large trace logs. Mitigation: Ring buffer evicts oldest traces.
- Threat 3: Unapproved tool execution. Mitigation: Action buttons enforce server-side validation.

## UI

Swimlanes per agent. Nodes represent messages, tool invocations, and errors.

## Acceptance tests

- Verify new events append to the bottom of the timeline.
- Verify approval button triggers correct IPC message.
- Verify markdown renders without script execution.

## Open questions

- How do we handle traces that span across multiple days?
