//! Headless turn runner: spawn one CLI turn and stream parsed events to a sink.

use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use super::adapters::LineParser;
use super::events::AgentEvent;

pub(crate) trait AgentEventSink {
  fn on_event(&mut self, event: AgentEvent);
}

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

/// Parse every stdout line; true once a terminal event (completed/failed) was seen.
pub(crate) fn drive<R: BufRead>(reader: R, parse: LineParser, sink: &mut dyn AgentEventSink) -> bool {
  let mut finished = false;
  for line in reader.lines() {
    let Ok(line) = line else { break };
    for event in parse(&line) {
      finished |= event.is_terminal();
      sink.on_event(event);
    }
  }
  finished
}

pub(crate) struct TurnCommand<'a> {
  pub(crate) program: &'a str,
  pub(crate) args: &'a [String],
  pub(crate) cwd: &'a Path,
  pub(crate) envs: &'a [(String, String)],
}

/// Bare command names resolve through PATH. On Windows, npm installs CLIs as `.cmd`/`.ps1` shims that
/// `Command` cannot start directly; for those, use the real `.exe` the package ships under `node_modules`.
pub(crate) fn resolve_program(program: &str, path_var: Option<&std::ffi::OsStr>) -> PathBuf {
  let bare = !program.contains(['/', '\\']) && Path::new(program).extension().is_none();
  if !cfg!(windows) || !bare {
    return PathBuf::from(program);
  }
  let dirs: Vec<PathBuf> = path_var.map(|value| std::env::split_paths(value).collect()).unwrap_or_default();
  if let Some(exe) = dirs.iter().map(|dir| dir.join(format!("{program}.exe"))).find(|path| path.is_file()) {
    return exe;
  }
  for dir in &dirs {
    let is_shim = ["cmd", "ps1"].iter().any(|ext| dir.join(format!("{program}.{ext}")).is_file());
    let Ok(packages) = std::fs::read_dir(dir.join("node_modules")).map(|entries| entries.flatten()) else {
      continue;
    };
    if !is_shim {
      continue;
    }
    let mut candidates: Vec<PathBuf> = packages
      .filter(|entry| !entry.file_name().to_string_lossy().starts_with('.'))
      .map(|entry| entry.path().join("bin").join(format!("{program}.exe")))
      .filter(|path| path.is_file())
      .collect();
    candidates.sort();
    if let Some(exe) = candidates.into_iter().next() {
      return exe;
    }
  }
  PathBuf::from(program)
}

pub(crate) fn run_turn(
  command_spec: &TurnCommand<'_>,
  parse: LineParser,
  sink: &mut dyn AgentEventSink,
  on_spawn: &mut dyn FnMut(u32),
) {
  let program = command_spec.program;
  let mut command = Command::new(resolve_program(program, std::env::var_os("PATH").as_deref()));
  command
    .args(command_spec.args)
    .current_dir(command_spec.cwd)
    .envs(command_spec.envs.iter().map(|(key, value)| (key.as_str(), value.as_str())))
    .stdin(Stdio::null())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped());
  #[cfg(windows)]
  {
    use std::os::windows::process::CommandExt;
    command.creation_flags(CREATE_NO_WINDOW);
  }
  let mut child = match command.spawn() {
    Ok(child) => child,
    Err(err) => {
      sink.on_event(AgentEvent::TurnFailed { message: format!("Could not start `{program}`: {err}") });
      return;
    }
  };
  on_spawn(child.id());
  let stderr = child.stderr.take();
  let stderr_reader = std::thread::spawn(move || {
    let mut text = String::new();
    if let Some(mut stream) = stderr {
      let _ = stream.read_to_string(&mut text);
    }
    text
  });
  let finished = match child.stdout.take() {
    Some(stdout) => drive(BufReader::new(stdout), parse, sink),
    None => false,
  };
  let code = child.wait().ok().and_then(|status| status.code());
  let stderr_text = stderr_reader.join().unwrap_or_default();
  if !finished {
    sink.on_event(AgentEvent::TurnFailed { message: exit_message(program, code, &stderr_text) });
  }
}

