use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use serde_json::{json, Value};
use tauri::{AppHandle, Manager, State, WebviewWindow};

use crate::agent_runtime::{AgentRuntime, ChatTurnContext, TurnJob};
use crate::application::agents::{find_agent, AgentAdapterKind, AgentManifest};
use crate::contracts::chat_dispatch::{ChatDispatchMentions, ChatDispatchPayload};
use crate::message_service::chat_db::{chat_get_conversation_member_ids, ChatDbManager};
use crate::message_service::project_data;
use crate::platform::{diagnostics_log_backend_event, DiagnosticsState};
use crate::runtime::StorageManager;
use crate::terminal_engine::session::{terminal_create, TerminalDispatchContext, TerminalManager};
use super::chat_dispatch_batcher::ChatDispatchBatcher;


#[derive(Clone, Debug)]
pub(crate) struct MemberTerminalConfig {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) terminal_type: Option<String>,
    pub(crate) terminal_command: Option<String>,
    pub(crate) terminal_path: Option<String>,
    pub(crate) unlimited_access: bool,
}


/// 发送聊天消息后，按 mention 规则编排派发到终端。
pub fn orchestrate_chat_dispatch(
    app: &AppHandle,
    window: &WebviewWindow,
    terminal_state: State<'_, TerminalManager>,
    chat_state: State<'_, ChatDbManager>,
    storage: &StorageManager,
    payload: ChatDispatchPayload,
) -> Result<(), String> {
    let member_ids = chat_get_conversation_member_ids(
        chat_state,
        payload.workspace_id.clone(),
        payload.conversation_id.clone(),
    )?;
    let member_set: HashSet<String> = member_ids.iter().cloned().collect();
    if member_set.is_empty() {
        log_chat_dispatch_skip(
            app,
            &payload,
            "no_conversation_members",
            json!({
                "memberCount": member_ids.len()
            }),
        );
        return Ok(());
    }

    let project_data = project_data::read_project_data(
        storage,
        &payload.workspace_path,
        &payload.workspace_id,
    )?;
    let member_configs = match project_data.data {
        Some(value) => collect_member_configs(&value),
        None => HashMap::new(),
    };
    let member_config_count = member_configs.len();

    let default_mentions = ChatDispatchMentions {
        mention_ids: Vec::new(),
        mention_all: false,
    };
    let mentions = payload.mentions.as_ref().unwrap_or(&default_mentions);
    let mut targets = resolve_targets(
        payload.conversation_type.as_str(),
        &member_ids,
        &member_set,
        payload.sender_id.as_str(),
        mentions,
    );
    if targets.is_empty() {
        log_chat_dispatch_skip(
            app,
            &payload,
            "no_targets",
            json!({
                "memberCount": member_ids.len()
            }),
        );
        return Ok(());
    }
    let targets_before_filter = targets.len();
    targets.retain(|id| member_configs.contains_key(id));
    if targets.is_empty() {
        log_chat_dispatch_skip(
            app,
            &payload,
            "targets_missing_config",
            json!({
                "targetsBeforeFilter": targets_before_filter,
                "memberConfigCount": member_config_count
            }),
        );
        return Ok(());
    }

    let context = TerminalDispatchContext {
        conversation_id: payload.conversation_id.clone(),
        conversation_type: payload.conversation_type.clone(),
        sender_id: payload.sender_id.clone(),
        sender_name: payload.sender_name.clone(),
        message_id: payload.message_id.clone(),
        client_trace_id: payload.client_trace_id.clone(),
        client_timestamp: payload.timestamp,
    };

    let batcher = app.state::<Arc<ChatDispatchBatcher>>();
    let mut dispatched_count = 0usize;
    let mut skipped_missing_terminal_config = 0usize;
    for target_id in targets {
        let Some(config) = member_configs.get(&target_id) else {
            continue;
        };
        if !has_terminal_config(config) {
            skipped_missing_terminal_config = skipped_missing_terminal_config.saturating_add(1);
            continue;
        }
        if let Some(manifest) = headless_manifest(config, &payload.workspace_path) {
            let runtime = app.state::<Arc<AgentRuntime>>();
            let prompt = headless_prompt(
                &runtime,
                &payload,
                config,
                &member_ids,
                &member_configs,
            );
            runtime.enqueue(
                app.clone(),
                TurnJob {
                    manifest,
                    cwd: PathBuf::from(&payload.workspace_path),
                    prompt,
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
                    chain: Vec::new(),
                    on_complete: None,
                },
            );
            dispatched_count = dispatched_count.saturating_add(1);
            continue;
        }
        let terminal_id = ensure_backend_member_session(
            app,
            window,
            &terminal_state,
            config,
            &payload.workspace_id,
            &payload.workspace_path,
        )?;
        batcher.enqueue_for_terminal(app, terminal_id, payload.text.clone(), context.clone())?;
        dispatched_count = dispatched_count.saturating_add(1);
    }
    if dispatched_count == 0 {
        log_chat_dispatch_skip(
            app,
            &payload,
            "targets_no_terminal_config",
            json!({
                "targetsBeforeFilter": targets_before_filter,
                "memberConfigCount": member_config_count,
                "skippedMissingTerminalConfig": skipped_missing_terminal_config
            }),
        );
    }
    Ok(())
}

