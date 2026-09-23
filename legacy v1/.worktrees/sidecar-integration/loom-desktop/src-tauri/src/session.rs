use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::path::Path;

use crate::adapter::AgentAdapter;
use crate::pty::manager::PtyManager;

pub struct SessionManager {
    sessions: Mutex<HashMap<String, Arc<dyn AgentAdapter + Send + Sync>>>,
    #[allow(dead_code)] // Reserved for future PTY-level session management
    pty_manager: Arc<std::sync::Mutex<PtyManager>>,
}

impl SessionManager {
    pub fn new(pty_manager: Arc<std::sync::Mutex<PtyManager>>) -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
            pty_manager,
        }
    }

    pub fn register(&self, id: String, adapter: Arc<dyn AgentAdapter + Send + Sync>) {
        self.sessions.lock().unwrap().insert(id, adapter);
    }

    pub fn get(&self, id: &str) -> Option<Arc<dyn AgentAdapter + Send + Sync>> {
        self.sessions.lock().unwrap().get(id).cloned()
    }

    pub fn remove(&self, id: &str) -> Option<Arc<dyn AgentAdapter + Send + Sync>> {
        self.sessions.lock().unwrap().remove(id)
    }

    pub fn validate_cwd(cwd: &str) -> Result<String, String> {
        let path = Path::new(cwd);

        if !path.exists() {
            return Err("Path does not exist".to_string());
        }

        // Resolve symlinks, junctions, ..
        let canonical = path.canonicalize()
            .map_err(|e| format!("Failed to canonicalize path: {}", e))?;
        
        let path_str = canonical.to_string_lossy().to_string();
        let upper_path = path_str.to_uppercase();

        // Check for drive roots (e.g. C:\ or \\?\C:\)
        if canonical.parent().is_none() {
            return Err("Cannot spawn agent in a drive root".to_string());
        }

        // Check for Windows system folders
        if upper_path.contains("C:\\WINDOWS") {
            return Err("Cannot spawn agent in Windows system directory".to_string());
        }
        if upper_path.contains("C:\\PROGRAM FILES") {
            return Err("Cannot spawn agent in Program Files".to_string());
        }

        // Check for user profile directory (allow subdirectories like Documents, but not the profile root itself)
        if let Ok(profile) = std::env::var("USERPROFILE") {
            if let Ok(canon_profile) = Path::new(&profile).canonicalize() {
                let profile_str = canon_profile.to_string_lossy().to_string();
                if path_str == profile_str {
                    return Err("Cannot spawn agent directly in the user profile root".to_string());
                }
            }
        }

        Ok(path_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_cwd_validation_valid() {
        // Use the current temp dir
        let temp_dir = std::env::temp_dir();
        let valid_dir = temp_dir.join("loom_test_valid");
        let _ = fs::create_dir_all(&valid_dir);
        
        let result = SessionManager::validate_cwd(&valid_dir.to_string_lossy());
        assert!(result.is_ok());
        
        let _ = fs::remove_dir_all(&valid_dir);
    }

    #[test]
    fn test_cwd_validation_drive_root() {
        let result = SessionManager::validate_cwd("C:\\");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("drive root"));
    }

    #[test]
    fn test_cwd_validation_windows_dir() {
        let result = SessionManager::validate_cwd("C:\\Windows");
        if Path::new("C:\\Windows").exists() {
            assert!(result.is_err());
            assert!(result.unwrap_err().contains("Windows system directory"));
        }
    }
}
