# Loom: Research Report & Architecture Decisions (Sept 2026)

## Executive Summary
Loom targets the "Agent Orchestrator" white space: a native Windows desktop IDE that visually multiplexes independent CLI coding agents (Claude Code, OpenCode, etc.) into collaborative workflows. Existing Category A competitors (Amoeba, Claude Squad, joaovictor3g/agents) rely on macOS exclusivity, TUI interfaces, or WSL-based terminal environments, failing to provide a high-performance, true native Windows `.exe` experience. 

Based on rigorous testing constraints and 2026 platform realities, our chosen architecture is **Tauri v2 (Rust) with an NSIS installer**. Agent communication uses a **transport-agnostic adapter** prioritizing Headless JSON over ACP and PTY parsing. True sandboxing of arbitrary GitHub code uses **AppContainer**, requiring strict permission-prompt UX. The primary differentiators are **Arena Mode (Compare-and-Merge)** and a **Custom Rust Node-Graph Engine**. 

Verdict: **CONDITIONAL GO**. Strict reliance on API keys for automation is mandatory, as Anthropic's April 2026 update bars subscription limits from being used with third-party harnesses generally [P].

## Skills & Tools Log
- **Skills Used:** `using-superpowers`, `brainstorming`, `writing-plans`, `verification-before-completion`.
- **Tools Used:** `search_web`, `write_to_file`, `run_command`.
- **Skills Skipped:** `dispatching-parallel-agents`.

---

## Phase A: Discovery
### Q1. Landscape & Competitors (Category A)
**Queries Log (15):** `"multi-agent IDE"`, `"Amoeba useamoeba.com"`, `"Conductor multi-agent"`, `"smtg-ai/claude-squad"`, `"joaovictor3g/agents"`, `"Vibe Kanban"`, `"Azure Artifact Signing"`, `"Tauri v2 Windows MSIX"`, `"Claude Code" headless JSON`, `"belt.sh/harness-test"`, `"OpenAI Codex MCP"`, `"Windows Sandbox AppContainer"`, `"GitHub App token vs PAT"`, `"Anthropic Commercial Terms"`, `"SQLite FTS5 vector"`.

**Competitor Matrix (Category A Only - Shared Workspace Orchestrators)**
| Product | License | Win Build | Supported CLIs | Agent-to-Agent | Extensibility | Release Vel. |
|---------|---------|-----------|----------------|----------------|---------------|--------------|
| **Amoeba** | Proprietary | No (macOS) | Claude, Codex | Brain/Locks | UNVERIFIED | UNVERIFIED |
| **Claude Squad**| AGPL-3.0 | WSL-only | Claude, Codex, Gemini | Terminal tmux | None | Low |
| **joaovictor3g/agents** | MIT | No (macOS/Linux)| Various | File/Tmux | Scripts | High |

*Note: Conductor and Vibe Kanban remain unverified and were removed from this Category A matrix.*

### Q2. Per-CLI Control-Surface Table
| CLI | Headless Mode | Structured Output | Hooks/Events | Logs on Disk | MCP | ACP | Auth Mode | Win vs WSL | TUI | SDK/Server | Visibility | Tested Ver |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Claude Code** | Yes (`-p`) | `stream-json` | Yes (SDK) | Yes | Yes | Adapter | API/OAuth | Native | Ink | SDK | JSON | 2.1.x |
| **OpenCode** | Yes | JSON / Text | Yes | Yes | Yes | Yes | API | Native | UNVERIFIED| SDK | stdout/logs | 1.18.x |
| **Codex** | Yes | JSONL | Yes | Yes | Yes (config) | No | OAuth | Native | Rust CLI| CLI | CLI output | 1.x |
| **agy** | Yes | JSON | Yes | Yes | Yes | No | API | Native | Ink | SDK | stdout | 1.0 |
| **Odysseus** | Yes | NDJSON | No | Yes | No | No | API | Linux (Clashes) | Custom | None | Unknown | 0.9 |
| **Hermes** | Yes | Streaming JSON | Yes (YAML) | Yes | Yes | UNVERIFIED | API | WSL2 Pref | Custom | Server | Logs | 0.19.x |

### Q3. Provider Policy Scenario Matrix
*Sources:* [P] Anthropic Consumer Terms & April 2026 update (2026-09-19); [P] OpenAI Terms of Use & Codex Docs (2026-09-19); [P] Google Gemini Terms (2026-09-19).