fn resolve_targets(
    conversation_type: &str,
    member_ids: &[String],
    member_set: &HashSet<String>,
    sender_id: &str,
    mentions: &ChatDispatchMentions,
) -> Vec<String> {
    let mut targets = Vec::new();
    if conversation_type == "dm" {
        if let Some(target_id) = member_ids.iter().find(|id| id.as_str() != sender_id) {
            if member_set.contains(target_id) {
                targets.push(target_id.clone());
            }
        }
        return targets;
    }

    if mentions.mention_all {
        targets.extend(member_ids.iter().cloned());
    } else {
        targets.extend(mentions.mention_ids.iter().cloned());
    }
    
    // Fallback: If no explicit mentions were made, dispatch to all non-sender
    // members so CLI terminals receive the message without requiring @mentions.
    if targets.is_empty() {
        for target_id in member_ids.iter() {
            if target_id.as_str() != sender_id && member_set.contains(target_id) {
                targets.push(target_id.clone());
            }
        }
    }

    let mut unique = HashSet::new();
    targets
        .into_iter()
        .filter(|id| id.as_str() != sender_id)
        .filter(|id| member_set.contains(id))
        .filter(|id| unique.insert(id.clone()))
        .collect()
}

pub(crate) fn collect_member_configs(payload: &Value) -> HashMap<String, MemberTerminalConfig> {
    let mut map = HashMap::new();
    let Some(members) = payload.get("members").and_then(|value| value.as_array()) else {
        return map;
    };
    for member in members {
        let Some(obj) = member.as_object() else {
            continue;
        };
        let id = obj
            .get("id")
            .and_then(|value| value.as_str())
            .unwrap_or("")
            .trim();
        if id.is_empty() {
            continue;
        }
        let name = obj
            .get("name")
            .and_then(|value| value.as_str())
            .unwrap_or("")
            .trim();
        let terminal_type = normalize_string(obj.get("terminalType"));
        let terminal_command = normalize_string(obj.get("terminalCommand"));
        let terminal_path = normalize_string(obj.get("terminalPath"));
        let unlimited_access = obj
            .get("unlimitedAccess")
            .and_then(|value| value.as_bool())
            .unwrap_or(false);
        map.insert(
            id.to_string(),
            MemberTerminalConfig {
                id: id.to_string(),
                name: name.to_string(),
                terminal_type,
                terminal_command,
                terminal_path,
                unlimited_access,
            },
        );
    }
    map
}

fn normalize_string(value: Option<&Value>) -> Option<String> {
    value
        .and_then(|value| value.as_str())
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .map(|value| value.to_string())
}

fn has_terminal_config(config: &MemberTerminalConfig) -> bool {
    config
        .terminal_type
        .as_ref()
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false)
        || config
            .terminal_command
            .as_ref()
            .map(|value| !value.trim().is_empty())
            .unwrap_or(false)
        || config
            .terminal_path
            .as_ref()
            .map(|value| !value.trim().is_empty())
            .unwrap_or(false)
}

