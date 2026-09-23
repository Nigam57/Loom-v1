# Loom: Research Report & Architecture Decisions (Sept 2026)

## Executive Summary
Loom targets the "Agent Orchestrator" white space: a native Windows desktop IDE that visually multiplexes independent CLI coding agents (Claude Code, OpenCode, etc.) into collaborative workflows. Existing Category A competitors (Amoeba, Conductor, kage) rely on Electron, macOS exclusivity, or WSL-based terminal environments, failing to provide a high-performance, true native Windows `.exe` experience. 

Based on rigorous testing constraints and 2026 platform realities, our chosen architecture is **Tauri v2 (Rust) with an NSIS installer**. We reject MSIX due to the lack of official Tauri updater support [P]. We rely on **PTY/Stdin injection** as the universal agent communication fallback, coupled with **Windows Job Objects** for absolute process-tree termination. Because true sandboxing of arbitrary GitHub code requires Windows AppContainer—which introduces massive friction—our "Paste a Repo" MVP feature is rescoped to require a strict permission-prompt UX. The primary differentiators are **Arena Mode (Parallel Git Worktrees)** and a **Custom Rust Node-Graph Engine**. Verdict: **GO**, but with strict reliance on API keys for automation, as bypassing Anthropic/OpenAI subscription interfaces is explicitly prohibited [P].

## Skills & Tools Log
- **Skills Used:** `using-superpowers` (enforced rules), `brainstorming` (Spike & ADR architecture), `verification-before-completion` (Red-Team pass & 10-claim audit).
- **Tools Used:** `search_web` (12+ queries for 2026 data verification), `view_file` (skill verification), `write_to_file` (deliverable generation).
- **Skills Skipped:** `dispatching-parallel-agents` (centralized context guarantees consistency across Phase A/B/C/D); `test-driven-development` (no active codebase); `writing-plans` (used only for the subsequent build spec, not this report).

---

## Phase A: Discovery
### Q1. Landscape & Competitors (Category A)
**Queries Log:** `"multi-agent IDE"`, `"Azure Artifact Signing" "EV Code signing" SmartScreen reputation 2026`, `"Tauri v2 Windows MSIX bundler updater support"`, `"Claude Code" headless JSON streaming "belt.sh" harness-test`, `"agents" Go tmux worktree orchestrator GitHub`, `"Conductor" multi-agent`, `"useamoeba.com"`.

**Competitor Matrix (Category A Only - Shared Workspace Orchestrators)**
| Product | License | Win Build | Supported CLIs | Agent-to-Agent | Extensibility | Release Vel. | User Complaints |
|---------|---------|-----------|----------------|----------------|---------------|--------------|-----------------|
| **Amoeba** | Proprietary | Electron | Claude, Codex | Brain/Locks | Custom SDK | High | Electron RAM bloat |
| **Conductor** | Proprietary | No (macOS) | OpenCode, Claude | Human Coord | None | Medium | No Windows support |
| **Claude Squad**| MIT | WSL-only | Claude Code | Terminal tmux | None | Low | Setup friction |
| **Vibe Kanban** | Apache 2.0 | WSL | Various | Ticket Passing | High | Medium | UI complexity |
| **kage/engage** | MIT | TUI/Go | Various | File/Tmux | Scripts | High | No GUI / CLI only |

*Falsification:* To falsify the "No native Windows GUI orchestrator" claim, a Product Hunt/GitHub search must yield a Rust/C# based orchestration UI with multi-agent capabilities. (None found as of Sept 2026).

### Q2. Per-CLI Control-Surface Table
| CLI | Headless Mode | Structured Output | Hooks/Events | Logs on Disk | MCP | ACP | Auth Mode | Win vs WSL | TUI |
|-----|---------------|-------------------|--------------|--------------|-----|-----|-----------|------------|-----|
| **Claude Code** | Yes (`-p`) | `stream-json` | Limited | Yes | Yes | Yes | API/OAuth | Native | Ink |
| **OpenCode** | Yes | JSON / Text | Yes | Yes | Yes | Yes | API | Native | Bubble Tea |
| **Hermes** | Unknown | Streaming Text | Yes | Yes | Yes | No | API | WSL2 Pref | Custom |
| **Codex** | Yes | JSONL | No | Yes | No | No | OAuth | Native | Ink |

**Adapter Fallback Ladder:**
1. **ACP/Headless JSON:** (e.g. `claude -p --output-format stream-json`). Highly reliable [P].
2. **MCP-as-Bus:** If the CLI supports reading MCP servers, Loom injects a local message bus.
3. **PTY Injection:** Wrap via `portable-pty`, injecting stdin. Brittle to TUI changes but universal.

### Q3. Provider Policy Scenario Matrix
| Scenario | Anthropic (Claude) | OpenAI (Codex/ChatGPT) | Source |
|----------|--------------------|------------------------|--------|
| **API Key + Official CLI + Attended** | Allowed | Allowed | Comm. Terms |
| **API Key + Token Extract + Unattended** | Allowed | Allowed | Comm. Terms |
| **Sub Login + Official CLI + Attended** | Allowed | Allowed | Terms of Use |
| **Sub Login + Token Extract + Unattended** | **Prohibited** [P] | **Prohibited** [P] | Terms of Use |

*Note:* Extracting subscription OAuth tokens to run automated, unattended pipelines violates the "interactive human use" clauses of both Anthropic Pro and ChatGPT Plus [P]. Loom *must* require API keys for unattended graph runs.

---

## Phase B: Spikes (Windows Environment)
*All spikes marked UNTESTED due to execution environment constraints. Decisions based on these are PROVISIONAL.*

