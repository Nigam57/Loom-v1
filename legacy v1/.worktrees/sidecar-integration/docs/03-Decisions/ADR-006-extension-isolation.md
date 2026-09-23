---
id: ADR-006
type: adr
status: provisional
updated: 2026-09-20
tags: [architecture, security, extensions]
---

# ADR-006: Extension Isolation

## Decision
Windows AppContainer sandboxing for untrusted extensions.

## Context
Loom will support loading user-authored or community extensions (Python/Node scripts). These run arbitrary code and must be sandboxed to prevent filesystem/network access beyond granted permissions.

## Options Evaluated

| Option | Score | Notes |
|--------|-------|-------|
| **AppContainer** | **4.15** | OS-level, filesystem/network isolation, Windows-native |
| Docker container | 3.70 | Strong isolation but Docker Desktop required |
| Firecracker/gVisor | 3.85 | Server-grade, too heavy for desktop |
| WASM sandbox | 3.50 | Portable but limited FFI, no native CLI execution |
| Process-only sandbox | 2.60 | Job Objects + restricted token — weak isolation |

## Evidence
- Windows AppContainer docs [P] https://learn.microsoft.com/en-us/windows/win32/secauthz/appcontainer-isolation (2026-09-19)
- Requires `CreateAppContainerProfile` + `SetTokenInformation` — significant Win32 FFI

## Status: PROVISIONAL
No spike. This is M4 cut-first scope. Only implement if M1-M3 are solid.

## Risk
AppContainer is complex Windows-specific FFI. Estimated 24h but likely 48-96h for production quality. Consider deferring entirely.

## Counter-Argument
Without AppContainer, do not allow extensions at all (current cut-first policy).

## Change My Mind If
Tauri adds a sandboxing API, or WASM component model matures to support CLI execution.

## Related
- [[ADR-004-process-control]]
