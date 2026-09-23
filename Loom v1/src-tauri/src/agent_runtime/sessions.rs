//! CLI conversation ids per room + member, so each room keeps its own agent context across turns.

use std::collections::HashMap;
use std::path::PathBuf;

pub(crate) fn session_key(workspace_id: &str, conversation_id: &str, member_id: &str) -> String {
  format!("{workspace_id}/{conversation_id}/{member_id}")
}

pub(crate) struct SessionStore {
  path: PathBuf,
  entries: HashMap<String, String>,
}

impl SessionStore {
  pub(crate) fn load(path: PathBuf) -> Self {
    let entries = std::fs::read_to_string(&path)
      .ok()
      .and_then(|raw| serde_json::from_str(&raw).ok())
      .unwrap_or_default();
    Self { path, entries }
  }

  pub(crate) fn get(&self, key: &str) -> Option<&str> {
    self.entries.get(key).map(String::as_str)
  }

  pub(crate) fn set(&mut self, key: String, session_id: String) {
    if self.entries.get(&key) == Some(&session_id) {
      return;
    }
    self.entries.insert(key, session_id);
    self.persist();
  }

  pub(crate) fn remove(&mut self, key: &str) {
    if self.entries.remove(key).is_some() {
      self.persist();
    }
  }

  fn persist(&self) {
    if let Some(parent) = self.path.parent() {
      let _ = std::fs::create_dir_all(parent);
    }
    let result = serde_json::to_string_pretty(&self.entries)
      .map_err(|err| err.to_string())
      .and_then(|raw| std::fs::write(&self.path, raw).map_err(|err| err.to_string()));
    if let Err(err) = result {
      log::warn!("agent session store write failed path={} err={err}", self.path.display());
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn persists_and_reloads() {
    let path = std::env::temp_dir().join(format!("loom-sessions-{}.json", ulid::Ulid::new()));
    let key = session_key("w", "c", "m");
    let mut store = SessionStore::load(path.clone());
    store.set(key.clone(), "s1".into());
    assert_eq!(SessionStore::load(path.clone()).get(&key), Some("s1"));
    store.remove(&key);
    assert_eq!(SessionStore::load(path.clone()).get(&key), None);
    let _ = std::fs::remove_file(path);
  }
}
