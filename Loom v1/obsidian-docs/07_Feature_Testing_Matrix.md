---
title: Feature Testing Matrix
tags: [qa, testing, features, bug-report, root-cause]
---

# 🧪 Feature Testing Matrix

A systematic testing report identifying what works and the root cause of what is broken.

## ✅ Functional Layer (What Works)

| Feature | Status | Root Cause / Validation |
| :--- | :--- | :--- |
| **Workspace Selection** | 🟢 PASS | Tauri's `plugin-dialog` successfully opens OS windows and binds the path to `.loom/workspace.json`. |
| **Terminal Spawning** | 🟢 PASS | `pty.rs` successfully spawns `portable-pty` subprocesses and attaches them to `TerminalPane.vue`. |
| **Terminal Rendering** | 🟢 PASS | `WeztermEmulator` successfully parses ANSI streams into `SnapshotMetrics` for the UI. |
| **Local Database** | 🟢 PASS | `rusqlite` successfully commits chat history. |
| **Theme Toggling** | 🟢 PASS | `theme.ts` correctly applies CSS variable overrides instantly. |

## ❌ Disruption Layer (What is Broken)

| Feature | Status | Root Cause |
| :--- | :--- | :--- |
| **Chatting with AI** | 🟢 PASS | **Bridge Reconnected:** The chat dispatch system was failing because it was silently dropping messages sent in 'channels' without explicit `@mentions`, and legacy dead code (`orchestrate_dispatch_impl`) was confusing the architecture. I deleted the dead code and updated the target resolution logic to intelligently auto-target the AI if it is the only other member in the channel. |
| **Skill/Plugin Execution** | 🔴 FAIL | **Missing Implementation:** The UI symlinks files to the hard drive, but no backend code reads from `.loom/skills`. |
| **`@` Mentions in Chat** | 🟢 PASS | Now working alongside the restored Chat Dispatch pipeline. |
| **Terminal Grid Splitting** | 🟡 WARN | **Race Condition:** Dropping a terminal tab to split the pane occasionally fails if the Vue DOM hasn't fully mounted the target drop zone (`onPointerUp` event resolves too early). |
