---
id: SPEC-GITHUB
type: spec
status: in-progress
updated: '2026-09-21'
tags:
- spec
features:
- 2
---

# Spec: GitHub Connector

## Related

- [[ADR-005]]
- [[T-038]]
- [[T-040]]

## Scope

Authenticates user via OAuth Device Flow and injects short-lived, scoped credentials into agent execution environments to securely clone and push repositories.

## Interfaces/schemas

| Method | Input | Output |
|--------|-------|--------|
| `start_device_flow` | None | `DeviceCode` |
| `poll_token` | `DeviceCode` | `OAuthToken` |

## Data

Tokens encrypted at rest via Windows DPAPI. Decrypted only in memory to issue scoped session credentials.

## Security

- Threat 1: Token theft from disk. Mitigation: DPAPI encryption bounds access to current user profile.
- Threat 2: Agent leaks token in logs. Mitigation: Scoped tokens expire in 1 hour; logs scrubbed.
- Threat 3: Malicious agent deletes repositories. Mitigation: Token scopes explicitly restrict destructive operations.

## UI

OAuth flow dialog. Status bar indicator showing active GitHub connection.

## Acceptance tests

- Verify DPAPI encrypts and decrypts token successfully.
- Verify session token expires correctly.
- Verify missing DPAPI fallback behaves securely.

## Open questions

- Should we support Fine-Grained Personal Access Tokens as an alternative?
