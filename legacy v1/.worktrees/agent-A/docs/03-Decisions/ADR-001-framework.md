---
id: ADR-001
type: adr
status: confirmed
updated: 2026-09-20
tags: [architecture, framework]
spike: S6
---

# ADR-001: Framework & Architecture

## Decision
Tauri v2 (Rust backend) with NSIS installer.

## Context
Loom needs a native Windows `.exe` IDE that is lightweight, secure, and capable of managing multiple CLI processes. The framework must support WebView-based UI, native process management, and efficient IPC.

## Options Evaluated

| Option | Score | Notes |
|--------|-------|-------|
| **Tauri v2 (Rust)** | **4.25** | Small binary, native perf, Rust safety, NSIS support |
| Electron | 3.65 | Proven ecosystem but heavy (150 MB+), memory per window |
| Wails (Go) | 3.55 | Good perf but smaller ecosystem |
| Qt (C++) | 3.45 | Mature but complex build, no web UI |
| Native Rust UI | 3.45 | Maximum perf but immature widget ecosystem |
| Avalonia (.NET) | 3.40 | Good Windows support but .NET dependency |
| Flutter | 2.75 | Mobile-first, Windows desktop support immature |

*Weights: Fit 25%, Reliability 20%, Security 15%, Value 15%, Maintainability 10%, Defensibility 10%, Efficiency 5%*

## Evidence
- P0.1 build succeeded: `Loom_0.1.0_x64-setup.exe` (2 MB NSIS installer) — [[evidence/P0.1-tauri-build.txt]]
- Tauri v2 bundler docs [P] https://v2.tauri.app/guides/distribute/ (2026-09-19)

## Counter-Argument
Tauri IPC bridging can add latency for high-throughput terminal data. Mitigated by using Tauri Channels (not events).

## Change My Mind If
Tauri IPC limits terminal rendering to <30 FPS after S8 spike.

## Related
- [[ADR-002-agent-communication]]
- [[ADR-004-process-control]]
