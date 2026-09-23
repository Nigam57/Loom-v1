---
id: VISION
type: guide
status: draft
updated: 2026-09-21
tags: [vision]
---
# Loom Vision

What Loom is: the complete workbench for AI agents. Loom is an un-opinionated, lightweight, standalone desktop application that runs AI coding agents (Claude Code, Sweep, Aider, OpenHands, etc.) inside isolated pseudo-terminals (PTYs). 

Rather than building "the one agent to rule them all," Loom provides the **arena**: the visibility, safety, budget controls, and multi-agent **Conversation Rooms** needed to manage swarms of specialized, locally installed CLI agents safely. Agent-to-agent orchestration is one capability of many, not the whole product. It is an IDE, not an operating system.

Capability map:
- Run: adapters (ACP, headless JSON, PTY), multi-pane terminals, per-agent git worktrees, Job Object process control, a bridge for WSL-only CLIs.
- Coordinate: agent-to-agent router with guards (max turns, timeout, spend cap), Conversation Rooms, a flow view of who is doing what, Arena Mode compare-and-merge.
- Remember: shared semantic memory graph across all agents; Obsidian-style markdown vault with notes, links, and a graph view; exposed to every CLI.
- Extend: paste any GitHub repo and Loom classifies, installs, configures, and exposes it to every connected agent (MCP servers, tools, skills, agents, plugins); one-click GitHub connector for any agent; write-once skills/instructions sync (AGENTS.md to each CLI's format); built-in tools such as Scrapling for web scraping.
- Automate: visual node-based automation with triggers (webhooks, Git pushes, Telegram/remote messaging, schedules) bound to agent workflows, unattended runs with approval gates.
- Observe and control: live traces, diffs, per-agent activity, usage/cost where measurable, approval inbox, global kill switch, audit log.
- Secure: isolation for installed code, permission prompts, secret handling (agents never hold long-lived tokens), prompt-injection defenses, budget and loop guards.
- Workspace UX: Obsidian-style multi-pane/split-window layout, flow and graph views.

Principles: Loom wraps and enhances the CLIs and does not reimplement their agent loops. Local-first, open source (AGPL v3, PROVISIONAL), native Windows, safe by default. Sequencing is not scope: P0/MVP/later phases set build order only, and no capability leaves the vision because it is built later. Nothing unbuilt is faked in the UI; unbuilt areas are labeled "not implemented yet," never simulated.

Core features (stated by the developer): (1) agent-to-agent collaboration with a flow view; (2) one-click GitHub connector for any agent; (3) install anything from GitHub as an extension; (4) shared semantic memory graph with Obsidian-style notes; (5) multi-pane UI with built-in tools (e.g. Scrapling); (6) visual node-based automation with triggers for unattended runs.
Selected differentiators (from research, PROVISIONAL): Arena Mode (compare-and-merge across different CLIs in parallel worktrees), write-once skills sync, safety guards.
Candidates to validate (not commitments): role-based teams with cross-review, unified cost dashboard, session replay, conflict-aware merge agent, remote approval inbox, local-model routing, auto-built code knowledge graph.
Phasing (build order only): P0 walking skeleton (real CLI launch, process control, adapters, router with guards); MVP (memory, worktrees/Arena, extensions and GitHub connector, automation, packaging); later (candidates, macOS/Linux).
Non-goals: not an OS; no cloud/SaaS; no building our own LLM; not replacing the agent CLIs; full VS Code editor parity (ASSUMPTION: record in open-decisions.md for me to confirm).

## Related

- [[HOME]]
- [[AGENT_START_HERE]]
- [[05-Roadmap/roadmap]]
- [[05-Roadmap/checklist]]
- [[05-Roadmap/open-decisions]]
