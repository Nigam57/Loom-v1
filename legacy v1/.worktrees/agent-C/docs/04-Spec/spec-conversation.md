---
id: SPEC-CONVERSATION
type: spec
status: in-progress
updated: '2026-09-21'
tags:
- spec
features:
- 1
---

# Spec: Conversation Rooms

## Related

- [[ADR-002-agent-communication]]
- [[T-030]]
- [[T-031]]
- [[T-032]]

## Scope

Reframes agent-to-agent interaction from one-way handoffs into Conversation Rooms. Enables multi-agent discussion, stateful adapter persistence, live MCP channel tools (`post_message`, `read_messages`), HITL turn approval, and specific workspace safety rules (Role-based permissions). 

## Interfaces/schemas

| Method | Input | Output |
|--------|-------|--------|
| `create_room` | `goal, agents, guards` | `String (room_id)` |
| `start_room` | `room_id` | `Result<(), String>` |
| `pause_room` | `room_id` | `Result<(), String>` |
| `stop_room` | `room_id` | `Result<(), String>` |
| `send_manual_reply` | `room_id, text` | `Result<(), String>` |

## Data

- `ConversationRoom`: `id`, `goal_prompt`, `participants` (Agent Adapter + Role Prompt).
- `Transcript`: Append-only array of `Message` (id, speaker, text, timestamp, usage).
- `ContextStrategy`: Stateless (full transcript) vs Stateful (incremental).

## Security

- Threat 1: Infinite agent agreement loops ("Looks good"). Mitigation: N-gram loop detector guard and max turns wall.
- Threat 2: Workspace race conditions. Mitigation: Role-based assignments where only the `Writer` agent possesses write capabilities.
- Threat 3: Exhaustive budget drain. Mitigation: Hard stop when `cost_cents` exceeds limit.

## UI

A Conversation View beside terminal panes, displaying messages grouped by agent, timestamps, and guard counters. PTY-only agents display a "Send Reply" button instead of autonomous turn completion.

## Acceptance tests

- `test_fakecli_dialogue`: Scripted conversation exchanged properly with a valid stop rule.
- `test_guard_cutoff`: Force an infinite loop and verify the loop detector fires.
- `test_transcript_persistence`: Verify append-only transcript logging.
- `docs/evidence/P0-conversation-demo.ndjson`: Attended demo traces.

## Open questions

- Should the conversation state persist across app restarts? (Deferred to V2).
