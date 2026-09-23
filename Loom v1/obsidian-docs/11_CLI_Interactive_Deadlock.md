---
title: CLI Interactive Deadlock Analysis
tags: [architecture, chat, terminal, deadlock, authentication]
---

# 🛑 CLI Interactive Deadlock Analysis

## The Symptoms
You noticed that even though the Chat-to-CLI bridge is repaired and the CLIs are correctly invoked, you still cannot successfully "chat" with them—especially when they require authentication or interactive setup (like asking for a project name). 

When you send a message, it seems to go into a black hole.

## The Root Cause: The Semantic Batcher Deadlock

The bug lies in a fundamental conflict between **Interactive CLIs** and the **Semantic Chat Batcher** (`chat_dispatch_batcher.rs`).

Here is the exact timeline of the deadlock:
1. **The Auth Prompt:** The CLI (e.g., `agy`) launches and immediately prints an interactive prompt (e.g., `"Please enter your authentication token: "`).
2. **The User Responds:** You see this in the UI, so you type your token into the Chat Input and press Send.
3. **The First Dispatch:** The `ChatDispatchBatcher` receives your token. Because the queue is empty, it marks the token as `inflight` and successfully pipes it into the CLI's standard input.
4. **The Deadlock Trap:** The batcher is strictly designed for "Multiplayer AI Agent" interactions. After sending a message, it **locks its queue** and refuses to send any more messages until the `semantic_worker` detects the AI's final "I am done" prompt marker (e.g., `❯`). 
5. **The Freeze:** Because the CLI just accepted your auth token, it might now print `"Select a project: [1, 2, 3]"`. It does **not** print `❯` because it's still in an interactive setup wizard, not a standard chat loop.
6. **The Black Hole:** Because `❯` was never printed, the batcher's `inflight` lock is never released. When you try to send "1" to select your project, the batcher puts "1" into a pending queue and refuses to send it to the terminal. You are locked out.

## How to Fix It (The Plan)

To fix this, we need to bypass the `ChatDispatchBatcher` lock when the terminal is in an "interactive/raw" state, or we need to allow the UI to send raw keystrokes directly to the PTY bypassing the semantic batcher entirely.

### Option A: The "Raw Input" Mode (Recommended)
If the terminal is waiting for standard interactive input (like a password or selection), we should allow the user to type directly into the terminal emulator UI rather than forcing them to use the Chat Input box. 
Currently, the Chat UI intercepts all inputs. We can add a toggle or detect if the terminal hasn't hit its first `❯` yet, allowing raw terminal interaction until the setup is complete.

### Option B: The "Timeout Flush" 
If the `ChatDispatchBatcher` sends a message and doesn't receive a `❯` within a certain timeout (e.g., 2000ms), but it *does* detect terminal output (like "Select a project:"), it automatically drops the `inflight` lock and assumes the CLI is in a raw interactive loop, allowing subsequent chat messages to flow through.

*Let me know if you approve this analysis, and which option you prefer to implement!*
