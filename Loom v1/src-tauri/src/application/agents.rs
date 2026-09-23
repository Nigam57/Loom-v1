use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// How Loom talks to an agent: a headless structured stream, or the PTY screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AgentAdapterKind {
    AgyStreamJson,
    ClaudeStreamJson,
    OpencodeJson,
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

fn manifest(
    id: &str,
    name: &str,
    command: &str,
    adapter: AgentAdapterKind,
    ready_signal_ms: u64,
) -> AgentManifest {
    AgentManifest {
        id: id.to_string(),
        name: name.to_string(),
        command: command.to_string(),
        args: Vec::new(),
        adapter,
        parsing: AgentManifestParsing {
            prompt_marker: "❯".to_string(),
            bullet_marker: "•".to_string(),
            ready_signal_ms,
        },
    }
}

pub(crate) fn builtin_agents() -> Vec<AgentManifest> {
    vec![
        manifest("antigravity-cli", "Antigravity", "agy", AgentAdapterKind::AgyStreamJson, 500),
        manifest("claude", "Claude Code", "claude", AgentAdapterKind::ClaudeStreamJson, 500),
        manifest("opencode", "OpenCode", "opencode", AgentAdapterKind::OpencodeJson, 500),
        manifest("shell", "Shell", "shell", AgentAdapterKind::Pty, 0),
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
        assert_eq!(find_agent("Claude", None).unwrap().adapter, AgentAdapterKind::ClaudeStreamJson);
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
            r#"{"id":"aider","name":"Aider","command":"aider"}"#,
        )
        .unwrap();
        std::fs::write(root.join(".loom/agents/broken.json"), "{not json").unwrap();

        let agy = find_agent("antigravity-cli", Some(&root)).unwrap();
        assert_eq!(agy.args, vec!["--model".to_string(), "fast".to_string()]);
        assert_eq!(find_agent("aider", Some(&root)).unwrap().adapter, AgentAdapterKind::Pty);
        assert!(find_agent("broken", Some(&root)).is_none());
        let _ = std::fs::remove_dir_all(root);
    }
}
