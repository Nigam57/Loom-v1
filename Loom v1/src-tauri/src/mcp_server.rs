//! Loom MCP server protocol (stdio, newline-delimited JSON-RPC 2.0).
//! The `loom-mcp` binary wires this to stdin/stdout and forwards tool calls to the running app.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub const ENV_WORKSPACE_ID: &str = "LOOM_WORKSPACE_ID";
pub const ENV_WORKSPACE_PATH: &str = "LOOM_WORKSPACE_PATH";
pub const ENV_CONVERSATION_ID: &str = "LOOM_CONVERSATION_ID";
pub const ENV_CONVERSATION_TYPE: &str = "LOOM_CONVERSATION_TYPE";
pub const ENV_MEMBER_ID: &str = "LOOM_MEMBER_ID";
pub const ENV_VIEWER_ID: &str = "LOOM_VIEWER_ID";
pub const ENV_CHAIN: &str = "LOOM_CHAIN";

const DEFAULT_PROTOCOL_VERSION: &str = "2025-06-18";

/// Who is calling: taken from the environment Loom sets on every headless turn.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolCaller {
  pub workspace_id: String,
  pub workspace_path: String,
  pub conversation_id: String,
  pub conversation_type: String,
  pub member_id: String,
  pub viewer_id: String,
  /// Members already waiting on this turn, outermost first, ending with the caller.
  pub chain: Vec<String>,
}

impl ToolCaller {
  /// None when the CLI was not started by Loom (e.g. the user runs `agy` in their own terminal).
  pub fn from_env(get: impl Fn(&str) -> Option<String>) -> Option<Self> {
    let member_id = get(ENV_MEMBER_ID).filter(|value| !value.trim().is_empty())?;
    Some(Self {
      workspace_id: get(ENV_WORKSPACE_ID).unwrap_or_default(),
      workspace_path: get(ENV_WORKSPACE_PATH).unwrap_or_default(),
      conversation_id: get(ENV_CONVERSATION_ID).unwrap_or_default(),
      conversation_type: get(ENV_CONVERSATION_TYPE).unwrap_or_else(|| "channel".to_string()),
      viewer_id: get(ENV_VIEWER_ID).unwrap_or_default(),
      chain: get(ENV_CHAIN)
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .map(str::to_string)
        .collect(),
      member_id,
    })
  }
}

pub trait ToolBackend {
  fn call(&self, tool: &str, arguments: &Value) -> Result<String, String>;
}

pub fn tool_definitions() -> Value {
  json!([
    {
      "name": "list_agents",
      "description": "List the other AI agents in this Loom room (name, kind, whether you can ask them) and who you are.",
      "inputSchema": { "type": "object", "properties": {}, "additionalProperties": false }
    },
    {
      "name": "ask_agent",
      "description": "Send another agent in this Loom room a request and wait for its answer. Use it for reviews, second opinions, or work that suits the other agent better. The exchange is shown in the room.",
      "inputSchema": {
        "type": "object",
        "properties": {
          "agent": { "type": "string", "description": "The agent's name as shown by list_agents." },
          "message": { "type": "string", "description": "What you want from that agent. Include the context it needs." }
        },
        "required": ["agent", "message"],
        "additionalProperties": false
      }
    },
    {
      "name": "room_read_messages",
      "description": "Read the most recent messages in this Loom room, oldest first.",
      "inputSchema": {
        "type": "object",
        "properties": { "limit": { "type": "integer", "minimum": 1, "maximum": 100, "description": "How many messages (default 20)." } },
        "additionalProperties": false
      }
    },
    {
      "name": "room_post_message",
      "description": "Post a message to this Loom room without waiting for replies. Mention agents as @Name to notify them.",
      "inputSchema": {
        "type": "object",
        "properties": { "text": { "type": "string" } },
        "required": ["text"],
        "additionalProperties": false
      }
    }
  ])
}

fn response(id: &Value, result: Value) -> Value {
  json!({ "jsonrpc": "2.0", "id": id, "result": result })
}

fn error(id: &Value, code: i64, message: &str) -> Value {
  json!({ "jsonrpc": "2.0", "id": id, "error": { "code": code, "message": message } })
}

