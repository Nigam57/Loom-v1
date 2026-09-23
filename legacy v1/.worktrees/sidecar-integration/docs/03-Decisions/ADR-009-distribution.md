---
id: ADR-009
type: adr
status: confirmed
updated: 2026-09-20
tags: [architecture, distribution, signing]
---

# ADR-009: Distribution & Signing

## Decision
NSIS installer, unsigned alpha, OV cert deferred.

## Context
Loom needs to be distributed as a Windows `.exe` installer. Code signing affects SmartScreen warnings and user trust.

## Options Evaluated

| Option | Score | Notes |
|--------|-------|-------|
| **NSIS Installer (unsigned)** | **4.30** | Free, built into Tauri, functional |
| MSI + Azure Signing | 3.85 | Better Windows integration but Azure signing may not be available in India |
| MSIX + Store | 3.40 | Store review process, additional constraints |
| Portable ZIP | 3.20 | No install process, no Start Menu integration |

## Evidence
- P0.1 build produced both NSIS and MSI: `Loom_0.1.0_x64-setup.exe` (2 MB), `Loom_0.1.0_x64_en-US.msi`
- Azure Artifact Signing: limited to US/CA/EU/UK organizations, US/CA individuals — UNVERIFIED for India
- OV certificate from Sectigo/Comodo: ~$100-200/yr — deferred per user decision (no spending)

## User Decision
Unsigned alpha with SmartScreen warnings documented. No spending on certificates.

## Counter-Argument
SmartScreen warnings will deter non-technical users from installing. Acceptable for alpha/MVP.

## Change My Mind If
User base grows beyond technical early adopters and SmartScreen becomes a blocker.

## Related
- [[ADR-001-framework]]