/// Headless adapter for this member, unless it launches through a custom command (those keep the PTY path).
pub(crate) fn headless_manifest(config: &MemberTerminalConfig, workspace_path: &str) -> Option<AgentManifest> {
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

/// "Name (Claude Code)" for agents, plain name for people.
pub(crate) fn member_label(config: &MemberTerminalConfig) -> String {
    match config.terminal_type.as_deref().and_then(|kind| find_agent(kind, None)) {
        Some(manifest) if manifest.id != "shell" => {
            format!("{} ({})", config.name, manifest.name)
        }
        _ => config.name.clone(),
    }
}

/// First-turn briefing: who is in the room and how to reach them.
pub(crate) fn room_briefing(
    me: &MemberTerminalConfig,
    others: &[&MemberTerminalConfig],
    has_tools: bool,
) -> String {
    let roster = if others.is_empty() {
        "no other agents".to_string()
    } else {
        others.iter().map(|member| member_label(member)).collect::<Vec<_>>().join(", ")
    };
    let mut text = format!(
        "[Loom] You are \"{}\", one of several AI agents working with a human in a shared Loom room. \
Other agents in this room: {roster}. Messages from others are prefixed with the sender's name.",
        me.name
    );
    if has_tools {
        text.push_str(
            " You have Loom tools on the MCP server \"loom\": list_agents, ask_agent (send another agent a \
request and get its answer), room_read_messages and room_post_message. When another agent's skills, \
review or second opinion would make your answer better, use ask_agent instead of guessing.",
        );
    }
    text
}

fn headless_prompt(
    runtime: &AgentRuntime,
    payload: &ChatDispatchPayload,
    me: &MemberTerminalConfig,
    member_ids: &[String],
    member_configs: &HashMap<String, MemberTerminalConfig>,
) -> String {
    let message = if payload.conversation_type == "channel" && !payload.sender_name.trim().is_empty() {
        format!("{}: {}", payload.sender_name.trim(), payload.text)
    } else {
        payload.text.clone()
    };
    if !runtime.is_new_session(&payload.workspace_id, &payload.conversation_id, &me.id) {
        return message;
    }
    let others: Vec<&MemberTerminalConfig> = member_ids
        .iter()
        .filter(|id| id.as_str() != me.id && id.as_str() != payload.sender_id)
        .filter_map(|id| member_configs.get(id))
        .filter(|member| headless_manifest(member, &payload.workspace_path).is_some())
        .collect();
    format!("{}\n\n{message}", room_briefing(me, &others, runtime.has_mcp()))
}

fn ensure_backend_member_session(
    app: &AppHandle,
    window: &WebviewWindow,
    state: &State<'_, TerminalManager>,
    config: &MemberTerminalConfig,
    workspace_id: &str,
    workspace_path: &str,
) -> Result<String, String> {
    if let Some(session_id) =
        state.find_session_id_by_member(config.id.as_str(), Some(workspace_id))
    {
        return Ok(session_id);
    }

    terminal_create(
        app.clone(),
        window.clone(),
        state.clone(),
        None,
        None,
        Some(workspace_path.to_string()),
        Some(config.id.clone()),
        Some(workspace_id.to_string()),
        Some(true),
        None,
        config.terminal_type.clone(),
        config.terminal_command.clone(),
        config.terminal_path.clone(),
        None,
        Some("none".to_string()),
        Some(config.name.clone()),
        None,
        None,
        None,
        None,
    )
}

fn log_chat_dispatch_skip(
    app: &AppHandle,
    payload: &ChatDispatchPayload,
    reason: &str,
    detail: Value,
) {
    let mentions = payload.mentions.as_ref();
    let mention_all = mentions.map(|value| value.mention_all).unwrap_or(false);
    let mention_ids_len = mentions.map(|value| value.mention_ids.len()).unwrap_or(0);
    diagnostics_log_backend_event(
        &app.state::<DiagnosticsState>(),
        Some(payload.sender_id.clone()),
        None,
        Some(payload.conversation_id.clone()),
        None,
        Some(payload.workspace_id.clone()),
        "chat_dispatch_skip",
        json!({
            "reason": reason,
            "conversationType": payload.conversation_type,
            "mentionAll": mention_all,
            "mentionIdsLen": mention_ids_len,
            "detail": detail
        }),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config(id: &str, terminal_type: &str, command: Option<&str>) -> MemberTerminalConfig {
        MemberTerminalConfig {
            id: id.into(),
            name: id.to_uppercase(),
            terminal_type: Some(terminal_type.into()),
            terminal_command: command.map(str::to_string),
            terminal_path: None,
            unlimited_access: false,
        }
    }

    #[test]
    fn agy_and_claude_members_go_headless() {
        assert!(headless_manifest(&config("a", "antigravity-cli", None), "").is_some());
        assert!(headless_manifest(&config("a", "antigravity-cli", Some("agy --dangerously-skip-permissions")), "").is_some());
        assert!(headless_manifest(&config("c", "claude", Some("claude")), "").is_some());
    }

    #[test]
    fn custom_command_or_pty_type_stays_on_pty() {
        assert!(headless_manifest(&config("a", "antigravity-cli", Some("wsl agy")), "").is_none());
        assert!(headless_manifest(&config("s", "shell", None), "").is_none());
        assert!(headless_manifest(&config("g", "gemini", None), "").is_none());
    }

    #[test]
    fn briefing_lists_other_agents_and_tools() {
        let me = config("kai", "antigravity-cli", None);
        let ada = config("ada", "claude", None);
        let text = room_briefing(&me, &[&ada], true);
        assert!(text.contains("You are \"KAI\""));
        assert!(text.contains("ADA (Claude Code)"));
        assert!(text.contains("ask_agent"));
        assert!(!room_briefing(&me, &[], false).contains("ask_agent"));
    }
}
