# Task 1 Report

## Status
DONE

## Changes Made
1. **Dependencies**: Added `axum`, `futures` for the internal PTY server, and `reqwest` for integration testing.
2. **Server Implementation**: Created `src/pty/server.rs` which implements a lightweight `axum` HTTP server.
   - `POST /pty/:pid/write`: takes a JSON payload with `{ data: "..." }` and calls `manager.write_pty()`.
   - `GET /pty/:pid/events`: returns an SSE (Server-Sent Events) stream of PTY output/exit events by subscribing to a `tokio::sync::broadcast` channel.
3. **PTY State & Broadcasts**: Modified `src/pty/commands.rs` to initialize and inject a `broadcaster` (a thread-safe hashmap of channels) into `PtyState`. `spawn_pty` now creates a channel for the spawned PTY and broadcasts `PtyEvent` to any subscribers.
4. **App Initialization**: Updated `src/lib.rs` to instantiate the `broadcaster` and launch the `axum` server dynamically via `tokio::spawn` in Tauri's `setup` block (currently runs on port 3030).
5. **Integration Test**: Wrote an integration test in `tests/pty_api_test.rs` which spawns the API server on a random port and makes real HTTP requests via `reqwest` to verify the routes are available and behaving correctly.

## Concerns / Notes
- The server currently starts on a fixed port (3030) in `lib.rs` for the main application, but can bind dynamically. We may want to pick a dynamic OS port and export it via a `.port` file or env var depending on how the node sidecar discovers it.
- To prevent locking issues, the thread spawned in `commands.rs` ignores channel send failures and continues reading as long as there are subscribers.
- Due to the Windows DLL bug, all testing goes through `tests/pty_api_test.rs`.
