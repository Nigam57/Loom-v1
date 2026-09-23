### Task 1: Rust Internal PTY API (Tauri Side)

**Files:**
- Modify: `loom-desktop/src-tauri/src/pty/manager.rs`
- Modify: `loom-desktop/src-tauri/src/main.rs` (or where local server/IPC is configured)

**Interfaces:**
- Produces: Local socket/HTTP endpoint in Tauri (e.g., `http://127.0.0.1:<port>/pty/:id`) to send/receive streams from existing PTYs.

- [ ] **Step 1: Write failing test for PTY IPC route**
```rust
// tests/pty_api_test.rs or inline
#[tokio::test]
async fn test_pty_bridge_api() {
    // Assert HTTP server boots and can interact with PTY manager
}
```

- [ ] **Step 2: Run test to verify it fails**
Run: `cargo test` in `loom-desktop/src-tauri`
Expected: FAIL

- [ ] **Step 3: Implement internal API**
Add a lightweight local web server (using `axum` or `warp` or Tauri's IPC event bus if bridged to the sidecar) that maps to `PtyManager::write_to_pty` and broadcasts `PtyManager` output events.

- [ ] **Step 4: Verify test passes**
Run: `cargo test`
Expected: PASS

- [ ] **Step 5: Commit**
```bash
git add loom-desktop/src-tauri/src/pty/manager.rs
git commit -m "feat: expose local PTY bridge API for sidecar"
```

