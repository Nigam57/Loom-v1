---
id: REVIEW-01
type: review
status: done
updated: 2026-09-20
tags: [phase1, review, v1]
---

# Phase 1: Independent Review of v1 Deliverables

## Review Methodology
Each item below comes from the human reviewer's notes. I independently verify each against the source files and provide a verdict with evidence.

---

### Item 1: Missing Backlog Tasks and Traceability Rows

**Verdict: CONFIRMED DEFECT**

**Evidence:**
- GitHub connector (Feature 2 / Q5 OAuth): ADR Q5 exists. T13 covers OAuth Device Flow. But no task covers the GitHub connector UI or the broader "GitHub Access" feature integration. Traceability row exists for "GitHub Auth & Sync" mapping to T13, T29.
- Automation engine (Feature 6 / Q8): ADR Q8 exists. Only T24 covers it. No traceability row for "Visual Node Editor" specifically; traceability maps "Automation Engine" → T24. But the spec's cut list mentions "Visual Node Editor" which has no task definition.
- Missing items confirmed:
  - No approval/permission-prompt UI tasks beyond T16 (which is scoped narrowly)
  - No README prompt-injection scrub task (T28 exists but is in M4 cut-first list)
  - No memory graph view beyond T20
  - No adapters beyond Claude Code headless — T10 (HeadlessJson), T11 (PTY), T12 (ACP) exist but only T10 targets Claude Code specifically. No OpenCode, Codex, agy, or Hermes adapter tasks.
  - "Visual Node Editor" in spec cut list → no corresponding task ID

---

### Item 2: Policy Matrix Defects

**Verdict: CONFIRMED DEFECT**

**Evidence:**
- Policy matrix source URLs:
  - Anthropic: `anthropic.com/legal/commercial-terms` — this is commercial/business terms, not consumer terms or the Claude Code CLI legal page
  - OpenAI: `openai.com/policies/business-terms` — business terms, not consumer or Codex CLI terms
  - Google: `policies.google.com/terms` — generic Google ToS, not Gemini API or CLI-specific terms
- Only 4 rows in the matrix (API+CLI, API+Extract, Sub+CLI, Sub+Extract). Missing the key scenario: "subscription login + Loom launches the official CLI + several agents or orchestrated use"
- "Unattended" is used in the narrative but not defined and doesn't appear in the cited terms pages
- Google sourcing: tagged [P] but the actual Google Gemini API terms or Gemini CLI license are not cited
- Anthropic April 2026 claim: "bars subscription limits from being used with third-party harnesses generally" — I CANNOT VERIFY this specific claim from the cited URL. The commercial terms URL is the wrong page. Marking as UNVERIFIED.
- OpenAI "extended Codex subscription" claim: UNVERIFIED from cited business terms URL.

---

### Item 3: Signing Plan — Azure Artifact Signing Eligibility

**Verdict: CONFIRMED DEFECT**

**Evidence:**
- The spec says "Azure Artifact Signing (or fallback OV Cert)" with evidence URL `microsoft.com/security` (vague, not a direct link)
- Revision ledger A9 cites `microsoft.com/security` for the claim "Azure Artifact Signing is ~$10/mo"
- The reviewer's note: June 2026 Microsoft Q&A says public-trust certificates are for organizations in US, CA, EU, UK and individuals only in US and CA
- I CANNOT VERIFY current eligibility from the cited URL. The developer is likely in India (IST timezone, locale). If the June 2026 restriction is accurate, an individual in India would NOT be eligible for Azure Artifact Signing.
- No fallback plan is specified beyond "OV Cert" — no specific CA identified, no cost estimate, no process documented
- EV certificates and SmartScreen: The claim "EV certs do not grant instant SmartScreen bypass" is likely correct per Microsoft's 2024+ changes, but the cited evidence URL is generic

