//! Registers `loom-mcp` with CLIs that keep a global MCP list (agy). Claude gets a per-turn `--mcp-config` instead.
//! Safe to leave registered: outside Loom the server exposes no tools (it only activates with Loom's turn env).

use std::path::Path;
use std::process::Command;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

fn quiet(program: &str) -> Command {
  #[allow(unused_mut)]
  let mut command = Command::new(program);
  #[cfg(windows)]
  {
    use std::os::windows::process::CommandExt;
    command.creation_flags(CREATE_NO_WINDOW);
  }
  command
}

/// True when `agy mcp list` output already has a `loom` entry pointing at `binary`.
pub(crate) fn agy_has_loom(list_output: &str, binary: &str) -> bool {
  list_output.lines().any(|line| {
    let mut parts = line.split_whitespace();
    parts.next() == Some("loom") && line.contains(binary)
  })
}

/// Best effort; logs and moves on if agy is missing or refuses.
pub(crate) fn ensure_agy_registration(binary: &Path) {
  let binary_text = binary.to_string_lossy().to_string();
  let listed = match quiet("agy").args(["mcp", "list"]).output() {
    Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
    Err(_) => return, // agy not installed
  };
  if agy_has_loom(&listed, &binary_text) {
    return;
  }
  match quiet("agy").args(["mcp", "add", "loom"]).arg(&binary_text).output() {
    Ok(output) if output.status.success() => log::info!("registered loom-mcp with agy: {binary_text}"),
    Ok(output) => log::warn!(
      "agy mcp add loom failed: {}",
      String::from_utf8_lossy(&output.stderr).trim()
    ),
    Err(err) => log::warn!("agy mcp add loom failed: {err}"),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn detects_existing_registration_by_name_and_path() {
    let listed = "NAME    TYPE   STATUS   COMMAND/URL\nloom    stdio  enabled  C:\\Loom\\loom-mcp.exe\nstitch  stdio  enabled  npx -y stitch-mcp-stdio\n";
    assert!(agy_has_loom(listed, "C:\\Loom\\loom-mcp.exe"));
    assert!(!agy_has_loom(listed, "D:\\other\\loom-mcp.exe"));
    assert!(!agy_has_loom("NAME TYPE\nstitch stdio enabled npx loom-mcp.exe\n", "loom-mcp.exe"));
  }
}
