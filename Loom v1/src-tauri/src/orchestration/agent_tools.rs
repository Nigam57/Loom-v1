//! Loom tools for agents (served to every CLI by `loom-mcp`): see who is in the room, ask each other, share the room.
//! Boundary: resolves members and posts/reads room messages; running turns is the agent runtime's job.

use std::path::PathBuf;
use std::sync::{mpsc, Arc};
use std::time::Duration;

use serde_json::Value;
use tauri::{AppHandle, Manager};

use super::dispatch::{collect_member_configs, headless_manifest, member_label, room_briefing, MemberTerminalConfig};
use crate::agent_runtime::{AgentRuntime, ChatTurnContext, TurnJob};
use crate::mcp_server::ToolCaller;
use crate::message_service::chat_db::{
  chat_append_terminal_message, chat_get_conversation_member_ids, chat_get_messages, ChatDbManager, MessageContent,
};
use crate::message_service::project_data;
use crate::runtime::StorageManager;

/// Longest ask_agent chain (A asks B asks C asks D).
const MAX_DELEGATION_DEPTH: usize = 4;
const ASK_TIMEOUT: Duration = Duration::from_secs(15 * 60);
const READ_DEFAULT: u32 = 20;
const READ_MAX: u32 = 100;
const READ_TEXT_LIMIT: usize = 2000;

pub(crate) fn execute_tool(app: &AppHandle, tool: &str, arguments: &Value, caller: &ToolCaller) -> Result<String, String> {
  let members = load_room_members(app, caller)?;
  match tool {
    "list_agents" => Ok(list_agents(&members, caller)),
    "ask_agent" => ask_agent(app, &members, caller, arguments),
    "room_read_messages" => read_messages(app, &members, caller, arguments),
    "room_post_message" => post_message(app, &members, caller, arguments),
    other => Err(format!("Unknown Loom tool: {other}")),
  }
}

fn load_room_members(app: &AppHandle, caller: &ToolCaller) -> Result<Vec<MemberTerminalConfig>, String> {
  let storage = app.state::<StorageManager>();
  let project = project_data::read_project_data(&storage, &caller.workspace_path, &caller.workspace_id)?;
  let configs = project.data.as_ref().map(collect_member_configs).unwrap_or_default();
  let ids = chat_get_conversation_member_ids(
    app.state::<ChatDbManager>(),
    caller.workspace_id.clone(),
    caller.conversation_id.clone(),
  )?;
  Ok(ids.iter().filter_map(|id| configs.get(id).cloned()).collect())
}

fn str_arg<'a>(arguments: &'a Value, key: &str) -> Result<&'a str, String> {
  arguments
    .get(key)
    .and_then(Value::as_str)
    .map(str::trim)
    .filter(|value| !value.is_empty())
    .ok_or_else(|| format!("`{key}` is required"))
}

/// Match by id, then by name (case-insensitive, spaces and a leading @ ignored).
pub(crate) fn resolve_target<'a>(members: &'a [MemberTerminalConfig], query: &str) -> Option<&'a MemberTerminalConfig> {
  let normalize = |value: &str| value.trim().trim_start_matches('@').replace(' ', "").to_lowercase();
  let wanted = normalize(query);
  members
    .iter()
    .find(|member| member.id == query.trim())
    .or_else(|| members.iter().find(|member| normalize(&member.name) == wanted))
}

pub(crate) fn check_delegation(caller: &ToolCaller, target: &MemberTerminalConfig) -> Result<(), String> {
  if target.id == caller.member_id {
    return Err("You can't ask yourself. Answer directly.".to_string());
  }
  if caller.chain.iter().any(|id| id == &target.id) {
    return Err(format!(
      "{} is already waiting on your answer, so asking it back would deadlock. Answer with what you have.",
      target.name
    ));
  }
  if caller.chain.len() >= MAX_DELEGATION_DEPTH {
    return Err(format!("Delegation limit reached ({MAX_DELEGATION_DEPTH} agents deep). Answer with what you have."));
  }
  Ok(())
}

/// Members whose `@Name` appears in the text.
pub(crate) fn mentioned<'a>(text: &str, members: &'a [MemberTerminalConfig]) -> Vec<&'a MemberTerminalConfig> {
  let lower = text.to_lowercase();
  members
    .iter()
    .filter(|member| !member.name.trim().is_empty() && lower.contains(&format!("@{}", member.name.trim().to_lowercase())))
    .collect()
}

