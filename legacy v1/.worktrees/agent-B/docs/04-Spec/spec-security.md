---
id: SPEC-SECURITY
type: spec
status: in-progress
updated: '2026-09-21'
tags:
- spec
features:
- 3
---

# Spec: AppContainer Isolation

## Related

- [[ADR-006]]
- [[T-042]]
- [[T-052]]

## Scope

Sandboxes third-party extensions and MCP servers inside Windows AppContainers to prevent unauthorized filesystem, network, and registry access.

## Interfaces/schemas

```rust
pub fn launch_isolated(exe: &Path) -> Result<Process, Error> {
    // Binds process to AppContainer profile
}
```

## Data

Extensions store state only in isolated AppData folders. IPC strictly validates incoming data schemas.

## Security

- Threat 1: Extension reads user ssh keys. Mitigation: AppContainer denies default filesystem access.
- Threat 2: Extension crypto-mines. Mitigation: Job object CPU limits.
- Threat 3: Network exfiltration. Mitigation: Block network capability unless explicitly granted.

## UI

Security badges display isolation status in Extension Manager UI.

## Acceptance tests

- Verify isolated process cannot read outside allowed directories.
- Verify network requests fail without explicit capability grant.
- Verify CPU quota enforcement.

## Open questions

- Does AppContainer break Python virtual environments requiring symlink support?
