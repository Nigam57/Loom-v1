# Loom Agent Rules & Guidelines (AGENTS.md)

Welcome to the Loom codebase. If you are an AI coding agent (Claude, OpenCode, Antigravity, Hermes), you must abide by these rules before executing any code changes.

## 1. Skill Requirements
You **MUST** use the `superpowers` skills before taking any action. Do not go raw.
- Before beginning a new feature, run `superpowers:brainstorming`.
- Before executing a plan, run `superpowers:writing-plans`.
- Before committing, run `superpowers:verification-before-completion`.

## 2. Tech Stack Boundaries
- **Backend:** We strictly use Rust via Tauri. Do not introduce Node.js or Python dependencies on the backend.
- **Process Management:** All CLI spawning must go through `src/pty/manager.rs`. Do not use standard `Command::new()` without binding it to a Windows Job Object.
- **Frontend:** React + Vite. All terminal interactions must pipe through the `useTerminal` hook into `xterm.js`. Do not manipulate the DOM for the terminal directly.

## 3. Red-Team Verification
If you claim a bug is fixed or a feature is complete, you must prove it.
- **Tests:** Run `cargo test` and paste the output showing 0 failures.
- **Builds:** Run `npm run tauri build` and confirm it exits with 0.
- Do not say "it should work now." Verify it.

## 4. Worktree Awareness
If you are operating inside `.loom-trees/`, you are in an isolated Arena Mode worktree. Do not commit directly to `main`. Produce your diffs, and the Loom orchestrator will handle the merge conflict resolution.
