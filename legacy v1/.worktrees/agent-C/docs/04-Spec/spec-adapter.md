---
id: SPEC-ADAPTER
type: spec
status: in-progress
updated: '2026-09-21'
tags:
- spec
features:
- 1
---

# Spec: Agent Adapter Module

## Related

- [[ADR-002]]
- [[T-033]]
- [[T-034]]
- [[T-037]]

## Scope

Abstracts communication between Loom and agent CLIs, normalizing diverse terminal output (PTY, NDJSON) into structured `AgentEvent` objects for the router.

## Interfaces/schemas

```rust
pub trait AgentAdapter: Send + Sync {
    async fn spawn(&self, config: SpawnConfig) -> Result<u32, String>;
    async fn send_message(&self, msg: AgentMessage) -> Result<(), String>;
    async fn recv_event(&self) -> Result<AgentEvent, String>;
}
```

## Data

Events flow from stdout streams through the adapter. NDJSON stream parsed directly. PTY stream uses heuristics to detect turn boundaries.

## Security

- Threat 1: Malicious payload in stdout escapes parser. Mitigation: strict JSON schema validation.
- Threat 2: Deadlock from unread streams. Mitigation: bounded channels with timeout.
- Threat 3: Credential leak in raw logs. Mitigation: redact secrets before emitting events.

## UI

No direct UI components. Provides real-time event feeds for the Flow View timeline.

## Acceptance tests

- Verify `HeadlessJsonAdapter` successfully parses Claude stream-json.
- Verify `PtyAdapter` extracts plain text from VT100 sequences.
- Verify `AdapterFactory` falls back correctly.

## Open questions

- Should we support custom parsers via extensions for proprietary CLIs?
