---
id: ADR-013
type: adr
status: confirmed
confirming_spike: null
feature: orchestration
updated: 2026-09-22
tags: [architecture, backend]
---

# ADR-013: Coordination Backend (Claw Orchestrator Evaluation)

## Decision
Confirmed: Option 4 (Rust Spawning + Sidecar). Loom's Rust layer will spawn real CLI processes (using Windows Job Objects for clean teardown) and route I/O to `claw-orchestrator` running as a Node sidecar. The sidecar will handle parsing real CLI streams, council logic, and budget ledgers.

## Context
We evaluated `@enderfga/claw-orchestrator` as a potential backend dependency for Loom to handle multi-agent coordination, persistent sessions, and budget metering, avoiding reinventing the wheel. Crucially, the orchestrator already maintains complex adapters for real-world CLIs (`claude`, `gemini`, `opencode`, etc.).

## Evaluation Findings
1. **Multi-Agent Council:** Real and implemented. The `council` workflow successfully spawns multiple distinct personas (Planner, Implementer, Reviewer). We confirmed it orchestrates multiple independent agents (the preset we ran used the `claude` engine for all three, though config allows mixing), and executes parallel rounds until a consensus marker is reached.
2. **Spend-Cap & Ledger:** Highly accurate and actively enforced. We tested an over-budget scenario (budget 0.0001, spend 0.0164) and the orchestrator halted execution with an error, protecting the budget. The `clawo runs` ledger accurately tracked turns and costs.
3. **Windows Support:** Critically flawed out of the box.
   - Node's `spawn` does not resolve `.cmd` or `.ps1` automatically, causing engines like `gemini` or `opencode` installed via npm to fail (`ENOENT`) out of the box on Windows unless wrapped with `shell: true` (or `cross-spawn`), which we manually patched to confirm cross-engine messaging worked.
   - `tree-kill` on Windows falls back to `process.kill(pid, 'SIGKILL')` because Windows lacks POSIX process groups, leaving child processes orphaned. (There is an open PR: "fix(autoloop): fix paused run resume, Windows process tree cleanup...").
4. **Integration Profile:**
   - **Package Size:** ~432 MB (including `node_modules`).
   - **Startup Time:** ~123 ms. Fast enough for a sidecar.
   - **API Stability:** HTTP/SSE API is very stable and well-suited to build a UI against.

## Options Evaluated

| Option | Fit (25%) | Rel (20%) | Sec (15%) | Val (15%) | Maint (10%) | Def (10%) | Eff (5%) | Total |
|--------|-----------|-----------|-----------|-----------|-------------|-----------|----------|-------|
| **Rust spawning + sidecar** | 5 | 5 | 3 | 5 | 4 | 4 | 2 | **4.35** |
| Use as reference only | 4 | 4 | 4 | 1 | 2 | 4 | 5 | **3.40** |
| Build our own (no ref) | 4 | 4 | 4 | 1 | 2 | 3 | 5 | **3.25** |
| Depend on claw-orchestrator | 2 | 2 | 3 | 4 | 2 | 2 | 1 | **2.40** |

*Scoring: 1 (Poor) to 5 (Excellent).*
*Weighted Total: (Fit×0.25) + (Rel×0.20) + (Sec×0.15) + (Val×0.15) + (Maint×0.10) + (Def×0.10) + (Eff×0.05)*

### Option Notes:
- **Rust spawning + sidecar (Winner):** Gets us real CLI support (claude, opencode, gemini) immediately without rebuilding their complex stream parsers in Rust. Solves Windows process leaks by letting Rust/Tauri own the actual `spawn` via Job Objects, then routing the pipes to the Node sidecar.
- **Use as reference only:** Dropped. While keeping the binary small, rebuilding and continuously maintaining parsing adapters for half a dozen third-party CLIs in Rust is an immense, ongoing maintenance burden (low Value/Maintainability).
- **Depend on claw-orchestrator (as-is):** Suffers from Windows process management issues and orphaned grandchildren.

## Counter-Argument
Shipping a 400MB+ Node payload just for orchestration logic is heavy. However, the time-to-market and reliability gained by not maintaining brittle third-party CLI wrappers in Rust vastly outweighs the binary size concern for an MVP.

## Evidence
- [[evidence/claw_session_transcript.txt]]
- [[evidence/overbudget_halt.txt]]

## Related
- [[ADR-001-framework]]
- [[ADR-004-process-control]]