/// Handle one JSON-RPC message; `None` for notifications (no reply).
/// `backend` is None when Loom did not launch this process: the server then exposes no tools.
pub fn handle_message(message: &Value, backend: Option<&dyn ToolBackend>) -> Option<Value> {
  let id = message.get("id")?.clone();
  let method = message.get("method").and_then(Value::as_str).unwrap_or("");
  let params = message.get("params").cloned().unwrap_or(Value::Null);
  Some(match method {
    "initialize" => {
      let version = params
        .get("protocolVersion")
        .and_then(Value::as_str)
        .unwrap_or(DEFAULT_PROTOCOL_VERSION);
      response(
        &id,
        json!({
          "protocolVersion": version,
          "capabilities": { "tools": { "listChanged": false } },
          "serverInfo": { "name": "loom", "version": env!("CARGO_PKG_VERSION") },
          "instructions": "Loom connects you with the other AI agents in your room. Call list_agents to see who is here and ask_agent to get their help."
        }),
      )
    }
    "ping" => response(&id, json!({})),
    "tools/list" => {
      let tools = if backend.is_some() { tool_definitions() } else { json!([]) };
      response(&id, json!({ "tools": tools }))
    }
    "tools/call" => {
      let Some(backend) = backend else {
        return Some(error(&id, -32602, "Loom tools only work for agents started by Loom."));
      };
      let Some(name) = params.get("name").and_then(Value::as_str) else {
        return Some(error(&id, -32602, "tools/call needs a tool name"));
      };
      let arguments = params.get("arguments").cloned().unwrap_or_else(|| json!({}));
      let (text, is_error) = match backend.call(name, &arguments) {
        Ok(text) => (text, false),
        Err(message) => (message, true),
      };
      response(&id, json!({ "content": [{ "type": "text", "text": text }], "isError": is_error }))
    }
    _ => error(&id, -32601, &format!("Method not found: {method}")),
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Echo;

  impl ToolBackend for Echo {
    fn call(&self, tool: &str, arguments: &Value) -> Result<String, String> {
      if tool == "fail" {
        return Err("nope".into());
      }
      Ok(format!("{tool}:{arguments}"))
    }
  }

  fn request(method: &str, params: Value) -> Value {
    json!({ "jsonrpc": "2.0", "id": 7, "method": method, "params": params })
  }

  #[test]
  fn initialize_echoes_protocol_version() {
    let reply = handle_message(&request("initialize", json!({ "protocolVersion": "2025-03-26" })), Some(&Echo)).unwrap();
    assert_eq!(reply["id"], 7);
    assert_eq!(reply["result"]["protocolVersion"], "2025-03-26");
    assert_eq!(reply["result"]["serverInfo"]["name"], "loom");
  }

  #[test]
  fn notifications_get_no_reply() {
    let note = json!({ "jsonrpc": "2.0", "method": "notifications/initialized" });
    assert!(handle_message(&note, Some(&Echo)).is_none());
  }

  #[test]
  fn lists_tools_only_when_launched_by_loom() {
    let with = handle_message(&request("tools/list", json!({})), Some(&Echo)).unwrap();
    let names: Vec<&str> = with["result"]["tools"].as_array().unwrap().iter().map(|t| t["name"].as_str().unwrap()).collect();
    assert_eq!(names, vec!["list_agents", "ask_agent", "room_read_messages", "room_post_message"]);
    let without = handle_message(&request("tools/list", json!({})), None).unwrap();
    assert_eq!(without["result"]["tools"], json!([]));
  }

  #[test]
  fn tool_call_returns_text_content_and_error_flag() {
    let ok = handle_message(&request("tools/call", json!({ "name": "ask_agent", "arguments": { "agent": "Ada" } })), Some(&Echo)).unwrap();
    assert_eq!(ok["result"]["content"][0]["text"], r#"ask_agent:{"agent":"Ada"}"#);
    assert_eq!(ok["result"]["isError"], false);
    let failed = handle_message(&request("tools/call", json!({ "name": "fail" })), Some(&Echo)).unwrap();
    assert_eq!(failed["result"]["isError"], true);
  }

  #[test]
  fn unknown_method_is_an_error() {
    let reply = handle_message(&request("resources/list", json!({})), Some(&Echo)).unwrap();
    assert_eq!(reply["error"]["code"], -32601);
  }

  #[test]
  fn caller_from_env_requires_member() {
    let env = |key: &str| match key {
      ENV_MEMBER_ID => Some("m2".to_string()),
      ENV_CHAIN => Some("m1, m2".to_string()),
      ENV_CONVERSATION_ID => Some("c1".to_string()),
      _ => None,
    };
    let caller = ToolCaller::from_env(env).unwrap();
    assert_eq!(caller.chain, vec!["m1".to_string(), "m2".to_string()]);
    assert_eq!(caller.conversation_id, "c1");
    assert!(ToolCaller::from_env(|_| None).is_none());
  }
}
