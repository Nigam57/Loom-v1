---
title: Loom v1 Codebase Overview
tags: [index, loom, overview]
---

# 🏠 Loom v1 (formerly Golutra)

Welcome to the **Loom v1** codebase documentation vault! This vault was generated dynamically via `graphiphy` AST structural analysis. 

## 🗺️ Vault Map
- **[[01_Architecture]]**: The high-level split between the Rust backend (Tauri) and the Vue 3 frontend.
- **[[02_Design_Decisions]]**: Why the system is built the way it is (e.g., PTYs over standard IPC, SQLite choice).
- **[[03_Frontend_Design_Language]]**: Vue 3, Tailwind CSS, theming, and component structures.
- **[[04_Features_and_UI_Reference]]**: A complete mapping of every feature, button, and user interaction.

## 🚀 What is Loom v1?
Loom v1 is a desktop application engineered as an **Agentic AI Overseer System**. It wraps CLI tools (like Claude, Gemini, etc.) into a "multiplayer chat interface," where AIs are treated as "Members" or "Friends" that the user can interact with. Behind the scenes, it manages real pseudoterminal (PTY) streams, ensuring full CLI interactivity disguised as a consumer-friendly chat app.

> **Next step for Loom:** Evolving from interactive CLI sessions into the true `Transport-Agnostic Adapter` and `A2A Message Router` defined in the [[loom_build_spec]].
