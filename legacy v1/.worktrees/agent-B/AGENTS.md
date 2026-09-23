# Loom Agent Rules & Guidelines (AGENTS.md)

Welcome to the Loom codebase. If you are an AI coding agent (Claude, OpenCode, Antigravity, Hermes), you must abide by these rules before executing any code changes.

## 1. Skill Requirements
You **MUST** use the `superpowers` skills before taking any action. Do not go raw.
- Before beginning a new feature, run `superpowers:brainstorming`.
- Before executing a plan, run `superpowers:writing-plans`.
- Before committing, run `superpowers:verification-before-completion`.

## 2. Tech Stack Boundaries
- **Backend (Tauri):** We strictly use Rust via Tauri. Do not introduce Node.js or Python dependencies directly into the core `src-tauri` backend build.
- **Extensions (Exception):** External LLM extensions/tools may use Python or Node.js. These are executed *only* within the isolated Windows AppContainer environment and are never imported natively into the Rust backend.
- **Process Management:** All CLI spawning must go through `src-tauri/src/pty/manager.rs` or the transport adapter `src-tauri/src/adapter/`. Do not use standard `std::process::Command` without binding it to a Windows Job Object.
- **Frontend:** React + Vite. Terminal interactions must pipe via IPC channels into `@xterm/xterm`.

## 3. Red-Team Verification
If you claim a bug is fixed or a feature is complete, you must prove it by running the exact verification commands:
- **Rust Backend Tests:** Run `cargo test --manifest-path src-tauri/Cargo.toml` and paste output showing 0 failures.
- **Frontend Linter:** Run `npm run lint` and show 0 errors.
- **Builds:** Run `npm run tauri build` and confirm it exits with 0.
- Do not say "it should work now." Verify it with the command output.

## 4. Worktree Awareness
If you are operating inside `.loom-trees/`, you are in an isolated Arena Mode worktree. Do not commit directly to `main`. Produce your diffs, and the Loom orchestrator will handle the merge conflict resolution.

## 5. Architectural Integrity
- Do not implement anything that depends on a **PROVISIONAL** ADR until its respective spike passes and the status is changed to CONFIRMED.

## 6. Documentation Integrity & Continuous Doc Sync
- **Update Docs After Every Pass:** After making code changes and verifying everything with tests/builds, you **MUST** update all corresponding documentation (specifications, roadmap checklist, ADR statuses, and evidence) before concluding your turn or committing.
- **Inherited / Stale Docs Remediation:** If you take over a session or find that a previous agent failed to update the documentation or left docs out of sync with the actual code, you **MUST first verify the actual codebase state and update the docs** before proceeding with any new feature development.
- **Vault Validation:** Whenever documentation in `docs/` is modified, run `python tools/validate_vault.py --vault-root docs` and ensure 0 errors before marking work complete.
- **No Fabricated / Mock Data in Docs:** All CLI commands, versions, and API signatures in docs must match verified implementations or real CLI behaviors. Tag unverified items with `UNVERIFIED`.
