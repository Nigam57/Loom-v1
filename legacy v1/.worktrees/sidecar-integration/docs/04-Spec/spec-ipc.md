---
id: SPEC-IPC
type: spec
status: in-progress
updated: '2026-09-21'
tags:
- spec
features:
- 1
- 5
---

# Spec: Tauri IPC Commands

## Related

- [[ADR-004]]
- [[T-089]]

## Scope

Defines the Tauri invoke commands used by the React frontend to spawn processes, resize terminals, and exchange messages with the Rust backend router.

## Interfaces/schemas

```typescript
// PTY management
invoke('spawn_pty', { cmd: 'cmd.exe', onEvent: Channel })
invoke('write_pty', { pid: 123, data: 'hello' })
invoke('resize_pty', { pid: 123, rows: 40, cols: 100 })
invoke('kill_pty', { pid: 123 })
invoke('kill_all')
invoke('list_pty')

// Agent management
invoke('detect_agents')            // -> AgentStatus[]
invoke('spawn_agent', { agentId: '...', cwd: '...' })
invoke('kill_agent', { sessionId: '...' })
invoke('get_adapter_capabilities', { agentId: '...' })

// Conversation rooms
invoke('create_room', { goal: '...', agentIds: [...], maxTurns, timeoutSeconds, budgetCents })
invoke('start_room', { roomId: '...' })
invoke('pause_room', { roomId: '...' })
invoke('stop_room', { roomId: '...' })
invoke('send_manual_reply', { roomId: '...', text: '...' })
invoke('get_room_transcript', { roomId: '...' })

// Settings
invoke('get_settings')             // -> AppSettings
invoke('update_settings', { settings: {...} })

// Routing (FakeCLI only)
invoke('run_handoff', { sessionA, sessionB, prompt, config })
```

## Data

JSON payloads passed via Tauri IPC channel. Payloads serialized/deserialized automatically by Serde in Rust backend.

## Security

- Threat 1: Arbitrary command execution via `spawn_pty`. Mitigation: Restrict spawn targets to allowed list.
- Threat 2: IPC flooding. Mitigation: Rate-limit frontend invokes.
- Threat 3: Path traversal in args. Mitigation: Validate working directory bounds.

## UI

Terminal panes bind IPC outputs to xterm.js instances via EventListener.

## Acceptance tests

- Verify `spawn_pty` returns valid PID.
- Verify `resize_pty` propagates bounds to ConPTY.
- Verify non-whitelisted command rejection.

## Open questions

- Do we need binary streams for large terminal outputs instead of JSON IPC?
