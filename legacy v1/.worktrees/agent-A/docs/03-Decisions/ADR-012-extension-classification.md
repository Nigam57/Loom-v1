---
id: ADR-012
type: adr
status: provisional
updated: 2026-09-21
tags: [architecture, extensions, classification, install]
features: [3]
confirming_spike: null
---

# ADR-012: Extension Classification & Install

## Decision
Deterministic signal-based classifier that infers extension type from files repos already contain (`package.json`, `pyproject.toml`, `mcp.json`, `server.json`, `SKILL.md`, `AGENTS.md`, `README.md`), with user guidance as fallback. Five categories: MCP server, tool, skill, agent-config, plugin. No repo-author work required.

## Context
Loom supports loading extensions from GitHub. Each extension must be classified into exactly one category to determine its execution environment, isolation level, and exposure to agents. Feature 3 requires that classification works automatically from signals repos already have — no special manifest authoring by the extension creator.

## Evaluation Criteria & Weights

| Criterion | Weight | Description |
|-----------|--------|-------------|
| Fit | 25% | How accurately does it classify real-world repos? |
| Reliability | 20% | Determinism, edge case handling, false positive rate |
| Security | 15% | Can a malicious repo game the classifier? |
| Value | 15% | Implementation effort vs. benefit |
| Maintainability | 10% | How easy to update when new signals emerge? |
| Defensibility | 10% | Standards alignment, not locked to one approach |
| Efficiency | 5% | Classification speed, no network calls needed |

## Options Evaluated

| Criterion (Weight) | Signal-based classifier | Manifest-only (`loom-extension.toml`) | LLM-based classification | User-specified category |
|---------------------|------------------------|--------------------------------------|-------------------------|------------------------|
| Fit (25%) | 4 | 3 | 4 | 5 |
| Reliability (20%) | 4 | 5 | 2 | 5 |
| Security (15%) | 3 | 4 | 2 | 4 |
| Value (15%) | 5 | 2 | 1 | 4 |
| Maintainability (10%) | 4 | 5 | 2 | 5 |
| Defensibility (10%) | 4 | 3 | 1 | 3 |
| Efficiency (5%) | 5 | 5 | 1 | 5 |
| **Weighted Total** | **4.05** | **3.55** | **2.15** | **4.45** |

Weighted total (Signal-based):
4×0.25 + 4×0.20 + 3×0.15 + 5×0.15 + 4×0.10 + 4×0.10 + 5×0.05
= 1.00 + 0.80 + 0.45 + 0.75 + 0.40 + 0.40 + 0.25 = **4.05**

Weighted total (Manifest-only):
3×0.25 + 5×0.20 + 4×0.15 + 2×0.15 + 5×0.10 + 3×0.10 + 5×0.05
= 0.75 + 1.00 + 0.60 + 0.30 + 0.50 + 0.30 + 0.25 = **3.70** (corrected)

Weighted total (LLM-based):
4×0.25 + 2×0.20 + 2×0.15 + 1×0.15 + 2×0.10 + 1×0.10 + 1×0.05
= 1.00 + 0.40 + 0.30 + 0.15 + 0.20 + 0.10 + 0.05 = **2.20** (corrected)

Weighted total (User-specified):
5×0.25 + 5×0.20 + 4×0.15 + 4×0.15 + 5×0.10 + 3×0.10 + 5×0.05
= 1.25 + 1.00 + 0.60 + 0.60 + 0.50 + 0.30 + 0.25 = **4.50** (corrected)

**Corrected ranking: User-specified (4.50) > Signal-based (4.05) > Manifest-only (3.70) > LLM-based (2.20)**

**Why signal-based wins over user-specified:** User-specified scores highest but fails the Feature 3 requirement of "no repo-author work" at scale. Signal-based is the chosen primary with user guidance as fallback for ambiguous cases.

## Classification Signal Table

| Signal File | Indicates | Priority |
|-------------|-----------|----------|
| `mcp.json` or `.mcp.json` with `mcpServers` key | MCP server | 1 (highest) |
| `server.json` with `command` key | MCP server | 1 |
| `pyproject.toml` with `[tool.mcp]` section | MCP server (Python) | 1 |
| `package.json` with `"mcp"` in keywords or `bin` with mcp-related name | MCP server (Node) | 2 |
| `SKILL.md` with YAML frontmatter | Skill | 2 |
| `AGENTS.md` or `CLAUDE.md` | Agent config | 3 |
| `package.json` with `bin` field (no MCP signals) | Tool/CLI | 3 |
| `pyproject.toml` with `[project.scripts]` (no MCP signals) | Tool/CLI | 3 |
| Ambiguous / no signals | Prompt user for classification | 4 (fallback) |

## MCP Manifest Format (verified)

The emerging standard uses `mcp.json` with a `mcpServers` object:
```json
{
  "mcpServers": {
    "server-name": {
      "command": "node",
      "args": ["/path/to/server.js"],
      "env": { "API_KEY": "..." }
    }
  }
}
```

MCP registries store metadata: name, version, repository, command, args, env.

## Evidence
- MCP specification uses `mcp.json` / `mcpServers` standard [P] https://modelcontextprotocol.io/ (2026-09-21)
- MCP Registry API: metadata-based discovery (name, version, repository, transport) [P] https://modelcontextprotocol.info (2026-09-21)
- Claude Desktop uses `claude_desktop_config.json` transitioning to `mcp.json` [P] https://gofastmcp.com (2026-09-21)
- VS Code MCP: uses `mcp.json` with `mcpServers` key [P] https://github.com/anthropics/anthropic-cookbook (2026-09-21) — UNVERIFIED exact path
- `package.json` keywords-based classification is standard npm pattern [S]

## Install Flow

1. User provides GitHub URL or selects from registry
2. `git clone --depth 1` into `~/.loom/extensions/<name>/`
3. Classifier scans repo root for signal files (table above)
4. If ambiguous: prompt user with detected signals and suggested category
5. Validate: check for `README.md` (prompt-injection scrub per T-049)
6. Install dependencies based on category (npm install / pip install / cargo build)
7. Register in `~/.loom/extensions.toml` with category, path, version, hash
8. Fan-out config to per-CLI formats (Claude's `.mcp.json`, agy's `mcp` config, etc.)

## Status: PROVISIONAL
No spike. Depends on ADR-006 (extension isolation). Extension classification spike needed before implementation — test against 10 real GitHub MCP repos to measure accuracy.

## Risk
- Signal overlap: a repo with both `package.json` (bin) and `mcp.json` — priority table resolves this
- Prompt injection via README: mitigated by T-049 scrubber
- Classification accuracy: signal-based may misclassify edge cases — user fallback handles this

## Counter-Argument
Starting with MCP servers only (single category) reduces complexity. Full five-category classification can wait until beta when the extension ecosystem is better understood.

## Change My Mind If
A universal standard emerges for AI tool extension packaging (e.g., a `tool.json` manifest adopted by all major IDEs) that Loom can adopt directly.

## Related
- [[ADR-006-extension-isolation]]
- [[spec-extensions]]
- [[T-044]]
- [[T-045]]
- [[T-049]]
