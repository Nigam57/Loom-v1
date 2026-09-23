use std::process::Command;
use std::fs;

// Testing CLI spawn directly from test script
fn main() {
    let clis = vec![
        ("claude", "--version"),
        ("coder", "--version"),
        ("opencode", "--version"),
        ("agy", "--version"),
        ("hermes", "--version"),
        ("odysseus", "--version"),
    ];

    let mut output_log = String::new();
    
    for (cmd, args) in clis {
        output_log.push_str(&format!("--- Testing {} ---\n", cmd));
        
        let mut resolved = None;
        if let Ok(output) = Command::new("where.exe").arg(cmd).output() {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let mut fallback_path = None;
                for line in stdout.lines() {
                    let path = line.trim().to_string();
                    if path.to_lowercase().ends_with(".exe") || path.to_lowercase().ends_with(".cmd") || path.to_lowercase().ends_with(".bat") {
                        resolved = Some(path);
                        break;
                    }
                    if fallback_path.is_none() && !path.is_empty() {
                        fallback_path = Some(path);
                    }
                }
                if resolved.is_none() && fallback_path.is_some() {
                    resolved = fallback_path;
                }
            }
        }

        if let Some(path) = resolved {
            output_log.push_str(&format!("Found at: {}\n", path));
            
            // If it's a .cmd shim, use cmd.exe /c
            let mut spawn_cmd = Command::new(&path);
            if path.to_lowercase().ends_with(".cmd") || path.to_lowercase().ends_with(".bat") {
                spawn_cmd = Command::new("cmd.exe");
                spawn_cmd.arg("/c").arg(&path).arg(args);
            } else {
                spawn_cmd.arg(args);
            }

            match spawn_cmd.output() {
                Ok(output) => {
                    output_log.push_str(&format!("Status: {}\n", output.status));
                    output_log.push_str(&format!("Stdout: {}\n", String::from_utf8_lossy(&output.stdout).trim()));
                    output_log.push_str(&format!("Stderr: {}\n", String::from_utf8_lossy(&output.stderr).trim()));
                }
                Err(e) => {
                    output_log.push_str(&format!("Failed to spawn: {}\n", e));
                }
            }
        } else {
            output_log.push_str("Not installed on this system.\n");
        }
        output_log.push_str("\n");
    }

    fs::write("docs/evidence/P0-cli-spawn-tests.txt", output_log).unwrap();
}
