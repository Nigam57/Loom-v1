pub mod live_channel;

pub fn detect_mcp_config(cwd: &str) -> Option<String> {
    let cwd_path = std::path::Path::new(cwd).join(".mcp.json");
    if let Ok(content) = std::fs::read_to_string(&cwd_path) {
        return Some(content);
    }
    
    if let Ok(appdata) = std::env::var("APPDATA") {
        let appdata_path = std::path::Path::new(&appdata).join("Claude").join("claude_desktop_config.json");
        if let Ok(content) = std::fs::read_to_string(&appdata_path) {
            return Some(content);
        }
    }

    if let Ok(home) = std::env::var("HOME") {
        let mac_path = std::path::Path::new(&home).join("Library").join("Application Support").join("Claude").join("claude_desktop_config.json");
        if let Ok(content) = std::fs::read_to_string(&mac_path) {
            return Some(content);
        }
    }

    None
}
