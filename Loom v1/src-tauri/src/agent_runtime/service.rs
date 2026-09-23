//! Agent runtime service: per-member queues driving headless turns on worker threads.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use serde_json::json;
use tauri::{AppHandle, Emitter};
use ulid::Ulid;

use super::adapters::{self, TurnRequest};
use super::chat_sink::{ChatSink, ChatTurnContext, EventObserver};
use super::queue::MemberQueue;
use super::runner::{kill_process_tree, run_turn, TurnCommand};
use super::sessions::{session_key, SessionStore};
use crate::application::agents::{AgentAdapterKind, AgentManifest};
use crate::mcp_server::{
  ENV_CHAIN, ENV_CONVERSATION_ID, ENV_CONVERSATION_TYPE, ENV_MEMBER_ID, ENV_VIEWER_ID, ENV_WORKSPACE_ID, ENV_WORKSPACE_PATH,
};
use crate::ports::terminal_message::TerminalMessagePipeline;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TurnResult {
  pub(crate) text: String,
  pub(crate) failed: bool,
}

pub(crate) type TurnCallback = Box<dyn FnOnce(TurnResult) + Send>;

pub(crate) struct TurnJob {
  pub(crate) manifest: AgentManifest,
  pub(crate) cwd: PathBuf,
  pub(crate) prompt: String,
  pub(crate) skip_permissions: bool,
  pub(crate) message_id: Option<String>,
  pub(crate) context: ChatTurnContext,
  /// Members already waiting on this turn through `ask_agent`, outermost first.
  pub(crate) chain: Vec<String>,
  pub(crate) on_complete: Option<TurnCallback>,
}

impl TurnJob {
  fn complete(&mut self, result: TurnResult) {
    if let Some(callback) = self.on_complete.take() {
      callback(result);
    }
  }
}

/// Where the `loom-mcp` tool server lives and where per-turn MCP configs are written.
pub(crate) struct McpBridge {
  pub(crate) binary: Option<PathBuf>,
  pub(crate) config_dir: PathBuf,
}

impl McpBridge {
  /// `loom-mcp(.exe)` next to the running app binary, if it was built.
  pub(crate) fn discover(config_dir: PathBuf) -> Self {
    let name = if cfg!(windows) { "loom-mcp.exe" } else { "loom-mcp" };
    let binary = std::env::current_exe()
      .ok()
      .and_then(|exe| exe.parent().map(|dir| dir.join(name)))
      .filter(|path| path.is_file());
    Self { binary, config_dir }
  }

  /// Writes a Claude-style `{"mcpServers": {...}}` file for one turn and returns its path.
  fn write_config(&self, envs: &[(String, String)]) -> Option<PathBuf> {
    let binary = self.binary.as_ref()?;
    let path = self.config_dir.join(format!("mcp-{}.json", Ulid::new()));
    let config = mcp_config_json(binary, envs);
    std::fs::create_dir_all(&self.config_dir).ok()?;
    std::fs::write(&path, serde_json::to_string_pretty(&config).ok()?).ok()?;
    Some(path)
  }
}

/// OpenCode reads extra config from `OPENCODE_CONFIG_CONTENT`, merged over the user's own config.
pub(crate) fn opencode_config_json(binary: &Path, envs: &[(String, String)]) -> serde_json::Value {
  let environment: serde_json::Map<String, serde_json::Value> =
    envs.iter().map(|(key, value)| (key.clone(), json!(value))).collect();
  json!({
    "mcp": {
      "loom": {
        "type": "local",
        "command": [binary.to_string_lossy()],
        "enabled": true,
        "environment": environment
      }
    }
  })
}

pub(crate) fn mcp_config_json(binary: &Path, envs: &[(String, String)]) -> serde_json::Value {
  let env: serde_json::Map<String, serde_json::Value> =
    envs.iter().map(|(key, value)| (key.clone(), json!(value))).collect();
  json!({
    "mcpServers": {
      "loom": {
        "type": "stdio",
        "command": binary.to_string_lossy(),
        "args": [],
        "env": env
      }
    }
  })
}

/// Environment every headless turn receives, so Loom's MCP tools know who is calling.
/// `sender_id` is always the human in the room (delegated turns keep it), which chat uses for unread counts.
pub(crate) fn turn_envs(job: &TurnJob) -> Vec<(String, String)> {
  let mut chain = job.chain.clone();
  chain.push(job.context.member_id.clone());
  vec![
    (ENV_WORKSPACE_ID.to_string(), job.context.workspace_id.clone()),
    (ENV_WORKSPACE_PATH.to_string(), job.cwd.to_string_lossy().to_string()),
    (ENV_CONVERSATION_ID.to_string(), job.context.conversation_id.clone()),
    (ENV_CONVERSATION_TYPE.to_string(), job.context.conversation_type.clone()),
    (ENV_MEMBER_ID.to_string(), job.context.member_id.clone()),
    (ENV_VIEWER_ID.to_string(), job.context.sender_id.clone()),
    (ENV_CHAIN.to_string(), chain.join(",")),
  ]
}

pub(crate) struct AgentRuntime {
  pipeline: Arc<dyn TerminalMessagePipeline>,
  mcp: McpBridge,
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
  pub(crate) fn new(pipeline: Arc<dyn TerminalMessagePipeline>, sessions_path: PathBuf, mcp: McpBridge) -> Self {
    Self {
      pipeline,
      mcp,
      queues: Mutex::new(HashMap::new()),
      sessions: Mutex::new(SessionStore::load(sessions_path)),
      running: Mutex::new(HashMap::new()),
    }
  }

  pub(crate) fn has_mcp(&self) -> bool {
    self.mcp.binary.is_some()
  }

