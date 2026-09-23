---
id: RESEARCH-CLI
type: research
status: in-progress
updated: 2026-09-20
tags: [research, cli, agents]
---

# CLI Agent Control Surfaces

## Verified on This Machine (2026-09-20)

| CLI | Installed | Version | Source |
|-----|-----------|---------|--------|
| Claude Code | ✅ | 2.1.228 | `claude --version` |
| OpenCode | ✅ | 1.18.24 | `opencode --version` |
| AGY | ✅ | 1.1.23 | `agy --version` |
| Codex | ❌ | — | Not installed |
| Hermes Agent | ❌ | — | Not installed |
| Odysseus | ❌ | — | Not installed |

## Control Surface Matrix

| Feature | Claude Code | OpenCode | AGY | Codex | Hermes | Odysseus |
|---------|------------|----------|-----|-------|--------|----------|
| **TUI Framework** | Ink (Node.js) | Bubble Tea (Go) | UNVERIFIED | Rust CLI | UNVERIFIED | UNVERIFIED |
| **Headless JSON** | `--output-format stream-json` | UNVERIFIED | UNVERIFIED | `--output-format jsonl` [P] | UNVERIFIED | UNVERIFIED |
| **Structured Output** | NDJSON events | UNVERIFIED | UNVERIFIED | JSONL events | UNVERIFIED | UNVERIFIED |
| **Auth: API Key** | `ANTHROPIC_API_KEY` | model-specific | model-specific | `OPENAI_API_KEY` | UNVERIFIED | UNVERIFIED |
| **Auth: Login** | `claude login` | N/A | N/A | `codex login` | UNVERIFIED | UNVERIFIED |
| **Approval Prompt** | Yes (tool use) | UNVERIFIED | UNVERIFIED | Yes (tool use) | UNVERIFIED | UNVERIFIED |
| **MCP Support** | Yes | Yes | UNVERIFIED | UNVERIFIED | UNVERIFIED | UNVERIFIED |
| **Win Native** | ✅ (installed) | ✅ (installed) | ✅ (installed) | UNVERIFIED | WSL2 preferred | Linux only |
| **PTY Compatible** | ✅ | ✅ | ✅ | ✅ | UNVERIFIED | UNVERIFIED |

## Adapter Priority

Based on what's installed and verified:

1. **Claude Code** — Primary target. Has `stream-json`, most features, verified installed.
2. **OpenCode** — Secondary. Installed, Go-based, verify headless mode.
3. **AGY** — Tertiary. Installed, verify capabilities.
4. **Codex** — Install when needed. Rust CLI with JSONL output.
5. **Hermes** — Deferred. WSL2 preference is a concern for Windows-native.
6. **Odysseus** — Deferred. Linux-only per research — needs WSL2 or Wine.

## PTY Fallback

All CLIs can be run via the PTY adapter regardless of structured output support. The PTY adapter:
- Allocates a ConPTY pseudo-terminal via `portable-pty`
- Forwards raw terminal output to xterm.js
- User sees the CLI's native TUI
- No structured event parsing (just terminal display)

## Sources

- Claude Code version: `claude --version` on this machine (2026-09-20) [P]
- OpenCode version: `opencode --version` on this machine (2026-09-20) [P]
- AGY version: `agy --version` on this machine (2026-09-20) [P]
- Claude Code headless: `claude --help` mentions `--output-format` [P]
- Codex JSONL: OpenAI Codex CLI docs [S] — needs verification
- All UNVERIFIED items need spikes or documentation review

## Related
- [[ADR-002-agent-communication]]
- [[SPEC-ADAPTER]]
