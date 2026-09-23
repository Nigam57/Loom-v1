# Structured Agent Adapters Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans (or superpowers:subagent-driven-development) to implement this plan task-by-task.

**Goal:** Chat with Antigravity (`agy`) and Claude Code returns only the agent's answer — no TUI chrome — by running them headless with `stream-json` output instead of scraping a PTY screen.

**Architecture:** A new `agent_runtime` module spawns one headless CLI process per turn (`agy -p … --output-format stream-json`, `claude -p … --output-format stream-json --verbose`), parses each NDJSON line into a single `AgentEvent` enum, and feeds a `ChatSink` that reuses the existing `TerminalMessagePipeline` (`terminal-message-stream` deltas + `append_terminal_message` finals keyed by `spanId`), so the frontend needs no changes. Per-member turn queues replace the PTY batcher lock for headless agents; CLI conversation IDs are persisted per room so context carries across turns. PTY stays as the fallback adapter and as a watchable view.

**Tech Stack:** Rust (Tauri 2, serde_json, std::process, std threads — matching the existing thread-based engine), existing chat pipeline. No new crates.

**Master plan:** `2026-09-23-00-master-plan.md` (Phase 1).

---

## Verified facts this plan relies on

- 2026-09-23, this machine: `agy` 1.2.9 prints NDJSON correctly when piped (upstream issue google-antigravity/antigravity-cli#408, empty stdout when piped on 1.0.9, did **not** reproduce). The fixture in Task 2 is a real capture, trimmed only in the `init.tools` list and the `cwd` path.
- `agy` resumes a conversation with `--conversation <id>`; `--dangerously-skip-permissions` auto-approves tools. Known upstream issue #548: print mode ignores `permissions.allow`.
- `claude` 2.1.280 is installed. The Claude fixture in Task 3 is **synthetic**; Task 3 Step 1 captures a real one and you must reconcile the parser if field names differ.
- Frontend `src/features/chat/chatStore.ts:348-402` renders streams by `spanId` (`mode: 'delta'` appends) and `:660-700` replaces the stream bubble with the persisted final message for the same `spanId`.
- Members in project data carry `terminalType`, `terminalCommand`, `unlimitedAccess` (`src-tauri/src/message_service/project_members.rs:126-132`).
- There are **zero** Rust tests in the crate today; this plan adds the first ones. Run all commands from `D:\Loom\Loom v1\src-tauri`.

---

### Task 0: Branch and baseline check

`Loom v1/` is its own git repository (branch `master`, remotes `loom_v1` → `github.com/Nigam57/Loom-v1`, `origin` → upstream Golutra); the outer `D:\Loom` repo only holds the legacy prototype. Work inside `Loom v1`:

```bash
cd "D:\Loom\Loom v1"
git switch -c feat/agent-collab
```

Confirm the crate builds and the (empty) test suite runs before starting: `cargo test --lib` from `src-tauri` → expected `running 0 tests … ok`.

---

### Task 1: `AgentEvent` type and module skeleton

**Files:**
- Create: `src-tauri/src/agent_runtime/mod.rs`
- Create: `src-tauri/src/agent_runtime/events.rs`
- Modify: `src-tauri/src/lib.rs` (add `mod agent_runtime;` next to `mod application;`)

**Step 1: Write the failing test** — `src-tauri/src/agent_runtime/events.rs`

```rust
//! Structured agent events: the one shape every adapter produces and chat consumes.

use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AgentUsage {
  pub(crate) input_tokens: u64,
  pub(crate) output_tokens: u64,
  pub(crate) thinking_tokens: u64,
  pub(crate) cache_read_tokens: u64,
  pub(crate) cost_usd: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub(crate) enum AgentEvent {
  SessionStarted { session_id: String },
  TextDelta { text: String },
  ToolStarted { id: String, name: String, input: Value },
  ToolFinished { id: String, is_error: bool },
  TurnCompleted { session_id: Option<String>, text: String, usage: Option<AgentUsage> },
  TurnFailed { message: String },
}

impl AgentEvent {
  /// A turn ends on exactly one of these.
  pub(crate) fn is_terminal(&self) -> bool {
    matches!(self, AgentEvent::TurnCompleted { .. } | AgentEvent::TurnFailed { .. })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn serializes_with_kind_tag_and_camel_case_fields() {
    let event = AgentEvent::SessionStarted { session_id: "abc".to_string() };
    let json = serde_json::to_value(&event).unwrap();
    assert_eq!(json, serde_json::json!({ "kind": "sessionStarted", "sessionId": "abc" }));
  }

  #[test]
  fn only_completed_and_failed_are_terminal() {
    assert!(AgentEvent::TurnFailed { message: "x".into() }.is_terminal());
    assert!(!AgentEvent::TextDelta { text: "x".into() }.is_terminal());
  }
}
```

`src-tauri/src/agent_runtime/mod.rs`:

```rust
//! Headless agent runtime: structured CLI adapters instead of PTY screen scraping.

pub(crate) mod events;
```

**Step 2: Run** `cargo test --lib agent_runtime::events` → Expected: 2 passed. (If it fails to compile with `rename_all_fields`, serde is older than 1.0.185; the lockfile pins 1.0.229, so this should not happen.)

**Step 3: Commit** `git commit -am "feat(agent-runtime): add AgentEvent type"`

---

### Task 2: Antigravity (`agy`) stream-json parser

**Files:**
- Create: `src-tauri/src/agent_runtime/adapters/mod.rs`
- Create: `src-tauri/src/agent_runtime/adapters/agy.rs`
- Create: `src-tauri/src/agent_runtime/adapters/fixtures/agy_tool_call.jsonl`
- Modify: `src-tauri/src/agent_runtime/mod.rs` (add `pub(crate) mod adapters;`)

**Step 1: Add the real fixture** — `adapters/fixtures/agy_tool_call.jsonl` (one JSON object per line, exactly as below):

```
{"event":"init","conversation_id":"e9beba6d-92d2-43bc-bc0d-600f70c939fc","init":{"cwd":"C:\\tmp\\agy-probe","tools":["run_command","view_file"]}}
{"event":"step_update","step_update":{"conversation_id":"e9beba6d-92d2-43bc-bc0d-600f70c939fc","step_index":0,"state":"DONE","step_type":"user_input"}}
{"event":"step_update","step_update":{"conversation_id":"e9beba6d-92d2-43bc-bc0d-600f70c939fc","step_index":1,"state":"DONE","step_type":"agent_response","duration_seconds":8.632046,"usage":{"input_tokens":4759,"output_tokens":854,"thinking_tokens":785,"cache_read_tokens":8091,"total_tokens":5613}}}
{"event":"step_update","step_update":{"conversation_id":"e9beba6d-92d2-43bc-bc0d-600f70c939fc","step_index":2,"state":"ACTIVE","step_type":"tool","tool_name":"run_command","tool_info":{"name":"run_command","parameters":{"CommandLine":"Get-ChildItem"}}}}
{"event":"step_update","step_update":{"conversation_id":"e9beba6d-92d2-43bc-bc0d-600f70c939fc","step_index":2,"state":"DONE","step_type":"tool","tool_name":"run_command","duration_seconds":0.2446146,"tool_info":{"name":"run_command","parameters":{"CommandLine":"Get-ChildItem"}}}}
{"event":"step_update","step_update":{"conversation_id":"e9beba6d-92d2-43bc-bc0d-600f70c939fc","step_index":3,"state":"ACTIVE","step_type":"agent_response","text_delta":"I have"}}
{"event":"step_update","step_update":{"conversation_id":"e9beba6d-92d2-43bc-bc0d-600f70c939fc","step_index":3,"state":"ACTIVE","step_type":"agent_response","text_delta":" listed the files in the current dir"}}
{"event":"step_update","step_update":{"conversation_id":"e9beba6d-92d2-43bc-bc0d-600f70c939fc","step_index":3,"state":"ACTIVE","step_type":"agent_response","text_delta":"ectory (which is current"}}
{"event":"step_update","step_update":{"conversation_id":"e9beba6d-92d2-43bc-bc0d-600f70c939fc","step_index":3,"state":"DONE","step_type":"agent_response","text_delta":"ly empty). \n\nDone.\n","duration_seconds":2.9317532,"usage":{"input_tokens":5700,"output_tokens":37,"thinking_tokens":18,"cache_read_tokens":8092,"total_tokens":5737}}}
{"event":"result","result":{"conversation_id":"e9beba6d-92d2-43bc-bc0d-600f70c939fc","status":"SUCCESS","response":"I have listed the files in the current directory (which is currently empty). \n\nDone.\n","duration_seconds":11.9884851,"num_turns":1,"usage":{"input_tokens":10459,"output_tokens":891,"thinking_tokens":803,"cache_read_tokens":16183,"total_tokens":11350}}}
```

**Step 2: Write shared helpers** — `adapters/mod.rs`

```rust
//! CLI adapters: turn each CLI's NDJSON line format into `AgentEvent`s.

pub(crate) mod agy;

use serde_json::Value;

use super::events::AgentEvent;

pub(crate) type LineParser = fn(&str) -> Vec<AgentEvent>;

pub(super) fn str_field(value: &Value, key: &str) -> String {
  value.get(key).and_then(Value::as_str).unwrap_or("").to_string()
}

pub(super) fn u64_field(value: &Value, key: &str) -> u64 {
  value.get(key).and_then(Value::as_u64).unwrap_or(0)
}
```

**Step 3: Write the failing tests** — bottom of `adapters/agy.rs` (write the tests first, with an empty `parse_line` that returns `Vec::new()`):

```rust
#[cfg(test)]
mod tests {
  use super::*;
  use crate::agent_runtime::events::AgentUsage;

  const FIXTURE: &str = include_str!("fixtures/agy_tool_call.jsonl");
  const SESSION: &str = "e9beba6d-92d2-43bc-bc0d-600f70c939fc";
  const REPLY: &str = "I have listed the files in the current directory (which is currently empty). \n\nDone.\n";

  fn events() -> Vec<AgentEvent> {
    FIXTURE.lines().flat_map(parse_line).collect()
  }

  #[test]
  fn starts_with_session() {
    assert_eq!(events()[0], AgentEvent::SessionStarted { session_id: SESSION.to_string() });
  }

  #[test]
  fn text_deltas_join_into_reply() {
    let text: String = events()
      .iter()
      .filter_map(|event| match event {
        AgentEvent::TextDelta { text } => Some(text.as_str()),
        _ => None,
      })
      .collect();
    assert_eq!(text, REPLY);
  }

  #[test]
  fn reports_tool_start_with_parameters_then_finish() {
    let tools: Vec<AgentEvent> = events()
      .into_iter()
      .filter(|event| matches!(event, AgentEvent::ToolStarted { .. } | AgentEvent::ToolFinished { .. }))
      .collect();
    assert_eq!(
      tools,
      vec![
        AgentEvent::ToolStarted {
          id: "step-2".into(),
          name: "run_command".into(),
          input: serde_json::json!({ "CommandLine": "Get-ChildItem" }),
        },
        AgentEvent::ToolFinished { id: "step-2".into(), is_error: false },
      ]
    );
  }

  #[test]
  fn ends_with_completed_turn_and_usage() {
    assert_eq!(
      events().last().unwrap(),
      &AgentEvent::TurnCompleted {
        session_id: Some(SESSION.to_string()),
        text: REPLY.to_string(),
        usage: Some(AgentUsage {
          input_tokens: 10459,
          output_tokens: 891,
          thinking_tokens: 803,
          cache_read_tokens: 16183,
          cost_usd: None,
        }),
      }
    );
  }

  #[test]
  fn non_success_status_is_failure() {
    let line = r#"{"event":"result","result":{"conversation_id":"c","status":"ERROR","error":"auth required"}}"#;
    assert_eq!(parse_line(line), vec![AgentEvent::TurnFailed { message: "auth required".into() }]);
  }

  #[test]
  fn ignores_blank_and_non_json_lines() {
    assert!(parse_line("").is_empty());
    assert!(parse_line("Loaded cached credentials.").is_empty());
  }
}
```

**Step 4: Run** `cargo test --lib agent_runtime::adapters::agy` → Expected: FAIL (empty parser).

**Step 5: Implement** — top of `adapters/agy.rs`:

```rust
//! Antigravity CLI (`agy -p … --output-format stream-json`) line parser.

use serde_json::Value;

use super::{str_field, u64_field};
use crate::agent_runtime::events::{AgentEvent, AgentUsage};

pub(crate) fn parse_line(line: &str) -> Vec<AgentEvent> {
  let Ok(value) = serde_json::from_str::<Value>(line.trim()) else {
    return Vec::new();
  };
  match value.get("event").and_then(Value::as_str) {
    Some("init") => value
      .get("conversation_id")
      .and_then(Value::as_str)
      .map(|id| vec![AgentEvent::SessionStarted { session_id: id.to_string() }])
      .unwrap_or_default(),
    Some("step_update") => value.get("step_update").map(parse_step).unwrap_or_default(),
    Some("result") => value.get("result").map(parse_result).unwrap_or_default(),
    _ => Vec::new(),
  }
}

fn parse_step(step: &Value) -> Vec<AgentEvent> {
  let state = step.get("state").and_then(Value::as_str).unwrap_or("");
  match step.get("step_type").and_then(Value::as_str) {
    Some("agent_response") => step
      .get("text_delta")
      .and_then(Value::as_str)
      .filter(|text| !text.is_empty())
      .map(|text| vec![AgentEvent::TextDelta { text: text.to_string() }])
      .unwrap_or_default(),
    Some("tool") => {
      let id = format!("step-{}", u64_field(step, "step_index"));
      match state {
        "ACTIVE" => vec![AgentEvent::ToolStarted {
          id,
          name: str_field(step, "tool_name"),
          input: step.pointer("/tool_info/parameters").cloned().unwrap_or(Value::Null),
        }],
        "DONE" => vec![AgentEvent::ToolFinished { id, is_error: false }],
        _ => Vec::new(),
      }
    }
    _ => Vec::new(),
  }
}

fn parse_result(result: &Value) -> Vec<AgentEvent> {
  let status = result.get("status").and_then(Value::as_str).unwrap_or("");
  if status != "SUCCESS" {
    let message = match result.get("error") {
      Some(Value::String(text)) => text.clone(),
      Some(other) => other.to_string(),
      None => format!("agy finished with status {status}"),
    };
    return vec![AgentEvent::TurnFailed { message }];
  }
  vec![AgentEvent::TurnCompleted {
    session_id: result.get("conversation_id").and_then(Value::as_str).map(str::to_string),
    text: str_field(result, "response"),
    usage: result.get("usage").map(|usage| AgentUsage {
      input_tokens: u64_field(usage, "input_tokens"),
      output_tokens: u64_field(usage, "output_tokens"),
      thinking_tokens: u64_field(usage, "thinking_tokens"),
      cache_read_tokens: u64_field(usage, "cache_read_tokens"),
      cost_usd: None,
    }),
  }]
}
```

Note: a tool's `ACTIVE` state could be reported more than once for long tools; downstream consumers must treat `ToolStarted` as idempotent by `id`.

**Step 6: Run** `cargo test --lib agent_runtime::adapters::agy` → Expected: 6 passed.

**Step 7: Commit** `git commit -am "feat(agent-runtime): parse agy stream-json"`

---

### Task 3: Claude Code stream-json parser

**Files:**
- Create: `src-tauri/src/agent_runtime/adapters/claude.rs`
- Create: `src-tauri/src/agent_runtime/adapters/fixtures/claude_tool_call.jsonl`
- Modify: `src-tauri/src/agent_runtime/adapters/mod.rs` (add `pub(crate) mod claude;`)

**Step 1: Capture a real stream to reconcile against** (PowerShell, in an empty temp dir):

```powershell
$d = Join-Path $env:TEMP "claude-probe"; New-Item -ItemType Directory -Force $d | Out-Null; Set-Location $d
claude -p "List the files here with a tool, then say done" --output-format stream-json --verbose | Out-File -Encoding utf8 claude_real.jsonl
Get-Content claude_real.jsonl | Select-Object -First 8
```

Expected shapes: `{"type":"system","subtype":"init","session_id":…}`, `{"type":"assistant","message":{"content":[{"type":"text"|"tool_use",…}]}}`, `{"type":"user","message":{"content":[{"type":"tool_result","tool_use_id":…}]}}`, `{"type":"result","subtype":"success","result":…,"session_id":…,"total_cost_usd":…,"usage":{…}}`. If the real capture differs, update the parser and the synthetic fixture below to match reality, and keep the real capture (with paths scrubbed) as an extra fixture.

**Step 2: Add the synthetic fixture** — `fixtures/claude_tool_call.jsonl`:

```
{"type":"system","subtype":"init","session_id":"sess-1","cwd":"C:\\tmp","tools":["Bash"]}
{"type":"assistant","message":{"content":[{"type":"text","text":"Let me look."},{"type":"tool_use","id":"toolu_1","name":"Bash","input":{"command":"ls"}}]},"session_id":"sess-1"}
{"type":"user","message":{"content":[{"type":"tool_result","tool_use_id":"toolu_1","content":"a.txt","is_error":false}]},"session_id":"sess-1"}
{"type":"assistant","message":{"content":[{"type":"text","text":"There is one file: a.txt."}]},"session_id":"sess-1"}
{"type":"result","subtype":"success","is_error":false,"result":"There is one file: a.txt.","session_id":"sess-1","total_cost_usd":0.0123,"usage":{"input_tokens":120,"output_tokens":30,"cache_read_input_tokens":4000}}
```

**Step 3: Write the failing tests** (bottom of `claude.rs`, with a stub `parse_line` returning `Vec::new()`):

```rust
#[cfg(test)]
mod tests {
  use super::*;
  use crate::agent_runtime::events::AgentUsage;

  const FIXTURE: &str = include_str!("fixtures/claude_tool_call.jsonl");

  fn events() -> Vec<AgentEvent> {
    FIXTURE.lines().flat_map(parse_line).collect()
  }

  #[test]
  fn maps_full_turn() {
    assert_eq!(
      events(),
      vec![
        AgentEvent::SessionStarted { session_id: "sess-1".into() },
        AgentEvent::TextDelta { text: "Let me look.".into() },
        AgentEvent::ToolStarted {
          id: "toolu_1".into(),
          name: "Bash".into(),
          input: serde_json::json!({ "command": "ls" }),
        },
        AgentEvent::ToolFinished { id: "toolu_1".into(), is_error: false },
        AgentEvent::TextDelta { text: "There is one file: a.txt.".into() },
        AgentEvent::TurnCompleted {
          session_id: Some("sess-1".into()),
          text: "There is one file: a.txt.".into(),
          usage: Some(AgentUsage {
            input_tokens: 120,
            output_tokens: 30,
            thinking_tokens: 0,
            cache_read_tokens: 4000,
            cost_usd: Some(0.0123),
          }),
        },
      ]
    );
  }

  #[test]
  fn error_result_is_failure() {
    let line = r#"{"type":"result","subtype":"error_max_turns","is_error":true,"session_id":"s"}"#;
    assert_eq!(
      parse_line(line),
      vec![AgentEvent::TurnFailed { message: "claude finished with error_max_turns".into() }]
    );
  }
}
```

**Step 4: Run** `cargo test --lib agent_runtime::adapters::claude` → Expected: FAIL.

**Step 5: Implement** (top of `claude.rs`):

```rust
//! Claude Code (`claude -p … --output-format stream-json --verbose`) line parser.

use serde_json::Value;

use super::{str_field, u64_field};
use crate::agent_runtime::events::{AgentEvent, AgentUsage};

pub(crate) fn parse_line(line: &str) -> Vec<AgentEvent> {
  let Ok(value) = serde_json::from_str::<Value>(line.trim()) else {
    return Vec::new();
  };
  match value.get("type").and_then(Value::as_str) {
    Some("system") if value.get("subtype").and_then(Value::as_str) == Some("init") => value
      .get("session_id")
      .and_then(Value::as_str)
      .map(|id| vec![AgentEvent::SessionStarted { session_id: id.to_string() }])
      .unwrap_or_default(),
    Some("assistant") => content_blocks(&value)
      .iter()
      .filter_map(|block| match block.get("type").and_then(Value::as_str) {
        Some("text") => block
          .get("text")
          .and_then(Value::as_str)
          .filter(|text| !text.is_empty())
          .map(|text| AgentEvent::TextDelta { text: text.to_string() }),
        Some("tool_use") => Some(AgentEvent::ToolStarted {
          id: str_field(block, "id"),
          name: str_field(block, "name"),
          input: block.get("input").cloned().unwrap_or(Value::Null),
        }),
        _ => None,
      })
      .collect(),
    Some("user") => content_blocks(&value)
      .iter()
      .filter(|block| block.get("type").and_then(Value::as_str) == Some("tool_result"))
      .map(|block| AgentEvent::ToolFinished {
        id: str_field(block, "tool_use_id"),
        is_error: block.get("is_error").and_then(Value::as_bool).unwrap_or(false),
      })
      .collect(),
    Some("result") => parse_result(&value),
    _ => Vec::new(),
  }
}

fn content_blocks(value: &Value) -> &[Value] {
  value
    .pointer("/message/content")
    .and_then(Value::as_array)
    .map(Vec::as_slice)
    .unwrap_or(&[])
}

fn parse_result(value: &Value) -> Vec<AgentEvent> {
  let subtype = value.get("subtype").and_then(Value::as_str).unwrap_or("");
  let is_error = value.get("is_error").and_then(Value::as_bool).unwrap_or(false) || subtype != "success";
  if is_error {
    let message = value
      .get("result")
      .and_then(Value::as_str)
      .filter(|text| !text.trim().is_empty())
      .map(str::to_string)
      .unwrap_or_else(|| format!("claude finished with {subtype}"));
    return vec![AgentEvent::TurnFailed { message }];
  }
  vec![AgentEvent::TurnCompleted {
    session_id: value.get("session_id").and_then(Value::as_str).map(str::to_string),
    text: str_field(value, "result"),
    usage: value.get("usage").map(|usage| AgentUsage {
      input_tokens: u64_field(usage, "input_tokens"),
      output_tokens: u64_field(usage, "output_tokens"),
      thinking_tokens: 0,
      cache_read_tokens: u64_field(usage, "cache_read_input_tokens"),
      cost_usd: value.get("total_cost_usd").and_then(Value::as_f64),
    }),
  }]
}
```

**Step 6: Run** `cargo test --lib agent_runtime::adapters` → Expected: all agy + claude tests pass. Also sanity-run the parser over the real capture from Step 1 by temporarily adding a test that `include_str!`s it and asserts the last event is `TurnCompleted`.

**Step 7: Commit** `git commit -am "feat(agent-runtime): parse claude stream-json"`

---

### Task 4: Agent manifests with an adapter kind, loaded from `.loom/agents/*.json`

**Files:**
- Modify: `src-tauri/src/application/agents.rs` (replace the hardcoded list)
- Modify: `src-tauri/src/terminal_engine/semantic.rs:93-98` (use `find_agent`)

**Step 1: Write the failing tests** (append to `agents.rs`):

```rust
#[cfg(test)]
mod tests {
  use super::*;

  fn temp_workspace() -> PathBuf {
    let dir = std::env::temp_dir().join(format!("loom-agents-{}", ulid::Ulid::new()));
    std::fs::create_dir_all(dir.join(".loom").join("agents")).unwrap();
    dir
  }

  #[test]
  fn builtins_mark_agy_and_claude_headless() {
    assert_eq!(find_agent("antigravity-cli", None).unwrap().adapter, AgentAdapterKind::AgyStreamJson);
    assert_eq!(find_agent("claude", None).unwrap().adapter, AgentAdapterKind::ClaudeStreamJson);
    assert_eq!(find_agent("shell", None).unwrap().adapter, AgentAdapterKind::Pty);
  }

  #[test]
  fn workspace_manifest_overrides_builtin_and_adds_new() {
    let root = temp_workspace();
    std::fs::write(
      root.join(".loom/agents/agy.json"),
      r#"{"id":"antigravity-cli","name":"Antigravity (fast)","command":"agy","args":["--model","fast"],"adapter":"agy-stream-json"}"#,
    )
    .unwrap();
    std::fs::write(
      root.join(".loom/agents/aider.json"),
      r#"{"id":"aider","name":"Aider","command":"aider","args":[]}"#,
    )
    .unwrap();
    std::fs::write(root.join(".loom/agents/broken.json"), "{not json").unwrap();

    let agy = find_agent("antigravity-cli", Some(&root)).unwrap();
    assert_eq!(agy.args, vec!["--model".to_string(), "fast".to_string()]);
    let aider = find_agent("aider", Some(&root)).unwrap();
    assert_eq!(aider.adapter, AgentAdapterKind::Pty);
    assert!(find_agent("broken", Some(&root)).is_none());
    let _ = std::fs::remove_dir_all(root);
  }
}
```

**Step 2: Run** `cargo test --lib application::agents` → Expected: FAIL to compile (`find_agent`, `AgentAdapterKind` missing).

**Step 3: Implement** — replace `agents.rs` body:

```rust
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AgentAdapterKind {
    AgyStreamJson,
    ClaudeStreamJson,
    #[default]
    Pty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentManifestParsing {
    pub prompt_marker: String,
    pub bullet_marker: String,
    pub ready_signal_ms: u64,
}

impl Default for AgentManifestParsing {
    fn default() -> Self {
        Self {
            prompt_marker: "›".to_string(),
            bullet_marker: "•".to_string(),
            ready_signal_ms: 500,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentManifest {
    pub id: String,
    pub name: String,
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub adapter: AgentAdapterKind,
    #[serde(default)]
    pub parsing: AgentManifestParsing,
}

fn manifest(id: &str, name: &str, command: &str, adapter: AgentAdapterKind, prompt_marker: &str) -> AgentManifest {
    AgentManifest {
        id: id.to_string(),
        name: name.to_string(),
        command: command.to_string(),
        args: Vec::new(),
        adapter,
        parsing: AgentManifestParsing {
            prompt_marker: prompt_marker.to_string(),
            ..AgentManifestParsing::default()
        },
    }
}

pub(crate) fn builtin_agents() -> Vec<AgentManifest> {
    vec![
        manifest("antigravity-cli", "Antigravity", "agy", AgentAdapterKind::AgyStreamJson, "❯"),
        manifest("claude", "Claude Code", "claude", AgentAdapterKind::ClaudeStreamJson, "❯"),
        manifest("shell", "Shell", "shell", AgentAdapterKind::Pty, "❯"),
    ]
}

/// Built-ins, then `<workspace>/.loom/agents/*.json` (same id replaces, new id appends).
pub(crate) fn resolve_agents(workspace_path: Option<&Path>) -> Vec<AgentManifest> {
    let mut agents = builtin_agents();
    let Some(root) = workspace_path else {
        return agents;
    };
    let Ok(entries) = std::fs::read_dir(root.join(".loom").join("agents")) else {
        return agents;
    };
    let mut paths: Vec<PathBuf> = entries
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    paths.sort();
    for path in paths {
        let parsed = std::fs::read_to_string(&path)
            .map_err(|err| err.to_string())
            .and_then(|raw| serde_json::from_str::<AgentManifest>(&raw).map_err(|err| err.to_string()));
        match parsed {
            Ok(next) => match agents.iter_mut().find(|agent| agent.id == next.id) {
                Some(slot) => *slot = next,
                None => agents.push(next),
            },
            Err(err) => log::warn!("skipping agent manifest {}: {err}", path.display()),
        }
    }
    agents
}

pub(crate) fn find_agent(agent_id: &str, workspace_path: Option<&Path>) -> Option<AgentManifest> {
    let wanted = agent_id.trim().to_lowercase();
    resolve_agents(workspace_path).into_iter().find(|agent| agent.id == wanted)
}

#[tauri::command]
pub fn get_available_agents(workspace_path: Option<String>) -> Result<Vec<AgentManifest>, String> {
    Ok(resolve_agents(workspace_path.as_deref().map(Path::new)))
}
```

Then in `terminal_engine/semantic.rs` replace the `get_available_agents()` lookup with:

```rust
    let (prompt_marker, bullet_marker) = crate::application::agents::find_agent(&terminal_type, None)
      .map(|agent| (agent.parsing.prompt_marker, agent.parsing.bullet_marker))
      .unwrap_or_else(|| ("›".to_string(), "•".to_string()));
```

**Step 4: Run** `cargo test --lib` → Expected: all tests pass; `cargo build` succeeds. The frontend's `invoke('get_available_agents')` still works (the new argument is optional). Check the frontend list still renders (Claude Code now appears).

**Step 5: Commit** `git commit -am "feat(agents): manifest adapter kind + workspace overrides"`

---

### Task 5: Build CLI arguments per adapter

**Files:**
- Modify: `src-tauri/src/agent_runtime/adapters/mod.rs`

**Step 1: Failing tests** (append to `adapters/mod.rs`):

```rust
#[cfg(test)]
mod tests {
  use super::*;

  fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
  }

  #[test]
  fn agy_fresh_turn() {
    let request = TurnRequest { prompt: "hi", resume_session_id: None, skip_permissions: false };
    assert_eq!(
      build_args(AgentAdapterKind::AgyStreamJson, &request),
      Some(strings(&["-p", "hi", "--output-format", "stream-json"]))
    );
  }

  #[test]
  fn agy_resume_with_skip_permissions() {
    let request = TurnRequest { prompt: "hi", resume_session_id: Some("c1"), skip_permissions: true };
    assert_eq!(
      build_args(AgentAdapterKind::AgyStreamJson, &request),
      Some(strings(&["-p", "hi", "--output-format", "stream-json", "--conversation", "c1", "--dangerously-skip-permissions"]))
    );
  }

  #[test]
  fn claude_resume() {
    let request = TurnRequest { prompt: "hi", resume_session_id: Some("s1"), skip_permissions: false };
    assert_eq!(
      build_args(AgentAdapterKind::ClaudeStreamJson, &request),
      Some(strings(&["-p", "hi", "--output-format", "stream-json", "--verbose", "--resume", "s1"]))
    );
  }

  #[test]
  fn pty_has_no_headless_args() {
    let request = TurnRequest { prompt: "hi", resume_session_id: None, skip_permissions: false };
    assert_eq!(build_args(AgentAdapterKind::Pty, &request), None);
    assert!(parser_for(AgentAdapterKind::Pty).is_none());
  }
}
```

**Step 2: Run** → FAIL. **Step 3: Implement** (add to `adapters/mod.rs`, plus `pub(crate) mod claude;` and `use crate::application::agents::AgentAdapterKind;`):

```rust
pub(crate) struct TurnRequest<'a> {
  pub(crate) prompt: &'a str,
  pub(crate) resume_session_id: Option<&'a str>,
  pub(crate) skip_permissions: bool,
}

pub(crate) fn parser_for(kind: AgentAdapterKind) -> Option<LineParser> {
  match kind {
    AgentAdapterKind::AgyStreamJson => Some(agy::parse_line),
    AgentAdapterKind::ClaudeStreamJson => Some(claude::parse_line),
    AgentAdapterKind::Pty => None,
  }
}

pub(crate) fn build_args(kind: AgentAdapterKind, request: &TurnRequest<'_>) -> Option<Vec<String>> {
  let (mut args, resume_flag) = match kind {
    AgentAdapterKind::AgyStreamJson => (vec!["-p", request.prompt, "--output-format", "stream-json"], "--conversation"),
    AgentAdapterKind::ClaudeStreamJson => {
      (vec!["-p", request.prompt, "--output-format", "stream-json", "--verbose"], "--resume")
    }
    AgentAdapterKind::Pty => return None,
  };
  if let Some(session_id) = request.resume_session_id {
    args.extend([resume_flag, session_id]);
  }
  if request.skip_permissions {
    args.push("--dangerously-skip-permissions");
  }
  Some(args.into_iter().map(str::to_string).collect())
}
```

Known limits to record in code comments: the prompt goes through argv (Windows caps a command line at 32,767 chars; long prompts move to stdin/`--input-format stream-json` in a later task), and a prompt starting with `-` may be read as a flag — prefix a space if `prompt.starts_with('-')`.

**Step 4: Run** `cargo test --lib agent_runtime::adapters` → PASS. **Step 5: Commit** `git commit -am "feat(agent-runtime): per-adapter CLI args"`

---

### Task 6: Turn runner (spawn, stream, fail loudly)

**Files:**
- Create: `src-tauri/src/agent_runtime/runner.rs`
- Modify: `src-tauri/src/agent_runtime/mod.rs` (add `pub(crate) mod runner;`)

**Step 1: Failing tests** (bottom of `runner.rs`):

```rust
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
    run_turn("loom-missing-binary-3f9c", &[], &std::env::temp_dir(), agy::parse_line, &mut recorder, &mut |_| {});
    match recorder.0.as_slice() {
      [AgentEvent::TurnFailed { message }] => assert!(message.contains("Could not start")),
      other => panic!("unexpected events: {other:?}"),
    }
  }

  #[test]
  fn exit_message_keeps_stderr_tail() {
    let message = exit_message("agy", Some(1), "\nline one\nAuthentication required\n");
    assert_eq!(message, "`agy` exited with code 1 before replying:\nline one\nAuthentication required");
  }
}
```

**Step 2: Run** → FAIL. **Step 3: Implement** (top of `runner.rs`):

```rust
//! Headless turn runner: spawn one CLI turn and stream parsed events to a sink.

use std::io::{BufRead, BufReader, Read};
use std::path::Path;
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

pub(crate) fn run_turn(
  program: &str,
  args: &[String],
  cwd: &Path,
  parse: LineParser,
  sink: &mut dyn AgentEventSink,
  on_spawn: &mut dyn FnMut(u32),
) {
  let mut command = Command::new(program);
  command
    .args(args)
    .current_dir(cwd)
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
```

Note: `Command::new("gemini")` will not resolve npm `.ps1`/`.cmd` shims — only real executables (`agy.exe`, `claude.exe`) work headless here. Resolving shims is a follow-up (see master plan Phase 1b).

**Step 4: Run** `cargo test --lib agent_runtime::runner` → PASS. **Step 5: Commit** `git commit -am "feat(agent-runtime): headless turn runner"`

---

### Task 7: `ChatSink` — events into the existing chat pipeline

**Files:**
- Create: `src-tauri/src/agent_runtime/chat_sink.rs`
- Modify: `src-tauri/src/agent_runtime/mod.rs` (add `pub(crate) mod chat_sink;`)

**Step 1: Failing tests** (bottom of `chat_sink.rs`):

```rust
#[cfg(test)]
mod tests {
  use std::sync::Mutex;

  use super::*;
  use crate::agent_runtime::adapters::agy;
  use crate::ports::message_service::TerminalMessageAppendResult;

  const FIXTURE: &str = include_str!("adapters/fixtures/agy_tool_call.jsonl");

  #[derive(Default)]
  struct RecordingPipeline {
    calls: Mutex<Vec<(String, String)>>,
  }

  impl TerminalMessagePipeline for RecordingPipeline {
    fn process_stream(&self, payload: TerminalMessagePayload) -> Result<(), String> {
      self.calls.lock().unwrap().push((payload.mode, payload.content));
      Ok(())
    }
    fn process_final(&self, payload: TerminalMessagePayload) -> Result<TerminalMessageAppendResult, String> {
      self.calls.lock().unwrap().push((payload.mode, payload.content));
      Ok(TerminalMessageAppendResult::skipped())
    }
  }

  fn context(conversation_type: &str) -> ChatTurnContext {
    ChatTurnContext {
      member_id: "m1".into(),
      workspace_id: "w1".into(),
      conversation_id: "c1".into(),
      conversation_type: conversation_type.into(),
      sender_id: "u1".into(),
      sender_name: "Mira".into(),
    }
  }

  fn run(conversation_type: &str, lines: &str) -> (Arc<RecordingPipeline>, ChatSink) {
    let pipeline = Arc::new(RecordingPipeline::default());
    let mut sink = ChatSink::new(context(conversation_type), "span-1".into(), pipeline.clone(), None);
    for event in lines.lines().flat_map(agy::parse_line) {
      sink.on_event(event);
    }
    (pipeline, sink)
  }

  #[test]
  fn streams_deltas_then_posts_clean_final() {
    let (pipeline, sink) = run("dm", FIXTURE);
    let calls = pipeline.calls.lock().unwrap();
    assert!(calls[..calls.len() - 1].iter().all(|(mode, _)| mode == "delta"));
    assert_eq!(
      calls.last().unwrap(),
      &("final".to_string(), "I have listed the files in the current directory (which is currently empty). \n\nDone.".to_string())
    );
    assert_eq!(sink.outcome.session_id.as_deref(), Some("e9beba6d-92d2-43bc-bc0d-600f70c939fc"));
    assert!(!sink.outcome.failed);
  }

  #[test]
  fn channel_final_mentions_sender() {
    let (pipeline, _) = run("channel", FIXTURE);
    let calls = pipeline.calls.lock().unwrap();
    assert!(calls.last().unwrap().1.starts_with("@Mira I have listed"));
  }

  #[test]
  fn failure_posts_error_message() {
    let pipeline = Arc::new(RecordingPipeline::default());
    let mut sink = ChatSink::new(context("dm"), "span-1".into(), pipeline.clone(), None);
    sink.on_event(AgentEvent::TurnFailed { message: "boom".into() });
    assert_eq!(pipeline.calls.lock().unwrap().last().unwrap(), &("final".to_string(), "Agent error: boom".to_string()));
    assert!(sink.outcome.failed);
  }
}
```

**Step 2: Run** → FAIL. **Step 3: Implement** (top of `chat_sink.rs`):

```rust
//! Chat sink: feeds structured agent events into the existing stream/final chat pipeline.

use std::sync::Arc;

use super::events::{AgentEvent, AgentUsage};
use super::runner::AgentEventSink;
use crate::contracts::terminal_message::{TerminalMessageMeta, TerminalMessagePayload};
use crate::now_millis;
use crate::ports::terminal_message::TerminalMessagePipeline;

#[derive(Clone, Debug)]
pub(crate) struct ChatTurnContext {
  pub(crate) member_id: String,
  pub(crate) workspace_id: String,
  pub(crate) conversation_id: String,
  pub(crate) conversation_type: String,
  pub(crate) sender_id: String,
  pub(crate) sender_name: String,
}

#[derive(Debug, Default, PartialEq)]
pub(crate) struct TurnOutcome {
  pub(crate) session_id: Option<String>,
  pub(crate) usage: Option<AgentUsage>,
  pub(crate) failed: bool,
}

/// Sees every event before the sink handles it (used to emit `agent-event` to the UI).
pub(crate) type EventObserver = Box<dyn FnMut(&ChatTurnContext, &str, &AgentEvent) + Send>;

pub(crate) struct ChatSink {
  context: ChatTurnContext,
  span_id: String,
  pipeline: Arc<dyn TerminalMessagePipeline>,
  observer: Option<EventObserver>,
  seq: u64,
  streamed: String,
  break_before_next_text: bool,
  pub(crate) outcome: TurnOutcome,
}

impl ChatSink {
  pub(crate) fn new(
    context: ChatTurnContext,
    span_id: String,
    pipeline: Arc<dyn TerminalMessagePipeline>,
    observer: Option<EventObserver>,
  ) -> Self {
    Self {
      context,
      span_id,
      pipeline,
      observer,
      seq: 0,
      streamed: String::new(),
      break_before_next_text: false,
      outcome: TurnOutcome::default(),
    }
  }

  fn payload(&mut self, content: String, mode: &str) -> TerminalMessagePayload {
    self.seq += 1;
    TerminalMessagePayload {
      terminal_id: format!("agent:{}", self.context.member_id),
      member_id: Some(self.context.member_id.clone()),
      workspace_id: Some(self.context.workspace_id.clone()),
      conversation_id: Some(self.context.conversation_id.clone()),
      conversation_type: Some(self.context.conversation_type.clone()),
      sender_id: Some(self.context.sender_id.clone()),
      sender_name: Some(self.context.sender_name.clone()),
      seq: self.seq,
      timestamp: now_millis().unwrap_or(0),
      content,
      message_type: "info".to_string(),
      source: "chat".to_string(),
      mode: mode.to_string(),
      span_id: Some(self.span_id.clone()),
      meta: Some(TerminalMessageMeta { command: None, line_count: None, cursor: None, start_row: None, end_row: None }),
    }
  }

  /// Channel replies must mention the sender or the chat side ignores them (same rule as `build_semantic_payload`).
  fn with_channel_mention(&self, content: String) -> String {
    let sender = self.context.sender_name.trim();
    if self.context.conversation_type != "channel" || sender.is_empty() {
      return content;
    }
    let mention = format!("@{sender}");
    if content.trim_start().starts_with(&mention) {
      content
    } else {
      format!("{mention} {content}")
    }
  }

  fn post_final(&mut self, content: String) {
    let content = if content.trim().is_empty() { "(no reply)".to_string() } else { content };
    let content = self.with_channel_mention(content);
    let payload = self.payload(content, "final");
    if let Err(err) = self.pipeline.process_final(payload) {
      log::warn!("agent chat append failed member_id={} err={err}", self.context.member_id);
    }
  }
}

impl AgentEventSink for ChatSink {
  fn on_event(&mut self, event: AgentEvent) {
    if let Some(observer) = self.observer.as_mut() {
      observer(&self.context, &self.span_id, &event);
    }
    match event {
      AgentEvent::SessionStarted { session_id } => self.outcome.session_id = Some(session_id),
      AgentEvent::TextDelta { text } => {
        let delta = if self.break_before_next_text && !self.streamed.is_empty() { format!("\n\n{text}") } else { text };
        self.break_before_next_text = false;
        self.streamed.push_str(&delta);
        let payload = self.payload(delta, "delta");
        if let Err(err) = self.pipeline.process_stream(payload) {
          log::warn!("agent stream emit failed member_id={} err={err}", self.context.member_id);
        }
      }
      AgentEvent::ToolStarted { .. } => self.break_before_next_text = true,
      AgentEvent::ToolFinished { .. } => {}
      AgentEvent::TurnCompleted { session_id, text, usage } => {
        if session_id.is_some() {
          self.outcome.session_id = session_id;
        }
        self.outcome.usage = usage;
        let content = if text.trim().is_empty() { std::mem::take(&mut self.streamed) } else { text };
        self.post_final(content.trim_end().to_string());
      }
      AgentEvent::TurnFailed { message } => {
        self.outcome.failed = true;
        self.post_final(format!("Agent error: {message}"));
      }
    }
  }
}
```

**Step 4: Run** `cargo test --lib agent_runtime::chat_sink` → PASS. **Step 5: Commit** `git commit -am "feat(agent-runtime): chat sink over existing pipeline"`

---

### Task 8: Per-member turn queue and session store

**Files:**
- Create: `src-tauri/src/agent_runtime/queue.rs`
- Create: `src-tauri/src/agent_runtime/sessions.rs`
- Modify: `src-tauri/src/agent_runtime/mod.rs` (add both modules)

**Step 1: Failing tests** — `queue.rs` bottom:

```rust
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn first_turn_runs_next_waits_duplicates_drop() {
    let mut queue = MemberQueue::default();
    assert_eq!(queue.push(Some("a".into()), 1), Some(1));
    assert_eq!(queue.push(Some("b".into()), 2), None);
    assert_eq!(queue.push(Some("a".into()), 9), None);
    assert_eq!(queue.push(Some("b".into()), 9), None);
    assert_eq!(queue.finish(), Some(2));
    assert_eq!(queue.finish(), None);
    assert!(!queue.is_running());
    assert_eq!(queue.push(None, 3), Some(3));
  }

  #[test]
  fn clear_pending_keeps_running_turn() {
    let mut queue = MemberQueue::default();
    queue.push(None, 1);
    queue.push(None, 2);
    queue.push(None, 3);
    assert_eq!(queue.clear_pending(), 2);
    assert!(queue.is_running());
    assert_eq!(queue.finish(), None);
  }
}
```

`sessions.rs` bottom:

```rust
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn persists_and_reloads() {
    let path = std::env::temp_dir().join(format!("loom-sessions-{}.json", ulid::Ulid::new()));
    let key = session_key("w", "c", "m");
    let mut store = SessionStore::load(path.clone());
    store.set(key.clone(), "s1".into());
    assert_eq!(SessionStore::load(path.clone()).get(&key), Some("s1"));
    store.remove(&key);
    assert_eq!(SessionStore::load(path.clone()).get(&key), None);
    let _ = std::fs::remove_file(path);
  }
}
```

**Step 2: Run** → FAIL. **Step 3: Implement** — `queue.rs`:

```rust
//! Per-member turn queue: one running turn at a time, later messages wait, duplicate message ids drop.

use std::collections::VecDeque;

pub(crate) struct MemberQueue<T> {
  running: bool,
  inflight_id: Option<String>,
  pending: VecDeque<(Option<String>, T)>,
}

impl<T> Default for MemberQueue<T> {
  fn default() -> Self {
    Self { running: false, inflight_id: None, pending: VecDeque::new() }
  }
}

impl<T> MemberQueue<T> {
  /// Returns the item if it should start now.
  pub(crate) fn push(&mut self, message_id: Option<String>, item: T) -> Option<T> {
    if let Some(id) = message_id.as_deref() {
      let duplicate = self.inflight_id.as_deref() == Some(id)
        || self.pending.iter().any(|(pending_id, _)| pending_id.as_deref() == Some(id));
      if duplicate {
        return None;
      }
    }
    if self.running {
      self.pending.push_back((message_id, item));
      return None;
    }
    self.running = true;
    self.inflight_id = message_id;
    Some(item)
  }

  /// Call when the running turn ends; returns the next one to start.
  pub(crate) fn finish(&mut self) -> Option<T> {
    match self.pending.pop_front() {
      Some((id, item)) => {
        self.inflight_id = id;
        Some(item)
      }
      None => {
        self.running = false;
        self.inflight_id = None;
        None
      }
    }
  }

  pub(crate) fn clear_pending(&mut self) -> usize {
    let dropped = self.pending.len();
    self.pending.clear();
    dropped
  }

  pub(crate) fn is_running(&self) -> bool {
    self.running
  }
}
```

`sessions.rs`:

```rust
//! CLI conversation ids per room + member, so each room keeps its own agent context across turns.

use std::collections::HashMap;
use std::path::PathBuf;

pub(crate) fn session_key(workspace_id: &str, conversation_id: &str, member_id: &str) -> String {
  format!("{workspace_id}/{conversation_id}/{member_id}")
}

pub(crate) struct SessionStore {
  path: PathBuf,
  entries: HashMap<String, String>,
}

impl SessionStore {
  pub(crate) fn load(path: PathBuf) -> Self {
    let entries = std::fs::read_to_string(&path)
      .ok()
      .and_then(|raw| serde_json::from_str(&raw).ok())
      .unwrap_or_default();
    Self { path, entries }
  }

  pub(crate) fn get(&self, key: &str) -> Option<&str> {
    self.entries.get(key).map(String::as_str)
  }

  pub(crate) fn set(&mut self, key: String, session_id: String) {
    self.entries.insert(key, session_id);
    self.persist();
  }

  pub(crate) fn remove(&mut self, key: &str) {
    if self.entries.remove(key).is_some() {
      self.persist();
    }
  }

  fn persist(&self) {
    if let Some(parent) = self.path.parent() {
      let _ = std::fs::create_dir_all(parent);
    }
    let result = serde_json::to_string_pretty(&self.entries)
      .map_err(|err| err.to_string())
      .and_then(|raw| std::fs::write(&self.path, raw).map_err(|err| err.to_string()));
    if let Err(err) = result {
      log::warn!("agent session store write failed path={} err={err}", self.path.display());
    }
  }
}
```

**Step 4: Run** `cargo test --lib agent_runtime` → PASS. **Step 5: Commit** `git commit -am "feat(agent-runtime): turn queue and session store"`

---

### Task 9: `AgentRuntime` service, `agent-event` emit, `agent_stop` command

**Files:**
- Create: `src-tauri/src/agent_runtime/service.rs`
- Modify: `src-tauri/src/agent_runtime/mod.rs` (add `mod service; pub(crate) use service::{AgentRuntime, TurnJob};` and `pub(crate) use chat_sink::ChatTurnContext;`)
- Modify: `src-tauri/src/lib.rs` (manage the runtime)
- Modify: `src-tauri/src/ui_gateway/commands.rs` (register `agent_stop`)

No unit test here (needs an `AppHandle`); it is covered by Task 11's manual run. Keep it thin — all logic lives in the tested pieces.

**Step 1: Implement `service.rs`:**

```rust
//! Agent runtime service: per-member queues driving headless turns on worker threads.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use serde_json::json;
use tauri::{AppHandle, Emitter};
use ulid::Ulid;

use super::adapters::{self, TurnRequest};
use super::chat_sink::{ChatSink, ChatTurnContext, EventObserver};
use super::queue::MemberQueue;
use super::runner::{kill_process_tree, run_turn};
use super::sessions::{session_key, SessionStore};
use crate::application::agents::AgentManifest;
use crate::ports::terminal_message::TerminalMessagePipeline;

pub(crate) struct TurnJob {
  pub(crate) manifest: AgentManifest,
  pub(crate) cwd: PathBuf,
  pub(crate) prompt: String,
  pub(crate) skip_permissions: bool,
  pub(crate) message_id: Option<String>,
  pub(crate) context: ChatTurnContext,
}

pub(crate) struct AgentRuntime {
  pipeline: Arc<dyn TerminalMessagePipeline>,
  queues: Mutex<HashMap<String, MemberQueue<TurnJob>>>,
  sessions: Mutex<SessionStore>,
  running: Mutex<HashMap<String, u32>>,
}

fn queue_key(workspace_id: &str, member_id: &str) -> String {
  format!("{workspace_id}/{member_id}")
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
  mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

impl AgentRuntime {
  pub(crate) fn new(pipeline: Arc<dyn TerminalMessagePipeline>, sessions_path: PathBuf) -> Self {
    Self {
      pipeline,
      queues: Mutex::new(HashMap::new()),
      sessions: Mutex::new(SessionStore::load(sessions_path)),
      running: Mutex::new(HashMap::new()),
    }
  }

  pub(crate) fn enqueue(self: &Arc<Self>, app: AppHandle, job: TurnJob) {
    let key = queue_key(&job.context.workspace_id, &job.context.member_id);
    let message_id = job.message_id.clone();
    let start = lock(&self.queues).entry(key.clone()).or_default().push(message_id, job);
    if let Some(job) = start {
      let runtime = Arc::clone(self);
      std::thread::spawn(move || runtime.run_queue(app, key, job));
    }
  }

  /// Kill switch for one member: drops waiting turns and kills the running CLI.
  pub(crate) fn stop_member(&self, workspace_id: &str, member_id: &str) -> usize {
    let key = queue_key(workspace_id, member_id);
    let dropped = lock(&self.queues).get_mut(&key).map(MemberQueue::clear_pending).unwrap_or(0);
    if let Some(pid) = lock(&self.running).get(&key).copied() {
      kill_process_tree(pid);
    }
    dropped
  }

  fn run_queue(self: Arc<Self>, app: AppHandle, key: String, mut job: TurnJob) {
    loop {
      self.run_one(&app, &key, job);
      let next = {
        let mut queues = lock(&self.queues);
        let next = queues.get_mut(&key).and_then(MemberQueue::finish);
        if next.is_none() {
          queues.remove(&key);
        }
        next
      };
      match next {
        Some(next) => job = next,
        None => break,
      }
    }
  }

  fn run_one(&self, app: &AppHandle, key: &str, job: TurnJob) {
    let Some(parse) = adapters::parser_for(job.manifest.adapter) else {
      return;
    };
    let session = session_key(&job.context.workspace_id, &job.context.conversation_id, &job.context.member_id);
    let resume = lock(&self.sessions).get(&session).map(str::to_string);
    let request = TurnRequest {
      prompt: &job.prompt,
      resume_session_id: resume.as_deref(),
      skip_permissions: job.skip_permissions,
    };
    let Some(turn_args) = adapters::build_args(job.manifest.adapter, &request) else {
      return;
    };
    let args: Vec<String> = job.manifest.args.iter().cloned().chain(turn_args).collect();

    let emitter = app.clone();
    let observer: EventObserver = Box::new(move |context, span_id, event| {
      let _ = emitter.emit(
        "agent-event",
        json!({
          "workspaceId": context.workspace_id,
          "conversationId": context.conversation_id,
          "memberId": context.member_id,
          "spanId": span_id,
          "event": event,
        }),
      );
    });
    let mut sink = ChatSink::new(job.context.clone(), Ulid::new().to_string(), Arc::clone(&self.pipeline), Some(observer));
    run_turn(&job.manifest.command, &args, &job.cwd, parse, &mut sink, &mut |pid| {
      lock(&self.running).insert(key.to_string(), pid);
    });
    lock(&self.running).remove(key);

    let mut sessions = lock(&self.sessions);
    match (&sink.outcome.session_id, sink.outcome.failed, resume.is_some()) {
      (Some(session_id), false, _) => sessions.set(session, session_id.clone()),
      // A stale resume id fails every turn; forget it so the next turn starts fresh.
      (_, true, true) => sessions.remove(&session),
      _ => {}
    }
  }
}
```

**Step 2: Wire into `lib.rs` setup** — change the pipeline block so the same pipeline feeds both paths (move the `app_data_dir` line above it):

```rust
            let app_data_dir = app.path().app_data_dir()?;
            let transport = Arc::new(UiMessageTransport::new(app_handle.clone()));
            let repository = Arc::new(UiMessageRepository::new(app_handle.clone()));
            let pipeline = Arc::new(UiTerminalMessagePipeline::new(transport, repository));
            app.state::<TerminalManager>()
                .set_message_pipeline(pipeline.clone());
            app.manage(Arc::new(agent_runtime::AgentRuntime::new(
                pipeline,
                app_data_dir.join("agent_sessions.json"),
            )));
```

(and delete the later duplicate `let app_data_dir = …` line).

**Step 3: Add the command** — in `service.rs` (or `ui_gateway/terminal.rs` if you prefer the gateway pattern):

```rust
#[tauri::command]
pub(crate) fn agent_stop(
  runtime: tauri::State<'_, Arc<AgentRuntime>>,
  workspace_id: String,
  member_id: String,
) -> Result<usize, String> {
  Ok(runtime.stop_member(&workspace_id, &member_id))
}
```

Register `crate::agent_runtime::agent_stop` in `ui_gateway/commands.rs` `generate_handler![…]` (re-export it from `agent_runtime/mod.rs`).

**Step 4: Run** `cargo build` → compiles; `cargo test --lib` → all pass. **Step 5: Commit** `git commit -am "feat(agent-runtime): runtime service, agent-event, agent_stop"`

---

### Task 10: Route chat dispatch to headless agents

**Files:**
- Modify: `src-tauri/src/orchestration/dispatch.rs`

**Step 1: Failing tests** (new `#[cfg(test)] mod tests` at the bottom of `dispatch.rs`) for the pure decision function:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn config(terminal_type: &str, command: Option<&str>) -> MemberTerminalConfig {
        MemberTerminalConfig {
            id: "m1".into(),
            name: "Agent".into(),
            terminal_type: Some(terminal_type.into()),
            terminal_command: command.map(str::to_string),
            terminal_path: None,
            unlimited_access: false,
        }
    }

    #[test]
    fn agy_member_goes_headless() {
        assert!(headless_manifest(&config("antigravity-cli", None), "").is_some());
        assert!(headless_manifest(&config("antigravity-cli", Some("agy --dangerously-skip-permissions")), "").is_some());
    }

    #[test]
    fn custom_command_or_pty_type_stays_on_pty() {
        assert!(headless_manifest(&config("antigravity-cli", Some("wsl agy")), "").is_none());
        assert!(headless_manifest(&config("shell", None), "").is_none());
        assert!(headless_manifest(&config("gemini", None), "").is_none());
    }
}
```

**Step 2: Run** → FAIL. **Step 3: Implement:**

1. Add `unlimited_access: bool` to `MemberTerminalConfig`, read in `collect_member_configs` with `obj.get("unlimitedAccess").and_then(Value::as_bool).unwrap_or(false)`.
2. Add:

```rust
use std::path::{Path, PathBuf};

use crate::agent_runtime::{AgentRuntime, ChatTurnContext, TurnJob};
use crate::application::agents::{find_agent, AgentAdapterKind, AgentManifest};

/// Headless adapter for this member, unless it uses a custom launch command.
fn headless_manifest(config: &MemberTerminalConfig, workspace_path: &str) -> Option<AgentManifest> {
    let workspace = (!workspace_path.is_empty()).then(|| Path::new(workspace_path));
    let manifest = find_agent(config.terminal_type.as_deref()?, workspace)?;
    if manifest.adapter == AgentAdapterKind::Pty {
        return None;
    }
    let custom_program = config
        .terminal_command
        .as_deref()
        .and_then(|command| command.split_whitespace().next())
        .is_some_and(|program| !program.eq_ignore_ascii_case(&manifest.command));
    (!custom_program).then_some(manifest)
}
```

3. In the dispatch loop, before `ensure_backend_member_session`:

```rust
        if let Some(manifest) = headless_manifest(config, &payload.workspace_path) {
            app.state::<Arc<AgentRuntime>>().enqueue(
                app.clone(),
                TurnJob {
                    manifest,
                    cwd: PathBuf::from(&payload.workspace_path),
                    prompt: payload.text.clone(),
                    skip_permissions: config.unlimited_access,
                    message_id: payload.message_id.clone(),
                    context: ChatTurnContext {
                        member_id: config.id.clone(),
                        workspace_id: payload.workspace_id.clone(),
                        conversation_id: payload.conversation_id.clone(),
                        conversation_type: payload.conversation_type.clone(),
                        sender_id: payload.sender_id.clone(),
                        sender_name: payload.sender_name.clone(),
                    },
                },
            );
            dispatched_count = dispatched_count.saturating_add(1);
            continue;
        }
```

**Step 4: Run** `cargo test --lib` → PASS; `cargo build` → OK. **Step 5: Commit** `git commit -am "feat(dispatch): send agy/claude members through headless runtime"`

---

### Task 11: Stop the Antigravity PTY passthrough, then verify end to end

**Files:**
- Modify: `src-tauri/src/terminal_engine/filters/registry.rs:83` — map `"antigravity-cli"` to `TerminalFilterProfile::Generic` (the passthrough is what dumped the whole screen into chat). Delete `filters/profiles/antigravity.rs` and its enum variant once nothing references them.

**Manual verification (required — record results in the PR):**

1. `pnpm install; pnpm run dev:tauri` (from `D:\Loom\Loom v1`).
2. Open a workspace, invite an Antigravity member, open a DM, send: `List the files in this folder, then say done.`
   - Expected: a streaming reply appears and is replaced by one clean final message. No boxes, spinners, status bars, prompt echo.
3. Send a follow-up: `What did you just list?` → Expected: it remembers (resume via `--conversation`). Check `%APPDATA%\<app id>\agent_sessions.json` has an entry.
4. Rename `agy.exe` temporarily (or set a member command to a missing binary) → Expected: an `Agent error: Could not start …` message, and the next message still goes through (no deadlock).
5. Send a long task, invoke `agent_stop` from devtools (`window.__TAURI__.core.invoke('agent_stop', { workspaceId, memberId })`) → Expected: process tree killed; `Agent error: … exited …` posted; queue unblocked.
6. Repeat 2–3 with a Claude Code member.
7. Open the agent's terminal pane: the PTY terminal still works for manual, interactive use.

**Commit** `git commit -am "fix(terminal): drop antigravity passthrough profile"`

---

## Follow-ups (tracked in the master plan, not in this plan)

- Phase 1b: Gemini (`--output-format stream-json`), Codex (`codex exec --json`), OpenCode (`opencode run --format json`) adapters — capture real fixtures first, as in Tasks 2/3; resolve npm `.ps1/.cmd` shims to their real entry points.
- Persistent processes (`agy --input-format stream-json`, `claude --input-format stream-json`) instead of one process per turn, and prompts over stdin.
- Permission prompts routed to a Loom approval inbox via MCP (`claude --permission-prompt-tool`), instead of all-or-nothing `--dangerously-skip-permissions`.
- Windows Job Objects for process-tree control (legacy `loom-desktop` already has this code).
- Frontend: render `agent-event` tool calls, status and usage (UI plan 03).
