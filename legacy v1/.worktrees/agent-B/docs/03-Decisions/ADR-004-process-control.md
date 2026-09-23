---
id: ADR-004
type: adr
status: CONFIRMED
updated: 2026-09-21
tags: [architecture, security, process-control]
spike: S1
---

# ADR-004: Process Control (Kill Switch)

## Decision
OS Job Objects for process tree termination.

## Context
When a user clicks "Kill All" or closes the app, all spawned agent processes and their children must terminate immediately. On Windows, child processes spawned by CLIs don't automatically die when the parent dies.

## Options Evaluated

| Option | Score | Notes |
|--------|-------|-------|
| **OS Job Objects** | **4.85** | Guaranteed child cleanup, OS-level enforcement |
| Taskkill /f /t | 3.30 | Works but race conditions, no guarantee |
| Graceful SIGTERM | 2.40 | Windows doesn't have SIGTERM natively |

## Evidence
- Job Objects implemented in `loom-desktop/src-tauri/src/pty/manager.rs`
- `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE` flag ensures children die when handle closes
- `Drop` impl on `PtyManager` calls `kill_all_processes()`
- Tests pass: [[evidence/P0.4-P0.5-tests.txt]]

## Counter-Argument
Adds OS-specific unsafe code. Only works on Windows. For cross-platform, would need process group management on Unix.

## Change My Mind If
`std::process` natively supports Windows process groups in a future Rust release.

## Related
- [[ADR-001-framework]]
- [[ADR-006-extension-isolation]]
