---
title: Features and UI Reference
tags: [features, buttons, interface, reference]
---

# 🎮 Features & UI Reference

This document catalogs every major feature, button, and interaction point in the Loom v1 application based on the AST structure.

## 1. Application Shell & Window Management (`App.vue`)
- **Main Viewport:** The app launches into a single OS window.
- **Titlebar:** Custom frameless titlebar with Vue-driven close/minimize buttons on Windows, native on macOS.
- **Global Keybinds (`useAppKeybinds.ts`):** Handles `Ctrl+K` for command palette, `Ctrl+T` for new terminal, etc.

## 2. Workspace Selection (`WorkspaceSelection.vue`)
When the app opens, it checks for a saved `__LOOM_WORKSPACE__`. If none is found, it shows the Workspace Selection screen.
- **"Open Workspace" Button:** Opens a native OS dialog (`@tauri-apps/plugin-dialog`) to select a directory.
- **Recent Workspaces List:** Shows a list of previously opened folders. Clicking one binds the App to that directory.
- **Search Bar:** Filters the recent workspace list.

## 3. Sidebar Navigation (`SidebarNav.vue`)
Located on the far left of the screen.
- **Avatar Profile (Top):** Clicking this opens a dropdown to switch Avatars (`toggleAvatarMenu()`) or edit the `AccountSettings`.
- **"Chats" Tab:** Navigates to the `ChatInterface`.
- **"Terminals" Tab:** Navigates to the raw `TerminalWorkspace`.
- **"Skills" Tab:** Navigates to `SkillStore.vue`.
- **Settings Gear (Bottom):** Opens the `Settings.vue` modal.

## 4. The Chat Interface (`ChatInterface.vue` & `ChatSidebar.vue`)
Designed to look like a direct messaging app.
- **Conversation List (Left Pane):** Shows all active "Friends" (Agents). 
- **"Invite Assistant" Button (`InviteAssistantModal.vue`):** Spawns a modal to add a new CLI AI to the workspace. Allows you to set its instances, role, and permissions.
- **"Rename Chat" (`RenameConversationModal.vue`):** Right-click a chat to rename the group or agent.
- **Message List (`MessagesList.vue`):** The center pane. 
  - **"Jump to Latest" Button:** Appears when scrolling up, auto-scrolls to the bottom.
  - **Avatar Click:** Clicking an agent's avatar in the chat opens their specific Terminal Pane configuration.
- **Chat Input (`ChatInput.vue`):**
  - **Textarea:** For typing prompts.
  - **`@` Mentions:** Typing `@` opens a popup to mention specific agents or inject skills.
  - **Emoji Button:** Opens the `closeEmojiPanel()` / `openEmojiPanel()` grid to insert standard emojis.
  - **Send Button / Enter:** Triggers `emitSend()` which pushes the prompt into the `chat_outbox_enqueue` in Rust.

## 5. Terminal Workspace (`TerminalWorkspace.vue` & `TerminalPane.vue`)
The hardcore developer view where actual CLI streams are visible.
- **Terminal Grid:** A drag-and-drop grid. You can drag a tab to split the screen horizontally or vertically (`onPointerUp`, `resolvePaneDropTarget()`).
- **Terminal Tabs:** 
  - **"Close" (X) Button:** Sends a SIGKILL to the underlying PTY and cleans up the UI state.
  - **Context Menu (`context-menu/controller.ts`):** Right-clicking a tab allows "Close Others", "Close All", or "Rename".
- **Find Widget (`refreshFindResults`):** `Ctrl+F` inside a terminal pane opens a search bar to grep text in the active `xterm.js` canvas.
- **Copy/Paste:** Fully supported via `attachClipboardHandlers()` and `clipboard-manager` Tauri plugin.

## 6. Skills Management (`SkillStore.vue`)
- **"Add Skill Folder" Button:** Links a local directory of markdown files or scripts to the project.
- **Skill Library List:** Displays linked folders. 
- **"Unlink" / "Delete" Buttons:** Removes the skill from the project's `.loom/workspace.json` data.

## 7. Diagnostics & Monitoring (`NotificationPreview.vue`)
- **Taskbar Badge:** The app icon pulses if an agent is waiting for input or has thrown a fatal error (`fatalError` flag in `TerminalPane`).
- **"View All Terminals" Button:** A floating notification action that auto-focuses the terminal workspace if an agent completes a long-running generation.
- **Terminal Snapshot Audit (`TerminalSnapshotAuditReportModal.vue`):** A hidden debug modal showing the EXACT internal state parsed by `WeztermEmulator` in Rust, used for verifying why an agent behaved a certain way.
