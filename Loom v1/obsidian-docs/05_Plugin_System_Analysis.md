---
title: Plugin System Analysis
tags: [plugins, skills, backend, architecture, root-cause, debugging]
---

# 🧩 Plugin System Analysis

**Investigation Phase 1:** Tracing the execution flow of the "Skills" system.

## The Symptom
The UI allows you to add a "Skill Folder" to your workspace. The folder appears in the UI, but the AI agents never seem to have access to the skills or prompts inside them.

## Root Cause
The plugin system is a visual facade. 
1. When you click "Add Skill", the frontend calls `project_skills_link()` in `src-tauri/src/ui_gateway/project_skills.rs`.
2. This function takes the source path and creates an OS-level symlink inside `.loom/skills/`.
3. It then returns a success payload to Vue, which renders the Skill Card.
4. **The Dead End:** There is absolutely no code in the `WeztermEmulator`, `ChatDispatchBatcher`, or anywhere else in the Rust backend that actually *reads* these symlinked files or injects their contents into the PTY session. 

Because the backend execution environment is completely unaware of the `.loom/skills/` directory, the plugins cannot function.

## Recommended Fix
Instead of symlinking raw files, the Skill system must be rebuilt around the **Model Context Protocol (MCP)**. When a skill is added, an Embedded MCP Server should mount the directory, and the `A2A Message Router` should dynamically fetch context from it when an agent is queried.