pub(crate) fn list_agents(members: &[MemberTerminalConfig], caller: &ToolCaller) -> String {
  let me = members.iter().find(|member| member.id == caller.member_id);
  let mut lines = vec![format!("You are {}.", me.map(member_label).unwrap_or_else(|| "an agent in this room".to_string()))];
  let others: Vec<String> = members
    .iter()
    .filter(|member| member.id != caller.member_id)
    .map(|member| {
      let reach = if headless_manifest(member, &caller.workspace_path).is_some() {
        "can be asked with ask_agent"
      } else {
        "terminal only, reachable by @mention in room_post_message"
      };
      format!("- {} — {reach}", member_label(member))
    })
    .collect();
  if others.is_empty() {
    lines.push("No other agents are in this room. The human can add more.".to_string());
  } else {
    lines.push("Other agents in this room:".to_string());
    lines.extend(others);
  }
  lines.join("\n")
}

fn caller_name(members: &[MemberTerminalConfig], caller: &ToolCaller) -> String {
  members
    .iter()
    .find(|member| member.id == caller.member_id)
    .map(|member| member.name.clone())
    .unwrap_or_else(|| "An agent".to_string())
}

fn post_as_caller(app: &AppHandle, caller: &ToolCaller, content: String) -> Result<(), String> {
  chat_append_terminal_message(
    app,
    app.state::<ChatDbManager>().inner(),
    &caller.workspace_id,
    &caller.conversation_id,
    &caller.member_id,
    content,
    &caller.viewer_id,
    None,
  )
  .map(|_| ())
}

/// A turn for `target` on behalf of the caller; the first turn in a room carries the room briefing.
fn delegated_job(
  runtime: &AgentRuntime,
  members: &[MemberTerminalConfig],
  caller: &ToolCaller,
  target: &MemberTerminalConfig,
  prompt: String,
) -> Option<TurnJob> {
  let manifest = headless_manifest(target, &caller.workspace_path)?;
  let prompt = if runtime.is_new_session(&caller.workspace_id, &caller.conversation_id, &target.id) {
    let others: Vec<&MemberTerminalConfig> = members
      .iter()
      .filter(|member| member.id != target.id && headless_manifest(member, &caller.workspace_path).is_some())
      .collect();
    format!("{}\n\n{prompt}", room_briefing(target, &others, runtime.has_mcp()))
  } else {
    prompt
  };
  Some(TurnJob {
    manifest,
    cwd: PathBuf::from(&caller.workspace_path),
    prompt,
    skip_permissions: target.unlimited_access,
    message_id: None,
    context: ChatTurnContext {
      member_id: target.id.clone(),
      workspace_id: caller.workspace_id.clone(),
      conversation_id: caller.conversation_id.clone(),
      conversation_type: caller.conversation_type.clone(),
      // The human stays the viewer; the reply mentions the asking agent.
      sender_id: caller.viewer_id.clone(),
      sender_name: caller_name(members, caller),
    },
    chain: caller.chain.clone(),
    on_complete: None,
  })
}

fn ask_agent(app: &AppHandle, members: &[MemberTerminalConfig], caller: &ToolCaller, arguments: &Value) -> Result<String, String> {
  let query = str_arg(arguments, "agent")?;
  let message = str_arg(arguments, "message")?;
  let target = resolve_target(members, query)
    .ok_or_else(|| format!("No agent named \"{query}\" in this room. Call list_agents to see who is here."))?;
  check_delegation(caller, target)?;
  let runtime = app.state::<Arc<AgentRuntime>>();
  let from = caller_name(members, caller);
  let Some(mut job) = delegated_job(&runtime, members, caller, target, format!("{from} asks you: {message}")) else {
    return Err(format!(
      "{} runs in a terminal only and can't answer ask_agent yet. Mention it in room_post_message instead.",
      target.name
    ));
  };
  post_as_caller(app, caller, format!("@{} {message}", target.name))?;
  let (sender, receiver) = mpsc::channel();
  job.on_complete = Some(Box::new(move |result| {
    let _ = sender.send(result);
  }));
  runtime.enqueue(app.clone(), job);
  match receiver.recv_timeout(ASK_TIMEOUT) {
    Ok(result) if !result.failed => Ok(format!("{} answered:\n{}", target.name, result.text)),
    Ok(result) => Err(format!("{} could not answer: {}", target.name, result.text)),
    Err(_) => Err(format!(
      "{} did not answer within {} minutes; its reply will still appear in the room.",
      target.name,
      ASK_TIMEOUT.as_secs() / 60
    )),
  }
}

