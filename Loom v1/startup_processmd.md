# loom

loom is a local-first workspace application based on Vue 3 + Tauri. Create your team, easily deploy tasks, and bind projects to workspace management.

## Local Development

**Prerequisites:** Node.js, pnpm  
**Desktop Extra Dependencies:** Rust + cargo-tauri

1. Install dependencies  
   `pnpm install`
2. Start Web development server  
   `pnpm dev`
3. Start Desktop development (optional)  
   `cargo tauri dev`
4. Debug Desktop development (optional)  
   `$env:LOOM_TERMINAL_TRACE="1"`
   `$env:LOOM_TERMINAL_TRACE_DETAIL="1"`
   `$env:VITE_TERMINAL_TRACE="1"`
   `cargo tauri dev`


## Clean Rust / Cargo Cache

Execute in src-tauri or project root:

`cargo clean`

This deletes:

`target/`  # default Rust build artifacts directory
Note: cargo clean does not delete Cargo.lock or source code, only the generated binaries.

## Tests

`pnpm test`

## Linting

- `pnpm lint`
- `pnpm format:check`

## Directory Structure

- `src/app/App.vue`: App shell and navigation state
- `src/features`: Feature modules (Workspace, Chat, Skills Store, Plugins, Terminal, etc.)
- `src/shared`: Reusable components and composables
- `src/i18n`: Texts and language configuration
- `src/styles/global.css`: Global styles and utilities