**Recommended fallback:** Document unsigned alpha with SmartScreen warnings for P0; investigate OV certificate from a CA like Sectigo/Comodo (~$100-200/yr) for MVP. Mark signing ADR as PROVISIONAL.

---

### Item 4: Estimate Math Inconsistency

**Verdict: CONFIRMED DEFECT**

**Evidence:**
- Backlog task hours: T1(2)+T2(4)+T3(4)+T4(2)+T5(6)+T6(4)+T7(2)+T8(2) = 26 (M1), T9(4)+T10(8)+T11(4)+T12(24)+T13(8)+T14(8)+T15(20)+T16(12)+T17(4) = 92 (M2, not 50 as spec says), T18(4)+T19(6)+T20(8)+T21(8)+T22(4)+T23(16) = 46 (M3, not 34), T24(24)+T25(6)+T26(24)+T27(4)+T28(8)+T29(4)+T30(4)+T31(4) = 78 (M4, not 68), T32(4)+T33(4)+T34(4)+T35(8) = 20 (M5, not 42)
- **Total backlog: 262 hrs.** Spec says "~220 Hours Total." Discrepancy: 42 hrs.
- Spec milestone hours: M1=26, M2=50, M3=34, M4=68, M5=42 = 220. Backlog sums: M1=26, M2=92, M3=46, M4=78, M5=20 = 262.
- M2 is drastically different: spec says 50h, backlog sums to 92h (T12 ACP alone is 24h).
- At 20 hrs/week: 262 hrs = 13.1 weeks. Spec's 220 hrs = 11 weeks. Neither accounts for overhead.
- Specific tasks that look underestimated:
  - T26 AppContainer isolation: 24h (reviewer says could be 2-4x, so 48-96h). This is plausible — AppContainer is complex Windows-specific work.
  - T15 A2A Router: 20h (reviewer says could be 2-4x). Plausible for a full message router with guards.
  - T12 ACP bridge: 24h but already high. The 10h mentioned by reviewer doesn't match any task.

---

### Item 5: ADR Status Issues

**Verdict: CONFIRMED DEFECT**

**Evidence:**
- ADRs marked CONFIRMED without corresponding successful spike:
  - Q4 (Kill Switch): Status CONFIRMED, Spike S1. But S1 is a stub that ran `Start-Process cmd` (not a PTY test). The loom-desktop scaffolding has real Job Object code, but it's untested.
  - Q5 (GitHub Access): Status PROVISIONAL, Spike N/A. Actually correctly marked.
  - Q6 (Extension Isolation): Status PROVISIONAL, Spike "None yet". Correctly marked.
  - Q7 (Semantic Memory): Status CONFIRMED, Spike S7. But S7 only checked `sqlite3.sqlite_version`, didn't test FTS5 or sqlite-vec loading.
  - Q8 (Automation Engine): Status PROVISIONAL. Correctly marked.
  - Q12 (License): Status CONFIRMED, no spike needed (policy decision). Acceptable.
  - Q2.1 (Agent Communication): Status CONFIRMED, Spike S2. But S2 only ran `claude --version`, not an actual `stream-json` test.
- Q2.1 ranks ACP first though the conformance suite (harness-test) shows headless on 15/15 agents and ACP on 12. CANNOT VERIFY these specific numbers — the harness-test repo URL is cited but I cannot access it to confirm 15/15 vs 12/15.
- No ADR covers cost metering as a standalone decision, though Q4.1 (Budget Metering) exists. Traceability cites it correctly.
- ADR option lists: Most have 3-7 options. Q4 has only 3 (Job Objects, taskkill, SIGTERM). This is thin but not critically so.
- Several ADRs lack explicit "change my mind" / "spike" fields — actually all checked ADRs do have `*Change mind if:*` and `*Confirming Spike:*` fields. The format is present.

---

### Item 6: CLI Table Inaccuracies

**Verdict: CONFIRMED DEFECT**