fn exit_message(program: &str, code: Option<i32>, stderr: &str) -> String {
  let lines: Vec<&str> = stderr.lines().filter(|line| !line.trim().is_empty()).collect();
  let tail = lines[lines.len().saturating_sub(12)..].join("\n");
  let code = code.map(|value| value.to_string()).unwrap_or_else(|| "none".to_string());
  if tail.is_empty() {
    format!("`{program}` exited with code {code} before replying.")
  } else {
    format!("`{program}` exited with code {code} before replying:\n{tail}")
  }
}

/// Stop a running turn, including the CLI's own child processes.
pub(crate) fn kill_process_tree(pid: u32) {
  #[cfg(windows)]
  {
    use std::os::windows::process::CommandExt;
    let _ = Command::new("taskkill")
      .args(["/PID", &pid.to_string(), "/T", "/F"])
      .creation_flags(CREATE_NO_WINDOW)
      .status();
  }
  #[cfg(not(windows))]
  {
    let _ = Command::new("kill").args(["-TERM", &pid.to_string()]).status();
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::agent_runtime::adapters::agy;

  const FIXTURE: &str = include_str!("adapters/fixtures/agy_tool_call.jsonl");

  #[derive(Default)]
  struct Recorder(Vec<AgentEvent>);

  impl AgentEventSink for Recorder {
    fn on_event(&mut self, event: AgentEvent) {
      self.0.push(event);
    }
  }

  #[test]
  fn drive_finishes_on_result() {
    let mut recorder = Recorder::default();
    assert!(drive(std::io::Cursor::new(FIXTURE), agy::parse_line, &mut recorder));
    assert!(matches!(recorder.0.last(), Some(AgentEvent::TurnCompleted { .. })));
  }

  #[test]
  fn drive_reports_unfinished_stream() {
    let partial: String = FIXTURE.lines().take(4).collect::<Vec<_>>().join("\n");
    let mut recorder = Recorder::default();
    assert!(!drive(std::io::Cursor::new(partial), agy::parse_line, &mut recorder));
  }

  #[test]
  fn missing_program_becomes_turn_failed() {
    let mut recorder = Recorder::default();
    let cwd = std::env::temp_dir();
    let spec = TurnCommand { program: "loom-missing-binary-3f9c", args: &[], cwd: &cwd, envs: &[] };
    run_turn(&spec, agy::parse_line, &mut recorder, &mut |_| {});
    match recorder.0.as_slice() {
      [AgentEvent::TurnFailed { message }] => assert!(message.contains("Could not start")),
      other => panic!("unexpected events: {other:?}"),
    }
  }

  #[cfg(windows)]
  #[test]
  fn resolves_npm_shim_to_packaged_exe() {
    let root = std::env::temp_dir().join(format!("loom-shim-{}", ulid::Ulid::new()));
    let bin = root.join("node_modules").join("opencode-ai").join("bin");
    std::fs::create_dir_all(&bin).unwrap();
    std::fs::create_dir_all(root.join("node_modules").join(".opencode-stale").join("bin")).unwrap();
    std::fs::write(root.join("opencode.cmd"), "@echo off").unwrap();
    std::fs::write(bin.join("opencode.exe"), "").unwrap();
    let path_var = std::env::join_paths([root.clone()]).unwrap();
    assert_eq!(resolve_program("opencode", Some(&path_var)), bin.join("opencode.exe"));
    assert_eq!(resolve_program("missing", Some(&path_var)), PathBuf::from("missing"));
    assert_eq!(resolve_program("C:\\x\\agy.exe", Some(&path_var)), PathBuf::from("C:\\x\\agy.exe"));
    let _ = std::fs::remove_dir_all(root);
  }

  #[test]
  fn exit_message_keeps_stderr_tail() {
    let message = exit_message("agy", Some(1), "\nline one\nAuthentication required\n");
    assert_eq!(message, "`agy` exited with code 1 before replying:\nline one\nAuthentication required");
  }
}
