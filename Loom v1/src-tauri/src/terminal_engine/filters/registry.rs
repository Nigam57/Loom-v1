//! CLI 类型到过滤 profile 的映射。

use super::types::TerminalFilterProfile;

pub(crate) fn resolve_profile(terminal_type: &str) -> TerminalFilterProfile {
  match terminal_type.trim().to_lowercase().as_str() {
    // [TODO/core, 2026-01-26] Codex 暂无独立规则，仍复用通用规则集；保留 profile 以便后续分支。
    "codex" => TerminalFilterProfile::Codex,
    "gemini" => TerminalFilterProfile::Gemini,
    "claude" => TerminalFilterProfile::Claude,
    "shell" => TerminalFilterProfile::Shell,
    // Antigravity chat goes through the headless adapter; its PTY fallback uses the generic
    // prompt-block rules instead of passing the whole screen (TUI chrome included) into chat.
    _ => TerminalFilterProfile::Generic,
  }
}