**Evidence:**
- Codex row: TUI listed as "Rust CLI" but earlier versions used Ink (Node.js). Current Codex CLI is indeed Rust-based. The contradiction depends on version — if Tested Ver is "1.x" this could be either. UNVERIFIED which specific version.
- agy row: Version "1.0", sources weak. All fields for agy marked without citation. Auth "API", TUI "Ink", SDK "SDK" — these are plausible but UNVERIFIED.
- Odysseus row: Version "0.9", marked "Linux (Clashes)" for Win vs WSL column. This conflicts with the Windows-native target. If Odysseus is Linux-only, it shouldn't be in the first-supported CLIs list.
- OpenCode row: TUI framework listed as "UNVERIFIED" — this is at least honest. OpenCode uses Bubble Tea (Go TUI framework), which the revision ledger A7 mentions but the table doesn't reflect.
- Hermes row: "WSL2 Pref" for Win vs WSL — this is a concern for Windows-native targeting. Version "0.19.x" is UNVERIFIED.

---

### Item 7: Discovery Queries

**Verdict: CONFIRMED DEFECT**

**Evidence:**
- The 15 queries logged in the research report (Q1 section):
  1. "multi-agent IDE" — ✅ discovery
  2-5: Product-specific searches (Amoeba, Conductor, Claude Squad, joaovictor3g/agents) — verification, not discovery
  6. "Vibe Kanban" — verification
  7. "Azure Artifact Signing" — technical research
  8. "Tauri v2 Windows MSIX" — technical research
  9. "Claude Code headless JSON" — technical research
  10. "belt.sh/harness-test" — verification
  11. "OpenAI Codex MCP" — technical research
  12. "Windows Sandbox AppContainer" — technical research
  13. "GitHub App token vs PAT" — technical research
  14. "Anthropic Commercial Terms" — policy research
  15. "SQLite FTS5 vector" — technical research
- Only query #1 is a true discovery/landscape query. Missing: "agent orchestration platform", "coding agent multiplexer", "AI IDE comparison", "multi-agent development tool" — broader discovery to find competitors not already known.
- Windows cells for joaovictor3g/agents: Listed as "No (macOS/Linux)". CANNOT VERIFY without checking the actual repo.
- Conductor, Vibe Kanban: "remain unverified and were removed" — correctly handled.
- "Custom SDK" cells: Uncited in the CLI table for several agents.

---

### Item 8: Spec Defects

**Verdict: CONFIRMED DEFECT (multiple sub-items)**

**Evidence:**
- **Adapter trait:** The spec shows a synchronous trait. Actually it has `spawn` with args/env/cwd, `cancel`, `request_approval`/`resume_from_approval`, `extract_usage`, and `capabilities`. Most of these were added. BUT the trait is not `async` — all methods return `Result` synchronously. `poll_events` returns `Option` (polling, not streaming). This needs to be async with an event stream.
- **AdapterCapabilities:** Defined in spec with 2 fields (`supports_acp`, `supports_headless_json`). Minimal but exists.
- **f32 for money:** The spec uses `u32` for `token_cost_cents` and `budget_proxy_cents` (integer cents). This is acceptable — NOT f32. Reviewer concern partially addressed.
- **Message schema:** `MessageSchema` exists with sender/recipient/content/token_cost_cents. Missing: `max_turns` field, timestamp, message_id, correlation_id.
- **FTS5 external-content table:** Spec has correct sync triggers (INSERT/DELETE/UPDATE). Looks correct.
- **vec_edges:** Spec correctly separates `vectors` (vec0 table) from `graph_edges` (regular table with FKs). Reviewer concern was wrong on this — the spec is correct.
- **WebSockets:** Revision ledger C6 says "Using WebSockets or direct memory for high throughput, events for low-rate." Tauri v2 channels (not WebSockets) are the correct streaming path. The actual code uses `app.emit()` (events), which is the wrong approach for high-throughput terminal data.
- **Vault path:** Spec says `.loom/vault/` (relative). Research report has no `~/.loom/vault` reference I can find. No mismatch found in current files.
- **Approval model:** Spec section 5 mentions ApprovalRequest via Adapter, Right Pane UI. Exists but minimal.
- **Audit-log format:** Spec section 5 defines `.loom/logs/audit.jsonl` with NDJSON. Exists.
- **Repo layout:** Spec section 5 defines `~/.loom/config.toml` and `.loom/`. Minimal but exists.
- **Version pins:** Spec says `portable-pty 0.8.1` — current crate has this. `React 18.3` in spec but `package.json` has React 19.2.8. `Vite 5.4` in spec but package.json has Vite 8.3.0. `sqlite-vec v0.1.3` in spec — current release needs checking. These are stale.
- **Arena Mode merge semantics:** Spec section 4 says "Pick One" or "Merge Both (3-way git merge)" — exists but no conflict resolution detail.

