---
id: SPEC-EXTENSIONS
type: spec
status: in-progress
updated: '2026-09-21'
tags:
- spec
features:
- 3
---

# Spec: Extension Manager

## Related

- [[ADR-012]]
- [[T-044]]
- [[T-045]]

## Scope

Handles discovery, classification, installation, and lifecycle management of third-party GitHub repositories containing MCP servers, skills, or agent configs.

## Interfaces/schemas

| CLI Command | Action |
|-------------|--------|
| `install <url>` | Clone, classify, register |
| `update` | Git pull, re-validate |

## Data

Downloads git repositories into `~/.loom/extensions`. Classifies based on heuristics. Updates registry `extensions.toml`.

## Security

- Threat 1: Malicious install scripts. Mitigation: Extensions are never executed during install phase.
- Threat 2: Overwriting system tools. Mitigation: Strict path boundary within extension directory.
- Threat 3: Prompt injection via README. Mitigation: Scrub README contents before injection into agent context.

## UI

Extension marketplace UI showing installed extensions, categorization badges, and update buttons.

## Acceptance tests

- Verify classifier correctly identifies `mcp.json` as MCP Server.
- Verify invalid extension rejects installation.
- Verify uninstall cleans up directory and registry.

## Open questions

- Should we support npm/pip registries natively in addition to GitHub?