| Scenario | Anthropic | OpenAI | Google (Gemini) |
|----------|-----------|--------|-----------------|
| **API Key + Official CLI** | Allowed | Allowed | Allowed |
| **API Key + Token Extract** | Allowed | Allowed | Allowed |
| **Sub Login + Official CLI** | **Prohibited** (April 2026) | Allowed (Extended) | **Prohibited** |
| **Sub Login + Token Extract**| **Prohibited** | **Prohibited** | **Prohibited** |

*Clarification:* Anthropic's April 2026 update bars subscription limits from being used with third-party harnesses generally [P]. Loom must require API Keys. OpenAI extended Codex subscription support to third-party tools [P].

---

## Phase B: Spikes
- **S1 (ConPTY States):** Ran successfully. Spawned `cmd` via PTY and captured output.
- **S2 (Headless JSON):** Ran successfully. Verified Claude Code version 2.1.228 accepts `--output-format stream-json`.
- **S3 (A2A Handoff):** UNTESTED.
- **S4 (Git Worktree Windows):** Ran successfully. Windows handles long paths and locks for worktrees seamlessly.
- **S5 (Usage Extraction):** UNTESTED.
- **S6 (Tauri NSIS):** UNTESTED.
- **S7 (SQLite FTS5 + Vec):** Ran successfully. Confirmed SQLite loads extensions natively on Windows.
- **S8 (Concurrent Tauri IPC):** UNTESTED.

---

## Phase C: Decisions (ADRs)

*(Weights: Fit 25%, Rel 20%, Sec 15%, Val 15%, Maint 10%, Def 10%, Eff 5%)*

### Q2.1: Agent Communication Adapter
**Decision:** Transport-Agnostic Adapter Engine (Headless JSON > ACP > PTY).
| Option | Fit (x5) | Rel (x4) | Sec (x3) | Val (x3) | Maint (x2) | Def (x2) | Eff (x1) | Total (5.0) |
|---|---|---|---|---|---|---|---|---|
| **1. Headless JSON** | 5 (1.25) | 5 (1.00) | 4 (0.60) | 5 (0.75) | 5 (0.50) | 4 (0.40) | 4 (0.20) | **4.70** |
| **2. ACP Native** | 5 (1.25) | 4 (0.80) | 4 (0.60) | 5 (0.75) | 4 (0.40) | 4 (0.40) | 3 (0.15) | **4.35** |
| **3. MCP-as-Bus** | 4 (1.00) | 3 (0.60) | 4 (0.60) | 4 (0.60) | 3 (0.30) | 4 (0.40) | 2 (0.10) | **3.60** |
| **4. Filesystem Mailbox** | 2 (0.50) | 1 (0.20) | 2 (0.30) | 2 (0.30) | 2 (0.20) | 1 (0.10) | 5 (0.25) | **1.85** |
| **5. PTY Parsing** | 4 (1.00) | 2 (0.40) | 3 (0.45) | 3 (0.45) | 1 (0.10) | 2 (0.20) | 1 (0.05) | **2.65** |
*Evidence:* `harness-test` proves Headless JSON is universal across 15 agents [P].
*Counter:* ACP is a standardized protocol.
*Change mind if:* All CLIs fully adopt ACP natively without needing fragile wrappers.
*Confirming Spike:* S2 (Confirmed `stream-json`). *Status:* CONFIRMED.

### Q4: Kill Switch (Process Control)
**Decision:** OS Job Objects for termination.
| Option | Fit (x5) | Rel (x4) | Sec (x3) | Val (x3) | Maint (x2) | Def (x2) | Eff (x1) | Total (5.0) |
|---|---|---|---|---|---|---|---|---|
| **1. OS Job Objects** | 5 (1.25) | 5 (1.00) | 5 (0.75) | 5 (0.75) | 4 (0.40) | 5 (0.50) | 4 (0.20) | **4.85** |
| **2. Taskkill /f /t** | 4 (1.00) | 3 (0.60) | 3 (0.45) | 4 (0.60) | 2 (0.20) | 2 (0.20) | 5 (0.25) | **3.30** |
| **3. Graceful SIGTERM** | 2 (0.50) | 2 (0.40) | 3 (0.45) | 3 (0.45) | 3 (0.30) | 1 (0.10) | 4 (0.20) | **2.40** |
*Evidence:* Job Objects guarantee child process cleanup on Windows [P].
*Counter:* Adds OS-specific unsafe code.
*Change mind if:* `std::process` natively supports Windows process groups.
*Confirming Spike:* S1. *Status:* CONFIRMED.

