# Sidecar Integration Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Integrate `claw-orchestrator` as a Node sidecar where Loom's Rust layer natively owns process spawning and Job Object cleanup, while delegating council orchestration and ledger management to the sidecar.

**Architecture:** 
Loom (Rust) currently manages processes via `pty/manager.rs` with robust Windows Job Object cleanup. `claw-orchestrator` normally spawns its own processes via standard pipes (losing PTY and leaking on Windows). To achieve Option 4, we will invert control: 
1. Loom exposes an internal HTTP/WebSocket endpoint for active PTY sessions.
2. We programmatically wrap `claw-orchestrator` in a small Node sidecar (`sidecar/index.js`) that defines a custom `ISession` engine wrapper.
3. When the sidecar needs to orchestrate a session, it connects to Loom's internal API to drive the actual PTY process rather than spawning its own `child_process`.
4. Loom boots this Node sidecar natively in a Job Object at startup.

**Tech Stack:** Rust (Tauri), Node.js, `claw-orchestrator` SDK, WebSockets/HTTP.

**Spec:** [[ADR-013-coordination-backend]]

## Global Constraints
- Node sidecar must be bundled or run transparently.
- Rust must remain the strict owner of all `spawn` events and Windows Job Objects.
- No modifications to `claw-orchestrator` global installs; we use it programmatically as a module.

---

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

### Task 2: Node Sidecar Scaffold & Orchestrator Boot

**Files:**
- Create: `loom-desktop/sidecar/package.json`
- Create: `loom-desktop/sidecar/index.js`
- Modify: `loom-desktop/src-tauri/tauri.conf.json`

**Interfaces:**
- Consumes: The `claw-orchestrator` NPM package.

- [ ] **Step 1: Initialize sidecar project**
```bash
mkdir loom-desktop/sidecar
cd loom-desktop/sidecar
npm init -y
npm install @enderfga/claw-orchestrator express ws
```

- [ ] **Step 2: Write Sidecar HTTP server**
Implement `index.js` to expose an HTTP server that programmatically initializes `SessionManager` from `claw-orchestrator`.
```javascript
const { SessionManager } = require('@enderfga/claw-orchestrator');
const express = require('express');
const app = express();
// ... setup manager and route /start-council
```

- [ ] **Step 3: Wire Tauri Sidecar config**
Update `tauri.conf.json` to register the sidecar binary so Tauri spawns it in the same Job Object on boot.

- [ ] **Step 4: Commit**
```bash
git add loom-desktop/sidecar/ loom-desktop/src-tauri/tauri.conf.json
git commit -m "feat: setup claw-orchestrator node sidecar"
```

### Task 3: Custom Loom Engine Adapter in Sidecar

**Files:**
- Create: `loom-desktop/sidecar/loom-engine.js`
- Modify: `loom-desktop/sidecar/index.js`

**Interfaces:**
- Consumes: Loom's Rust internal PTY API (Task 1).
- Produces: An `ISession` compatible class for `claw-orchestrator`.

- [ ] **Step 1: Write Loom Engine class**
Implement a custom engine class that implements `ISession` (or extends `BaseOneshotSession`) from `claw-orchestrator`. Instead of `child_process.spawn()`, it makes a request to `http://127.0.0.1:<rust_port>/pty/spawn` and wires the WebSocket stream to the orchestrator's `stdout` parser.

- [ ] **Step 2: Register Engine with Orchestrator**
```javascript
manager.registerEngine('loom', LoomEngine);
```

- [ ] **Step 3: Integration Test**
Write a quick Node test script to hit the sidecar API, triggering a council run, and verify the sidecar hits the Rust API to spawn the PTYs.

- [ ] **Step 4: Commit**
```bash
git add loom-desktop/sidecar/loom-engine.js loom-desktop/sidecar/index.js
git commit -m "feat: implement loom engine adapter to route spawn to rust"
```

### Task 4: Connect Loom Frontend to Sidecar

**Files:**
- Modify: `loom-desktop/src/components/ConversationFlow.tsx` (or similar)
- Modify: `loom-desktop/src/api.ts`

**Interfaces:**
- Consumes: Node Sidecar HTTP API.

- [ ] **Step 1: UI Hook**
Update the frontend to dispatch "Start Council" or "Send Message" actions to the Node Sidecar's port instead of directly to Rust. 

- [ ] **Step 2: Event Stream wiring**
Connect the frontend's EventSource to the sidecar's `/council/:id/events` endpoint to stream the orchestration live updates directly to the UI.

- [ ] **Step 3: Verification**
Run `npm run dev` and `cargo tauri dev`, click "Start Council", and verify the Rust console logs the PTY spawns while the frontend renders the Sidecar's events.

- [ ] **Step 4: Commit**
```bash
git add loom-desktop/src/
git commit -m "feat: route UI orchestration to sidecar"
```
