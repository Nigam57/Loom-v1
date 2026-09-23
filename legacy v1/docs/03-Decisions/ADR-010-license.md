---
id: ADR-010
type: adr
status: confirmed
updated: 2026-09-20
tags: [architecture, license]
---

# ADR-010: License

## Decision
AGPL v3 (PROVISIONAL — can be revisited).

## Context
Loom is an open-source project by an individual developer. License choice affects contributor willingness, enterprise adoption, and competitive defense.

## Options Evaluated

| Option | Score | Notes |
|--------|-------|-------|
| **AGPL v3** | **4.15** | Network clause protects against SaaS clones, matches Claude Squad |
| MIT | 3.45 | Maximum adoption but no protection |
| Apache 2.0 | 3.65 | Enterprise-friendly, patent grant, but no copyleft |
| GPL v3 | 3.85 | Strong copyleft but no network clause |
| MPL 2.0 | 3.75 | File-level copyleft, permissive linking |
| BSL/FSL | 3.30 | Source-available, delayed open-source |

## Evidence
- Claude Squad uses AGPL v3 [P] — direct competitor precedent
- AGPL prevents SaaS competitors from using the orchestrator without sharing modifications

## Counter-Argument
AGPL deters enterprise adoption. If the goal is maximum distribution, Apache-2.0 or MIT is better. For an individual developer's project, AGPL's protection outweighs adoption concerns.

## Change My Mind If
Enterprise or large contributor interest requires a more permissive license.

## Related
- [[ADR-009-distribution]]