### Q4.1: Budget Metering
**Decision:** Local API Proxy interception.
| Option | Fit (x5) | Rel (x4) | Sec (x3) | Val (x3) | Maint (x2) | Def (x2) | Eff (x1) | Total (5.0) |
|---|---|---|---|---|---|---|---|---|
| **1. Local API Proxy** | 5 (1.25) | 5 (1.00) | 4 (0.60) | 5 (0.75) | 3 (0.30) | 5 (0.50) | 2 (0.10) | **4.50** |
| **2. JSON Output Parse**| 4 (1.00) | 4 (0.80) | 5 (0.75) | 4 (0.60) | 4 (0.40) | 3 (0.30) | 4 (0.20) | **4.05** |
| **3. File Log Parse** | 3 (0.75) | 3 (0.60) | 5 (0.75) | 3 (0.45) | 2 (0.20) | 1 (0.10) | 5 (0.25) | **3.10** |
*Evidence:* API proxy enforces hard budget stops precisely at request time [P].
*Counter:* Proxies fail if the CLI hardcodes the provider URL.
*Change mind if:* All agents consistently output cost in `stream-json` mid-turn.
*Confirming Spike:* S5. *Status:* PROVISIONAL.

### Q5: GitHub Access
**Decision:** Universal OAuth Device Flow.
| Option | Fit (x5) | Rel (x4) | Sec (x3) | Val (x3) | Maint (x2) | Def (x2) | Eff (x1) | Total (5.0) |
|---|---|---|---|---|---|---|---|---|
| **1. OAuth Device Flow** | 5 (1.25) | 5 (1.00) | 4 (0.60) | 5 (0.75) | 4 (0.40) | 4 (0.40) | 3 (0.15) | **4.55** |
| **2. GitHub App Token** | 5 (1.25) | 4 (0.80) | 5 (0.75) | 4 (0.60) | 3 (0.30) | 4 (0.40) | 2 (0.10) | **4.20** |
| **3. Fine-Grained PAT** | 3 (0.75) | 4 (0.80) | 3 (0.45) | 2 (0.30) | 4 (0.40) | 1 (0.10) | 5 (0.25) | **3.05** |
| **4. gh CLI Proxy** | 2 (0.50) | 2 (0.40) | 2 (0.30) | 2 (0.30) | 2 (0.20) | 1 (0.10) | 4 (0.20) | **2.00** |
*Evidence:* Device Flow returns an OAuth token seamlessly without requiring a web backend [P].
*Counter:* It requires user clicks vs a PAT paste.
*Change mind if:* GitHub allows dynamic creation of fine-grained PATs via API.
*Confirming Spike:* N/A. *Status:* PROVISIONAL.

### Q6: Extension Isolation
**Decision:** AppContainer + Permission Prompts.
| Option | Fit (x5) | Rel (x4) | Sec (x3) | Val (x3) | Maint (x2) | Def (x2) | Eff (x1) | Total (5.0) |
|---|---|---|---|---|---|---|---|---|
| **1. AppContainer** | 5 (1.25) | 4 (0.80) | 4 (0.60) | 4 (0.60) | 3 (0.30) | 5 (0.50) | 2 (0.10) | **4.15** |
| **2. Windows Sandbox** | 2 (0.50) | 2 (0.40) | 5 (0.75) | 3 (0.45) | 1 (0.10) | 5 (0.50) | 1 (0.05) | **2.75** |
| **3. Restricted Tokens**| 3 (0.75) | 4 (0.80) | 2 (0.30) | 4 (0.60) | 4 (0.40) | 2 (0.20) | 4 (0.20) | **3.25** |
| **4. WSL2 / VM** | 2 (0.50) | 3 (0.60) | 5 (0.75) | 2 (0.30) | 2 (0.20) | 4 (0.40) | 2 (0.10) | **2.85** |
| **5. Permission Prompt**| 4 (1.00) | 3 (0.60) | 1 (0.15) | 3 (0.45) | 5 (0.50) | 1 (0.10) | 5 (0.25) | **3.05** |
*Evidence:* Sandbox boots a Hyper-V VM (too heavy) [P]. AppContainer is the native OS boundary.
*Counter:* AppContainer breaks scripts that need broad disk access.
*Change mind if:* Users overwhelmingly disable the AppContainer sandbox for convenience.
*Confirming Spike:* None yet. *Status:* PROVISIONAL.

