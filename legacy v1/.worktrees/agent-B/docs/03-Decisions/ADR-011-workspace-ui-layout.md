---
id: ADR-011
type: adr
status: provisional
updated: 2026-09-21
tags: [architecture, ui, layout, tools]
features: [5]
confirming_spike: null
---

# ADR-011: Workspace UI Layout & Built-in Tool Registry

This ADR covers two related but separable decisions:
- **Part A:** Dockable panel layout system for the multi-pane IDE
- **Part B:** Built-in tool registry and delivery model (Scrapling, diff viewer, trace viewer)

---

## Part A: Workspace Layout System

### Decision
Adopt **Dockview** (`dockview-react` v8.x) as the panel layout engine. It provides IDE-grade docking, tabs, groups, drag-and-drop, serialization, and zero external dependencies.

### Context
Loom's UI needs a flexible layout system supporting multiple terminal panes, tool panels, and agent views simultaneously. Users must split, join, resize, and rearrange panels freely. Tauri v2 supports both single-window WebView layouts and multiple windows — the layout library choice should work in either mode.

### Evaluation Criteria & Weights

| Criterion | Weight | Description |
|-----------|--------|-------------|
| Fit | 25% | How well does it match Loom's IDE use case? |
| Reliability | 20% | Maintenance track record, bug count, community |
| Security | 15% | Dependency surface, XSS vectors, data handling |
| Value | 15% | Time to integrate vs. building from scratch |
| Maintainability | 10% | API surface area, upgrade path, TypeScript support |
| Defensibility | 10% | Lock-in risk, license terms, fork-ability |
| Efficiency | 5% | Runtime performance, bundle size, memory |

### Options Evaluated

| Criterion (Weight) | Dockview v8.x | FlexLayout v0.10.8 | react-resizable-panels v4.12.4 | Custom (React) |
|---------------------|---------------|---------------------|-------------------------------|----------------|
| Fit (25%) | 5 | 4 | 3 | 5 |
| Reliability (20%) | 4 | 4 | 5 | 2 |
| Security (15%) | 4 | 3 | 5 | 5 |
| Value (15%) | 5 | 4 | 3 | 1 |
| Maintainability (10%) | 4 | 3 | 4 | 3 |
| Defensibility (10%) | 3 | 4 | 5 | 5 |
| Efficiency (5%) | 4 | 3 | 5 | 4 |
| **Weighted Total** | **4.30** | **3.70** | **3.95** | **3.30** |

Weighted total calculation (Dockview):
5×0.25 + 4×0.20 + 4×0.15 + 5×0.15 + 4×0.10 + 3×0.10 + 4×0.05
= 1.25 + 0.80 + 0.60 + 0.75 + 0.40 + 0.30 + 0.20 = **4.30**

Weighted total (FlexLayout):
4×0.25 + 4×0.20 + 3×0.15 + 4×0.15 + 3×0.10 + 4×0.10 + 3×0.05
= 1.00 + 0.80 + 0.45 + 0.60 + 0.30 + 0.40 + 0.15 = **3.70**

Weighted total (react-resizable-panels):
3×0.25 + 5×0.20 + 5×0.15 + 3×0.15 + 4×0.10 + 5×0.10 + 5×0.05
= 0.75 + 1.00 + 0.75 + 0.45 + 0.40 + 0.50 + 0.25 = **4.10** (corrected)

Weighted total (Custom):
5×0.25 + 2×0.20 + 5×0.15 + 1×0.15 + 3×0.10 + 5×0.10 + 4×0.05
= 1.25 + 0.40 + 0.75 + 0.15 + 0.30 + 0.50 + 0.20 = **3.55** (corrected)

**Corrected ranking: Dockview (4.30) > react-resizable-panels (4.10) > FlexLayout (3.70) > Custom (3.55)**

### Evidence
- Dockview v8.x: zero-dependency, IDE-grade docking (tabs, groups, floating, popout), React bindings, serialization/restore [P] https://dockview.dev (2026-09-21)
- Dockview npm: `dockview-react` — free tier available, enterprise tier for advanced features [P] https://www.npmjs.com/package/dockview-react (2026-09-21)
- FlexLayout v0.10.8: multi-tab docking, sub-layouts, border tabsets, popout windows, actively maintained [P] https://github.com/caplin/FlexLayout (2026-09-21)
- react-resizable-panels v4.12.4: pixel/percentage units, SSR, shadcn/ui backing, accessible drag handles [P] https://www.npmjs.com/package/react-resizable-panels (2026-09-21)
- Tauri v2 supports multiple windows and WebView instances [P] https://v2.tauri.app/guides/window-customization/ (2026-09-21)

### Status: PROVISIONAL
No spike. A UI layout spike is needed to validate Dockview integration with xterm.js terminal panes inside Tauri. Consider whether react-resizable-panels (simpler, higher defensibility score) is sufficient for the MVP.

### Risk
- Dockview v8.x introduced an enterprise tier — the free tier must be verified to include all features Loom needs (tabs, groups, serialization, drag-drop).
- If Dockview's enterprise tier is required for critical features, react-resizable-panels is the fallback.

### Counter-Argument
react-resizable-panels scores nearly as high (4.10 vs 4.30) with better defensibility and security. It lacks full IDE docking (no tab groups, no floating panels) but may be sufficient for MVP.

### Change My Mind If
- react-resizable-panels adds tab grouping, or
- Dockview's free tier removes critical features Loom needs, or
- A layout spike shows Dockview integration is significantly harder than expected

---

## Part B: Built-in Tool Registry

### Decision
Opt-in tool registry with approval prompts. Tools are delivered as MCP servers where possible. Scrapling (BSD-3) is a candidate built-in but carries legal/ToS risk from anti-bot-bypass and must be opt-in.

### Scrapling Assessment

| Property | Value |
|----------|-------|
| License | BSD 3-Clause [P] https://github.com/D4Vinci/Scrapling/blob/main/LICENSE (2026-09-21) |
| Delivery | Python runtime dependency, ships MCP server interface + CLI |
| AGPL compatibility | BSD-3 + AGPL = compatible for distribution |
| ToS risk | **HIGH**: Anti-bot-bypass fetchers (Cloudflare Turnstile bypass) may violate target site Terms of Service. This is a legal liability risk regardless of open-source license. |
| Recommendation | Ship as opt-in extension, not bundled. Subject to approval prompts. User assumes ToS compliance responsibility. |

### Tool Registry Design
- Tools are registered in `~/.loom/tools.toml`
- Each tool has: name, type (mcp-server | builtin | extension), command, args, env, approval_required (default: true)
- Built-in tools must pass approval prompts before first use
- The registry exposes tools to connected agents via MCP tool listing

### Related
- [[ADR-001-framework]]
- [[spec-workspace-ui]]
- [[T-063]]
- [[T-065]]
- [[T-066]]
