//! loom-mcp: stdio MCP server that lets CLI agents talk to each other through the running Loom app.
//! Boundary: speaks MCP on stdio and forwards tool calls over Loom's local command pipe; no business logic here.

use std::io::{self, BufRead, BufReader, BufWriter, Write};

use interprocess::local_socket::LocalSocketStream;
use serde::Deserialize;
use serde_json::{json, Value};

use app_lib::mcp_server::{handle_message, ToolBackend, ToolCaller};
use app_lib::COMMAND_IPC_NAME;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct IpcResponse {
  ok: bool,
  result: Option<IpcResult>,
  error: Option<String>,
}

#[derive(Deserialize)]
struct IpcResult {
  message: Option<String>,
}

struct PipeBackend {
  caller: ToolCaller,
}

impl ToolBackend for PipeBackend {
  fn call(&self, tool: &str, arguments: &Value) -> Result<String, String> {
    let mut stream = LocalSocketStream::connect(COMMAND_IPC_NAME)
      .map_err(|err| format!("Loom is not running or not reachable: {err}"))?;
    let request = json!({ "mode": "tool", "tool": tool, "arguments": arguments, "caller": self.caller });
    {
      let mut writer = BufWriter::new(&mut stream);
      writer
        .write_all(request.to_string().as_bytes())
        .and_then(|_| writer.write_all(b"\n"))
        .and_then(|_| writer.flush())
        .map_err(|err| format!("could not send to Loom: {err}"))?;
    }
    let mut line = String::new();
    BufReader::new(&mut stream)
      .read_line(&mut line)
      .map_err(|err| format!("no answer from Loom: {err}"))?;
    let response: IpcResponse =
      serde_json::from_str(line.trim_end()).map_err(|err| format!("bad answer from Loom: {err}"))?;
    if response.ok {
      Ok(response.result.and_then(|result| result.message).unwrap_or_default())
    } else {
      Err(response.error.unwrap_or_else(|| "Loom tool call failed".to_string()))
    }
  }
}

fn main() {
  let backend = ToolCaller::from_env(|key| std::env::var(key).ok()).map(|caller| PipeBackend { caller });
  let backend_ref: Option<&dyn ToolBackend> = backend.as_ref().map(|backend| backend as &dyn ToolBackend);
  let stdin = io::stdin();
  let mut stdout = io::stdout().lock();
  for line in stdin.lock().lines() {
    let Ok(line) = line else { break };
    // Some hosts (e.g. PowerShell pipes) prefix stdin with a UTF-8 BOM.
    let line = line.trim_start_matches('\u{feff}');
    if line.trim().is_empty() {
      continue;
    }
    let reply = match serde_json::from_str::<Value>(line) {
      Ok(message) => handle_message(&message, backend_ref),
      Err(err) => Some(json!({ "jsonrpc": "2.0", "id": null, "error": { "code": -32700, "message": format!("Parse error: {err}") } })),
    };
    if let Some(reply) = reply {
      if writeln!(stdout, "{reply}").and_then(|_| stdout.flush()).is_err() {
        break;
      }
    }
  }
}