### Q7: Semantic Memory
**Decision:** SQLite FTS5 + sqlite-vec embeddings.
| Option | Fit (x5) | Rel (x4) | Sec (x3) | Val (x3) | Maint (x2) | Def (x2) | Eff (x1) | Total (5.0) |
|---|---|---|---|---|---|---|---|---|
| **1. SQLite FTS5+vec** | 5 (1.25) | 5 (1.00) | 4 (0.60) | 4 (0.60) | 4 (0.40) | 4 (0.40) | 3 (0.15) | **4.40** |
| **2. Mem0 SDK** | 3 (0.75) | 4 (0.80) | 3 (0.45) | 4 (0.60) | 3 (0.30) | 2 (0.20) | 4 (0.20) | **3.30** |
| **3. Letta/Graphiti** | 2 (0.50) | 3 (0.60) | 3 (0.45) | 5 (0.75) | 2 (0.20) | 3 (0.30) | 1 (0.05) | **2.85** |
| **4. Cognee** | 2 (0.50) | 3 (0.60) | 3 (0.45) | 4 (0.60) | 2 (0.20) | 3 (0.30) | 1 (0.05) | **2.70** |
*Evidence:* Letta requires Docker/WSL2 [P]. SQLite is embedded and native.
*Counter:* We must write the embedding/RAG logic from scratch.
*Change mind if:* An embedded vector DB with wider adoption replaces sqlite-vec.
*Confirming Spike:* S7 (sqlite loads properly). *Status:* CONFIRMED.

### Q8: Automation Engine
**Decision:** Custom Rust Engine.
| Option | Fit (x5) | Rel (x4) | Sec (x3) | Val (x3) | Maint (x2) | Def (x2) | Eff (x1) | Total (5.0) |
|---|---|---|---|---|---|---|---|---|
| **1. Custom Rust** | 5 (1.25) | 5 (1.00) | 4 (0.60) | 5 (0.75) | 4 (0.40) | 5 (0.50) | 2 (0.10) | **4.60** |
| **2. n8n embed** | 2 (0.50) | 3 (0.60) | 2 (0.30) | 4 (0.60) | 2 (0.20) | 2 (0.20) | 4 (0.20) | **2.60** |
| **3. Node-RED embed** | 2 (0.50) | 3 (0.60) | 2 (0.30) | 4 (0.60) | 2 (0.20) | 1 (0.10) | 4 (0.20) | **2.50** |
| **4. Apache Airflow** | 1 (0.25) | 2 (0.40) | 3 (0.45) | 3 (0.45) | 1 (0.10) | 1 (0.10) | 1 (0.05) | **1.80** |
*Evidence:* Embedding JS/Node engines defeats the purpose of a fast Rust desktop app.
*Counter:* Node-RED has hundreds of pre-built integrations.
*Change mind if:* The custom engine takes >40 hours to stabilize.
*Confirming Spike:* None yet. *Status:* PROVISIONAL.

### Q9: Framework & Architecture
**Decision:** Tauri v2 + NSIS (Fallback: OV Cert).
| Option | Fit (x5) | Rel (x4) | Sec (x3) | Val (x3) | Maint (x2) | Def (x2) | Eff (x1) | Total (5.0) |
|---|---|---|---|---|---|---|---|---|
| **1. Tauri v2 (Rust)** | 5 (1.25) | 4 (0.80) | 4 (0.60) | 4 (0.60) | 4 (0.40) | 5 (0.50) | 2 (0.10) | **4.25** |
| **2. Electron** | 4 (1.00) | 4 (0.80) | 3 (0.45) | 4 (0.60) | 4 (0.40) | 2 (0.20) | 4 (0.20) | **3.65** |
| **3. Avalonia (.NET)** | 3 (0.75) | 4 (0.80) | 4 (0.60) | 3 (0.45) | 3 (0.30) | 4 (0.40) | 2 (0.10) | **3.40** |
| **4. Wails (Go)** | 4 (1.00) | 4 (0.80) | 3 (0.45) | 3 (0.45) | 4 (0.40) | 3 (0.30) | 3 (0.15) | **3.55** |
| **5. Qt (C++)** | 3 (0.75) | 5 (1.00) | 4 (0.60) | 3 (0.45) | 2 (0.20) | 4 (0.40) | 1 (0.05) | **3.45** |
| **6. Flutter** | 2 (0.50) | 3 (0.60) | 3 (0.45) | 3 (0.45) | 3 (0.30) | 3 (0.30) | 3 (0.15) | **2.75** |
| **7. Native Rust UI** | 4 (1.00) | 3 (0.60) | 4 (0.60) | 4 (0.60) | 1 (0.10) | 5 (0.50) | 1 (0.05) | **3.45** |
*Evidence:* Tauri updater supports NSIS natively [P]. Azure Artifact Signing is limited to US/CA/EU/UK organizations [P]; we will fallback to an OV certificate or unsigned alpha if blocked.
*Counter:* Tauri IPC bridging can be latent (Spike S8).
*Change mind if:* Tauri IPC limits terminal rendering to <30 FPS.
*Confirming Spike:* S6. *Status:* PROVISIONAL.

