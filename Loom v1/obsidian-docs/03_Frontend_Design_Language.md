---
title: Frontend Design Language
tags: [ui, ux, tailwind, vue]
---

# 🎨 Frontend Design Language

The UI of Loom v1 is built to look like a modern, cyberpunk-inspired workspace. It uses a heavily customized **Tailwind CSS** configuration and a dark-mode first design philosophy.

## 🛠️ Tech Stack
- **Framework:** Vue 3 (Composition API, `<script setup>`)
- **Styling:** Tailwind CSS (`tailwind.config.cjs`) + PostCSS
- **Icons & Graphics:** Custom SVG avatars (`avatarRender.ts`), programmatic canvas drawing.
- **State Management:** Pinia (`globalStore.ts`, `settingsStore.ts`)

## 🌗 Theming System (`theme.ts`)
The application supports dynamic theming, managed by `applyThemeToDom()` and `syncTheme()`.
- **CSS Variables:** Theming relies on CSS Custom Properties (e.g., `--bg-primary`, `--text-muted`) mapped to Tailwind utility classes.
- **System Sync:** Listens to `(prefers-color-scheme: dark)` via `startSystemListener()` to adapt to the OS level if the user chooses the "System" theme option.

## 🧬 Component Structure
Components are highly atomic and split by feature domains:

1. **`shared/components`:**
   - **`SidebarNav.vue`**: The main left-hand navigation bar (Workspace, Chats, Settings).
   - **`MemberStatusDots.vue`**: Programmatic SVG indicators showing if an agent is "Online," "Thinking," or "Error."
2. **`features/chat/components`:**
   - **`ChatInput.vue`**: A rich textarea that supports `@mentions` (`activeMentionIndex`), Emojis (`insertEmoji`, `emoji-data.ts`), and multi-line formatting.
   - **`MessagesList.vue`**: A virtualization-ready list for rendering long histories, featuring `ensureTypewriter()` for smooth text reveals.
3. **`features/terminal`:**
   - **`TerminalPane.vue`**: Wraps the `xterm.js` canvas. Implements custom drag-and-drop (`onPointerUp`, `onPointerDown`) to split panes and arrange terminals in a grid.
   - **`WorkspaceSelection.vue`**: The landing screen where users pick which local directory to bind the system to.

## 📏 UI/UX Philosophies
- **Keyboard First:** `useAppKeybinds.ts` and `keyboard/controller.ts` register complex global shortcuts. Every action (finding text, opening panels, switching tabs) is accessible without a mouse.
- **Passive Monitoring:** Instead of alerting the user on every terminal output, the system uses `NotificationBadgeState` and `NotificationPreview.vue` to show a subtle pulse on the taskbar/avatar when an agent finishes a long task.