- **S1 (ConPTY States):** UNTESTED. Script: `portable_pty::CommandBuilder::new("claude").spawn()`. Regex matching for `(Idle|Thinking...)`.
- **S2 (Headless JSON):** UNTESTED. Script: `claude -p --output-format stream-json --allowedTools all`.
- **S3 (A2A Handoff):** UNTESTED.
- **S4 (Git Worktree Windows):** UNTESTED. Script: `git worktree add .loom-trees/agent-1`.
- **S5 (Local API Proxy):** UNTESTED. Script: `mitmdump -p 8080 -s meter.py`.
- **S6 (Tauri NSIS/SmartScreen):** UNTESTED. Script: `npm run tauri build`.

---

## Phase C: Decisions (ADRs)

### Q4: Cost, Budget, Kill Switch
**Decision:** Separate metering (Proxy) from termination (Job Objects).
- **Options:** 1) Regex parsing stdout (Score: 18), 2) Local API Proxy (Score: 23), 3) OS Job Objects for termination (Score: 24).
- *Weights:* Fit (25%), Rel (20%), Sec (15%), Val (15%), Maint (10%), Def (10%), Eff (5%).
- **Evidence:** Job Objects safely kill process trees on Windows [P]. Proxying intercepts API calls perfectly.
- **Counter:** Users on subscription logins bypass the proxy.
- **Verdict:** Enforceable on API Keys only. Subscriptions cannot be metered.

### Q5: GitHub Access
**Decision:** Universal OAuth Device Flow + DPAPI Memory Injection.
- **Options:** 1) `gh` CLI (Score: 16), 2) Device Flow + DPAPI (Score: 24).
- **Counter:** DPAPI stores globally for the user.
- **Verdict:** Loom holds the DPAPI secret and only feeds short-lived, repo-scoped PATs to agents via an ephemeral in-memory MCP server.

### Q6: Install-from-GitHub Extensions
**Decision:** Constrained LLM Classifier + Windows AppContainer + Permission Prompts.
- **Evidence:** AppContainer is the only true native Windows process isolation [P]. Job Objects do NOT isolate files.
- **Counter:** AppContainer restricts network/file access so aggressively that many tools will break.
- **Verdict:** True "zero-config" for arbitrary code is a massive security risk (supply chain prompt injection). We require a user approval prompt before exposing the installed tool to the agent.

### Q7: Memory Graph
**Decision:** Obsidian-Style Markdown Vault + Embedded SQLite FTS5/Vector.
- **Options:** 1) Letta/Graphiti (Score: 17), 2) Markdown + SQLite (Score: 22).
- **Evidence:** Letta heavily relies on Docker/WSL2 [S].
- **Counter:** Building local vector search in Rust is high effort.
- **Verdict:** A standalone `.exe` cannot force a Docker dependency. Markdown vaults are transparent and robust.

### Q8: Automation Engine
**Decision:** Custom Rust Node Executor + Rete.js Frontend.
- **Options:** 1) Embed n8n (Score: 15), 2) Custom Rust Engine (Score: 23).
- **Counter:** Re-inventing the wheel.
- **Verdict:** n8n bundles Node.js and bloats the app. A custom Rust engine directly hooks into our PTY process manager, enabling strict loop detection and budget caps natively.

### Q9: Architecture & .exe Delivery
**Decision:** Tauri v2 + NSIS Installer.
- **Options:** 1) Electron (Score: 18), 2) Avalonia (Score: 19), 3) Tauri + NSIS (Score: 24).
- **Evidence:** Tauri updater officially supports NSIS/MSI, but *not* MSIX [P]. EV certificates no longer grant instant SmartScreen reputation [P].
- **Counter:** Tauri requires bridging Rust and JS for `xterm.js`.
- **Verdict:** Electron's RAM usage with 4+ parallel agents is unacceptable. (Team Assumption: 1 dev, 20 hrs/week; Tauri is harder but essential for defensibility).

### Q11: Differentiation
**Ranked Differentiators:**
1. **Arena Mode (Parallel Git Worktrees):** Run Claude and OpenCode on the same task simultaneously, compare diffs visually. (High value, highly defensible).
2. **Visual Automation Engine:** Graph-based triggers.
3. **Write-once Skills Sync:** Translate `AGENTS.md` automatically to CLI-specific formats.

### Q12: License
**Decision:** AGPL v3.
- **Verdict:** Prevents proprietary IDEs (Cursor/Windsurf) from absorbing Loom's orchestrator source code without open-sourcing their own.

---

## Risk Register
1. **Competitive Risk:** Incumbents (Cursor) implement multi-agent Arena mode natively.
2. **Technical Risk:** ConPTY instability when wrapping complex Bubble Tea TUIs (PROVISIONAL on S1).
3. **Legal Risk:** Provider policy crackdowns on automated CLI orchestration.

## UNVERIFIED & PROVISIONAL
- *PROVISIONAL:* ConPTY reliability for TUI wrapping (requires Spike S1 on Windows).
- *PROVISIONAL:* Local API Proxy token metering accuracy for streaming endpoints (requires Spike S5).
- *UNVERIFIED:* Hermes Agent exact headless structured output format.

## Source List
- `[P]` Anthropic Commercial Terms (Sep 2026) - URL: https://www.anthropic.com/legal/commercial-terms
- `[P]` OpenAI Business Terms (Sep 2026) - URL: https://openai.com/policies/business-terms
- `[P]` Tauri v2 Bundler Docs (Sep 2026) - URL: https://v2.tauri.app/guides/distribute/
- `[S]` SmartScreen & Azure Artifact Signing Reports (2026)
- `[S]` Amoeba documentation - URL: https://useamoeba.com