  pub(crate) fn mcp_binary(&self) -> Option<&Path> {
    self.mcp.binary.as_deref()
  }

  /// True when this room has no CLI session for the member yet (first turn gets the room briefing).
  pub(crate) fn is_new_session(&self, workspace_id: &str, conversation_id: &str, member_id: &str) -> bool {
    lock(&self.sessions).get(&session_key(workspace_id, conversation_id, member_id)).is_none()
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

  /// Kill switch for one member: drops waiting turns (releasing anyone waiting on them) and kills the running CLI.
  pub(crate) fn stop_member(&self, workspace_id: &str, member_id: &str) -> usize {
    let key = queue_key(workspace_id, member_id);
    let dropped = lock(&self.queues).get_mut(&key).map(MemberQueue::take_pending).unwrap_or_default();
    let count = dropped.len();
    for mut job in dropped {
      job.complete(TurnResult { text: "Stopped before it started.".to_string(), failed: true });
    }
    if let Some(pid) = lock(&self.running).get(&key).copied() {
      kill_process_tree(pid);
    }
    count
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

  fn run_one(&self, app: &AppHandle, key: &str, mut job: TurnJob) {
    let Some(parse) = adapters::parser_for(job.manifest.adapter) else {
      job.complete(TurnResult { text: format!("{} has no headless adapter.", job.manifest.name), failed: true });
      return;
    };
    let session = session_key(&job.context.workspace_id, &job.context.conversation_id, &job.context.member_id);
    let resume = lock(&self.sessions).get(&session).map(str::to_string);
    let mut envs = turn_envs(&job);
    let mcp_config = match job.manifest.adapter {
      AgentAdapterKind::ClaudeStreamJson => self.mcp.write_config(&envs),
      _ => None,
    };
    if let (AgentAdapterKind::OpencodeJson, Some(binary)) = (job.manifest.adapter, self.mcp.binary.as_ref()) {
      let content = opencode_config_json(binary, &envs).to_string();
      envs.push(("OPENCODE_CONFIG_CONTENT".to_string(), content));
    }
    let mcp_config_text = mcp_config.as_ref().map(|path| path.to_string_lossy().to_string());
    let request = TurnRequest {
      prompt: &job.prompt,
      resume_session_id: resume.as_deref(),
      skip_permissions: job.skip_permissions,
      mcp_config: mcp_config_text.as_deref(),
    };
    let Some(turn_args) = adapters::build_args(job.manifest.adapter, &request) else {
      job.complete(TurnResult { text: format!("{} has no headless adapter.", job.manifest.name), failed: true });
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
    let command = TurnCommand { program: &job.manifest.command, args: &args, cwd: &job.cwd, envs: &envs };
    run_turn(&command, parse, &mut sink, &mut |pid| {
      lock(&self.running).insert(key.to_string(), pid);
    });
    lock(&self.running).remove(key);
    if let Some(path) = mcp_config {
      let _ = std::fs::remove_file(path);
    }

    {
      let mut sessions = lock(&self.sessions);
      match (&sink.outcome.session_id, sink.outcome.failed, resume.is_some()) {
        (Some(session_id), false, _) => sessions.set(session, session_id.clone()),
        // A stale resume id fails every turn; forget it so the next turn starts fresh.
        (_, true, true) => sessions.remove(&session),
        _ => {}
      }
    }
    job.complete(TurnResult { text: sink.outcome.final_text.clone(), failed: sink.outcome.failed });
  }
}

#[tauri::command]
pub(crate) fn agent_stop(
  runtime: tauri::State<'_, Arc<AgentRuntime>>,
  workspace_id: String,
  member_id: String,
) -> Result<usize, String> {
  Ok(runtime.stop_member(&workspace_id, &member_id))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::application::agents::find_agent;

  fn job(chain: Vec<String>) -> TurnJob {
    TurnJob {
      manifest: find_agent("claude", None).unwrap(),
      cwd: PathBuf::from("C:/work"),
      prompt: "hi".into(),
      skip_permissions: false,
      message_id: None,
      context: ChatTurnContext {
        member_id: "m2".into(),
        workspace_id: "w1".into(),
        conversation_id: "c1".into(),
        conversation_type: "channel".into(),
        sender_id: "m1".into(),
        sender_name: "Ada".into(),
      },
      chain,
      on_complete: None,
    }
  }

  #[test]
  fn envs_identify_caller_and_extend_chain() {
    let envs = turn_envs(&job(vec!["m1".into()]));
    assert!(envs.contains(&(ENV_MEMBER_ID.to_string(), "m2".to_string())));
    assert!(envs.contains(&(ENV_CHAIN.to_string(), "m1,m2".to_string())));
    assert!(envs.contains(&(ENV_CONVERSATION_ID.to_string(), "c1".to_string())));
  }

  #[test]
  fn mcp_config_points_at_binary_with_caller_env() {
    let config = mcp_config_json(Path::new("C:/loom/loom-mcp.exe"), &[(ENV_MEMBER_ID.to_string(), "m2".to_string())]);
    assert_eq!(config["mcpServers"]["loom"]["command"], "C:/loom/loom-mcp.exe");
    assert_eq!(config["mcpServers"]["loom"]["env"][ENV_MEMBER_ID], "m2");
  }

  #[test]
  fn completing_a_job_fires_callback_once() {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut job = job(Vec::new());
    job.on_complete = Some(Box::new(move |result| tx.send(result).unwrap()));
    job.complete(TurnResult { text: "done".into(), failed: false });
    job.complete(TurnResult { text: "again".into(), failed: false });
    assert_eq!(rx.try_recv().unwrap().text, "done");
    assert!(rx.try_recv().is_err());
  }
}