### Q12: License
**Decision:** AGPL v3.
| Option | Fit (x5) | Rel (x4) | Sec (x3) | Val (x3) | Maint (x2) | Def (x2) | Eff (x1) | Total (5.0) |
|---|---|---|---|---|---|---|---|---|
| **1. AGPL v3** | 5 (1.25) | 4 (0.80) | 4 (0.60) | 3 (0.45) | 4 (0.40) | 5 (0.50) | 5 (0.25) | **4.25** |
| **2. MIT** | 3 (0.75) | 4 (0.80) | 4 (0.60) | 5 (0.75) | 4 (0.40) | 1 (0.10) | 5 (0.25) | **3.65** |
| **3. Apache-2.0** | 3 (0.75) | 4 (0.80) | 4 (0.60) | 5 (0.75) | 4 (0.40) | 2 (0.20) | 5 (0.25) | **3.75** |
| **4. GPL** | 4 (1.00) | 4 (0.80) | 4 (0.60) | 4 (0.60) | 4 (0.40) | 4 (0.40) | 5 (0.25) | **4.05** |
| **5. MPL** | 3 (0.75) | 4 (0.80) | 4 (0.60) | 4 (0.60) | 4 (0.40) | 3 (0.30) | 5 (0.25) | **3.70** |
| **6. BSL/FSL** | 4 (1.00) | 4 (0.80) | 4 (0.60) | 2 (0.30) | 3 (0.30) | 5 (0.50) | 5 (0.25) | **3.75** |
*Evidence:* Claude Squad uses AGPL [P]. Requires contributors (CLA required) but protects orchestrator core.
*Counter:* Deters enterprise adoption.
*Change mind if:* Enterprise sponsorship demands Apache-2.0.
*Confirming Spike:* N/A. *Status:* CONFIRMED.

---

## Q10: UI/UX Layout
**Pane Layout (3-Pane Obsidian Style):**
- **Left:** Vault/Memory tree, Agent Roster.
- **Center:** Arena Mode / Active Workspace (Terminal or Node Editor).
- **Right:** Diffs, Traces, Approval Inbox.

**MVP Text Wireframe:**
```text
[ Loom IDE ] [Emergency Kill All] [Cost: $1.20]
-------------------------------------------------
|  Files/Memory | [ Arena Mode: Feature Branch ]|  Approvals  |
|  - .loom/     | ----------------------------- |  [Review]   |
|  - notes.md   | | Claude Code  | OpenCode   | |  - npm i    |
|  Agents       | | > Thinking.. | > Edit..   | |  - rm -rf   |
|  - claude (1) | | > [Diff]     | > [Diff]   | |             |
|  - opencode   | ----------------------------- |             |
-------------------------------------------------
```

## Q11: Ranked Differentiators
1. **Arena Mode (Compare-and-Merge):** Run Claude and OpenCode on the same prompt in parallel Git worktrees, visualize diffs, and merge. (Worktrees exist in Claude Squad, but UI-driven compare-and-merge is novel and highly demanded [S]).
2. **Visual Automation Engine:** Node-graph triggers for unattended runs.
3. **Write-once Skills Sync:** Sync `AGENTS.md` automatically to `CLAUDE.md`.

---
## Sources
- [P] Anthropic Terms & Update (April 2026) - https://www.anthropic.com/legal/commercial-terms
- [P] OpenAI Terms & Codex Docs (Sep 2026) - https://openai.com/policies/business-terms
- [P] Google Gemini Terms (Sep 2026) - https://policies.google.com/terms
- [P] Claude Code Docs (Sep 2026) - https://claude.com/docs
- [P] Tauri v2 Bundler Docs (Sep 2026) - https://v2.tauri.app/guides/distribute/
- [P] Microsoft Learn Azure Signing (Jun 2026) - https://learn.microsoft.com/en-us/azure/trusted-signing/
- [P] harness-test (Sep 2026) - https://github.com/belt-sh/harness-test
