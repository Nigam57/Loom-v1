# Loom v1

Loom is a local-first, multi-agent workspace application built with Vue 3 and Tauri. It transforms your CLI tools into a unified AI collaboration hub. 

## Architecture

Loom v1 is architected as a robust desktop application integrating a rich frontend with a high-performance Rust backend.

### Core Systems
Based on our codebase analysis:

1. **Terminal & PTY Management (`pty.rs`, `TerminalManager`)**
   - Built on robust PTY session management to handle multiple concurrent terminal environments.
   - Core abstractions like `TerminalSession` and `CommandCenter` orchestrate interactions seamlessly.

2. **Chat & Agent Dispatch (`ChatDbManager`, `orchestrate_chat_dispatch`)**
   - Agents and human users interact through a unified chat interface (`ChatInterface.vue`).
   - SQLite-backed storage (`chat_db`) ensures complete local persistence of conversations and artifacts.

3. **Diagnostics & Telemetry (`DiagnosticsState`)**
   - Advanced state tracking and diagnostic logging bridges the frontend and backend efficiently.

4. **Notifications & State (`NotificationBadgeState`, `Settings.vue`)**
   - Real-time notification systems and comprehensive workspace metadata management.

## Tech Stack
- **Frontend**: Vue 3, Pinia, TailwindCSS (Dark/Light parity, OKLCH token ramp)
- **Backend**: Rust (cargo-tauri), SQLite
- **Terminal Engine**: xterm.js coupled with custom PTY spawning

## Local Development

**Prerequisites:** Node.js, pnpm, Rust, and cargo-tauri.

1. Install dependencies: `pnpm install`
2. Start Web development server: `pnpm dev`
3. Start Desktop development: `cargo tauri dev`

## Contribution
Before contributing, ensure that all UI changes respect the unified token-driven design system (`Geist` fonts, no arbitrary values, strict dark/light parity). Run `pnpm lint` and `pnpm test` prior to submitting pull requests.

## License
This project follows the [Business Source License 1.1 (BSL 1.1)](https://mariadb.com/bsl11/) open-source license.