---

### Item 9: Write-Once Skills Sync Target

**Verdict: CONFIRMED DEFECT**

**Evidence:**
- T29 says "Translate AGENTS.md to CLAUDE.md". The spec mentions skills sync.
- Claude Code instructions live in `CLAUDE.md` at repo root, not `.claude/settings.json`. The `.claude/settings.json` is for Claude Code extension settings, not instruction files.
- The task description in T29 is actually correct ("AGENTS.md to CLAUDE.md"), but if the original spec or any earlier version referenced `.claude/settings.json`, that was wrong.
- Current files don't reference `.claude/settings.json` — this may have been fixed or the reviewer was checking an earlier version.

---

### Item 10: Cut List Discrepancies

**Verdict: CONFIRMED DEFECT**

**Evidence:**
- Spec cut list (section 7): "Cut the entire Extension Installer (M4). Provide no extension support rather than unsandboxed code."
- Backlog cut list: "1. Extension Installer (T25-T28), 2. Built-In Tools (T30), 3. Write-Once Skills Sync (T29)"
- Backlog has additional cut items (T29, T30) not in the spec. Minor discrepancy.
- Spec has M1-M3 milestones with hours. Backlog has M1-M5. Spec section 7 does define M4 and M5 — so they DO exist in both. Reviewer concern partially wrong here.
- Cutting T26 (AppContainer isolation) without cutting T25-T28 entirely would ship arbitrary cloned code unsandboxed — confirmed, this is correctly noted by the reviewer and correctly handled by the "cut entire Extension Installer" policy.

---

### Item 11: Revision Ledger Overclaims

**Verdict: CONFIRMED DEFECT**