fn read_messages(app: &AppHandle, members: &[MemberTerminalConfig], caller: &ToolCaller, arguments: &Value) -> Result<String, String> {
  let limit = arguments
    .get("limit")
    .and_then(Value::as_u64)
    .map(|value| (value as u32).clamp(1, READ_MAX))
    .unwrap_or(READ_DEFAULT);
  let mut messages = chat_get_messages(
    app.state::<ChatDbManager>(),
    caller.workspace_id.clone(),
    caller.conversation_id.clone(),
    Some(limit),
    None,
  )?;
  messages.sort_by_key(|message| message.created_at);
  if messages.is_empty() {
    return Ok("The room has no messages yet.".to_string());
  }
  let name_of = |sender: Option<&String>| match sender {
    Some(id) if id == &caller.member_id => "You".to_string(),
    Some(id) => members.iter().find(|member| &member.id == id).map(|member| member.name.clone()).unwrap_or_else(|| "Human".to_string()),
    None => "Loom".to_string(),
  };
  let lines: Vec<String> = messages
    .iter()
    .filter_map(|message| match &message.content {
      MessageContent::Text { text } => {
        let text: String = text.chars().take(READ_TEXT_LIMIT).collect();
        Some(format!("{}: {text}", name_of(message.sender_id.as_ref())))
      }
      MessageContent::System { .. } => None,
    })
    .collect();
  Ok(lines.join("\n\n"))
}

fn post_message(app: &AppHandle, members: &[MemberTerminalConfig], caller: &ToolCaller, arguments: &Value) -> Result<String, String> {
  let text = str_arg(arguments, "text")?;
  post_as_caller(app, caller, text.to_string())?;
  let runtime = app.state::<Arc<AgentRuntime>>();
  let from = caller_name(members, caller);
  let mut notified = Vec::new();
  let mut skipped = Vec::new();
  for target in mentioned(text, members) {
    if let Err(reason) = check_delegation(caller, target) {
      if target.id != caller.member_id {
        skipped.push(format!("{} ({reason})", target.name));
      }
      continue;
    }
    match delegated_job(&runtime, members, caller, target, format!("{from}: {text}")) {
      Some(job) => {
        runtime.enqueue(app.clone(), job);
        notified.push(target.name.clone());
      }
      None => skipped.push(format!("{} (terminal only)", target.name)),
    }
  }
  let mut summary = "Posted to the room.".to_string();
  if !notified.is_empty() {
    summary.push_str(&format!(" Notified: {}. Their replies will appear in the room.", notified.join(", ")));
  }
  if !skipped.is_empty() {
    summary.push_str(&format!(" Not notified: {}.", skipped.join("; ")));
  }
  Ok(summary)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn member(id: &str, name: &str, kind: &str) -> MemberTerminalConfig {
    MemberTerminalConfig {
      id: id.into(),
      name: name.into(),
      terminal_type: Some(kind.into()),
      terminal_command: None,
      terminal_path: None,
      unlimited_access: false,
    }
  }

  fn caller(member_id: &str, chain: &[&str]) -> ToolCaller {
    ToolCaller {
      member_id: member_id.into(),
      chain: chain.iter().map(|id| id.to_string()).collect(),
      ..ToolCaller::default()
    }
  }

  fn room() -> Vec<MemberTerminalConfig> {
    vec![member("a", "Kai", "antigravity-cli"), member("b", "Ada Lovelace", "claude"), member("s", "Box", "shell")]
  }

  #[test]
  fn resolves_by_id_or_loose_name() {
    let members = room();
    assert_eq!(resolve_target(&members, "b").unwrap().id, "b");
    assert_eq!(resolve_target(&members, "@ada lovelace").unwrap().id, "b");
    assert_eq!(resolve_target(&members, "AdaLovelace").unwrap().id, "b");
    assert!(resolve_target(&members, "Nobody").is_none());
  }

  #[test]
  fn delegation_guards_self_cycles_and_depth() {
    let members = room();
    assert!(check_delegation(&caller("a", &["a"]), &members[0]).is_err());
    assert!(check_delegation(&caller("a", &["b", "a"]), &members[1]).unwrap_err().contains("deadlock"));
    assert!(check_delegation(&caller("a", &["x", "y", "z", "a"]), &members[1]).unwrap_err().contains("limit"));
    assert!(check_delegation(&caller("a", &["a"]), &members[1]).is_ok());
  }

  #[test]
  fn finds_mentions_case_insensitively() {
    let members = room();
    let found: Vec<&str> = mentioned("hey @kai and @Ada Lovelace, review this", &members).iter().map(|m| m.id.as_str()).collect();
    assert_eq!(found, vec!["a", "b"]);
  }

  #[test]
  fn list_marks_who_can_be_asked() {
    let text = list_agents(&room(), &caller("a", &["a"]));
    assert!(text.starts_with("You are Kai (Antigravity)."));
    assert!(text.contains("Ada Lovelace (Claude Code) — can be asked"));
    assert!(text.contains("Box — terminal only"));
  }
}
