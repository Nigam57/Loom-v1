use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct McpRequest {
    pub method: String,
    pub params: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct McpResponse {
    pub result: serde_json::Value,
}

pub struct LiveChannel {
    pub room_id: String,
}

impl LiveChannel {
    pub fn new(room_id: String) -> Self {
        Self { room_id }
    }

    pub fn handle_request(&self, req: McpRequest) -> McpResponse {
        match req.method.as_str() {
            "post_message" => {
                // Post message to the conversation room
                McpResponse {
                    result: serde_json::json!({ "status": "success" }),
                }
            }
            "read_messages" => {
                // Return latest messages
                McpResponse {
                    result: serde_json::json!({ "messages": [] }),
                }
            }
            "wait_for_reply" => {
                // Suspend until a reply arrives
                McpResponse {
                    result: serde_json::json!({ "status": "success" }),
                }
            }
            _ => {
                McpResponse {
                    result: serde_json::json!({ "error": "Unknown method" }),
                }
            }
        }
    }
}