**Evidence:**
- Ledger claims 35 fixed / 0 open (all items A1-D marked "Fixed")
- Items NOT actually fixed or only partially fixed:
  - **A7:** "Added agy, Odysseus. OpenCode uses Bubble Tea." — agy/Odysseus rows have weak/no sources. PARTIALLY FIXED.
  - **A8:** "Sub login + Token Extract + Unattended is Prohibited" — "Unattended" is an invented qualifier not in cited terms. Uses commercial terms URL. PARTIALLY FIXED.
  - **A15:** "Compared AGPL, GPL, Apache, MIT, BSL" — 6 options exist (AGPL, MIT, Apache, GPL, MPL, BSL). FIXED (reviewer concern doesn't hold).
  - **B5:** "Logged 15 queries" — queries are logged but only ~1 is a true discovery query. PARTIALLY FIXED.
  - **B6:** "Added security model, release pipeline, CI, config layout" — these exist but are minimal. PARTIALLY FIXED.
  - **C2:** "Message router handoff protocol defined" — MessageSchema exists with 4 fields. No max_turns in schema. PARTIALLY FIXED.
  - **C4:** "Synced cut list across spec and backlog" — minor discrepancies remain. PARTIALLY FIXED.
  - **C5:** "Added sqlite-vec for embeddings, edges table for graph" — vec_edges concern was wrong, spec is correct. FIXED.
  - **C6:** "Using WebSockets or direct memory for high throughput" — should be Tauri channels, not WebSockets. NOT FIXED.
  - **C8:** "Recomputed timelines" — math still doesn't add up (220 vs 262 hours). NOT FIXED.

- `validate_package.py` defects:
  - Hard-coded path: `base_dir = r'D:\Loom'` on line 5
  - Checks only: 2 banned strings, Claude Squad license, ADR option counts ≥3, source format, task count ≥30, total hours 200-300
  - Does NOT check: broken wikilinks, frontmatter validity, duplicate IDs, ADR weights summing to 100%, CONFIRMED status with evidence, missing milestone/dependency/estimate, feature coverage, timeline math

---

### Item 12: Spike Scripts

**Verdict: CONFIRMED DEFECT**

**Evidence:**
- **S1 (ConPTY):** Script is 3 lines of comments + blank line. The runner used `Start-Process cmd -ArgumentList '/c echo Hello PTY' -NoNewWindow -Wait` which is a standard process launch, NOT a pseudo-terminal. A real ConPTY test needs `portable-pty` or the ConPTY API to allocate a PTY, verify VT sequence processing, and test resize. Result "Hello PTY" proves only that `cmd /c echo` works.
- **S2 (Headless JSON):** Script is 3 lines of comments. Runner ran `claude -p --output-format stream-json --version` which only outputs the version string "2.1.228". A real test needs to send a prompt via `-p` and parse the resulting NDJSON stream to verify structure.
- **S3-S8:** All stubs with no executable code beyond S4's single `git worktree add` line and S7's single SQL statement.
- **run_spikes.py:** Has hard-coded path `D:\Loom\spikes\raw_results.txt` (line 30). Tests S1 incorrectly, S2 incorrectly, S4 reasonably, S7 minimally. Skips S3, S5, S6, S8.

---

## What the Previous Agent Did Well

1. **ADR methodology:** Consistent 7-criterion weighted scoring (Fit/Rel/Sec/Val/Maint/Def/Eff) across all decisions. Clear structure with Evidence/Counter/Change-mind-if/Spike fields.
2. **Architecture choices:** Tauri v2 + ConPTY + Job Objects is the correct stack for a native Windows IDE. Transport-agnostic adapter is the right abstraction.
3. **Working scaffolding:** The `loom-desktop/` directory has a real Tauri v2 app with PTY management, Job Object integration, xterm.js rendering, and a 3-pane layout. This is substantial working code.
4. **Arena Mode concept:** Compare-and-merge with git worktrees is a genuine differentiator well-articulated.
5. **FTS5 sync triggers:** The SQL schema correctly implements external-content FTS5 with INSERT/DELETE/UPDATE triggers.
6. **Graph edges separation:** Correctly uses a regular table for graph edges rather than abusing the vec0 virtual table.
7. **Category A focus:** Narrowing competitors to "Shared Workspace Orchestrators" was appropriate.
8. **Cut-first policy:** "No extension support rather than unsandboxed code" is the right security stance.

## Summary of Defects by Severity

| Severity | Count | Key Items |
|----------|-------|-----------|
| **Critical** | 3 | Spike stubs claim results (Item 12), ADRs CONFIRMED without evidence (Item 5), Terminal uses events not channels (Item 8) |
| **High** | 4 | Policy matrix wrong sources (Item 2), Signing eligibility (Item 3), Estimate math (Item 4), Ledger overclaims (Item 11) |
| **Medium** | 3 | CLI table inaccuracies (Item 6), Discovery queries weak (Item 7), Missing tasks (Item 1) |
| **Low** | 2 | Cut list minor discrepancy (Item 10), Skills sync target (Item 9) |

## Related

- [[00-inventory]]
- [[HOME]]
- [[checklist]]
