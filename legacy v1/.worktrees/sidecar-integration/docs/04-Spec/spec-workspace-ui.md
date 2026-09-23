---
id: SPEC-WORKSPACE-UI
type: spec
status: in-progress
updated: '2026-09-21'
tags:
- spec
features:
- 5
---

# Spec: Workspace Layout System

## Related

- [[ADR-011]]
- [[T-063]]
- [[T-064]]

## Scope

Provides a dockable, multi-pane windowing system for terminal sessions, built-in tools, and agent memory views within the main application window.

## Interfaces/schemas

```tsx
<DockviewReact components={{ terminal: TerminalPane, tool: ToolPane }} />
```

## Data

Layout serialized to JSON and persisted to `~/.loom/config.toml`. Contains panel coordinates, active tabs, and split ratios.

## Security

- Threat 1: XSS in panel titles. Mitigation: React auto-escapes string properties.
- Threat 2: Malicious layout injection crashes app. Mitigation: Schema validate layout JSON before restore.
- Threat 3: Unbounded panel creation memory leak. Mitigation: Cap maximum panels.

## UI

Drag-and-drop tab headers. Resize handles. Floating popout panels.

## Acceptance tests

- Verify layout deserializes correctly on app start.
- Verify split layout maintains relative dimensions on window resize.
- Verify max panel limit enforcement.

## Open questions

- Should we support detaching panels into native OS windows?
