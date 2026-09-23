//! Per-member turn queue: one running turn at a time, later messages wait, duplicate message ids drop.

use std::collections::VecDeque;

pub(crate) struct MemberQueue<T> {
  running: bool,
  inflight_id: Option<String>,
  pending: VecDeque<(Option<String>, T)>,
}

impl<T> Default for MemberQueue<T> {
  fn default() -> Self {
    Self { running: false, inflight_id: None, pending: VecDeque::new() }
  }
}

impl<T> MemberQueue<T> {
  /// Returns the item if it should start now; otherwise it waits (or is dropped as a duplicate).
  pub(crate) fn push(&mut self, message_id: Option<String>, item: T) -> Option<T> {
    if let Some(id) = message_id.as_deref() {
      let duplicate = self.inflight_id.as_deref() == Some(id)
        || self.pending.iter().any(|(pending_id, _)| pending_id.as_deref() == Some(id));
      if duplicate {
        return None;
      }
    }
    if self.running {
      self.pending.push_back((message_id, item));
      return None;
    }
    self.running = true;
    self.inflight_id = message_id;
    Some(item)
  }

  /// Call when the running turn ends; returns the next one to start.
  pub(crate) fn finish(&mut self) -> Option<T> {
    match self.pending.pop_front() {
      Some((id, item)) => {
        self.inflight_id = id;
        Some(item)
      }
      None => {
        self.running = false;
        self.inflight_id = None;
        None
      }
    }
  }

  /// Drops waiting turns and hands them back so callers can resolve anyone waiting on them.
  pub(crate) fn take_pending(&mut self) -> Vec<T> {
    self.pending.drain(..).map(|(_, item)| item).collect()
  }

  pub(crate) fn is_running(&self) -> bool {
    self.running
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn first_turn_runs_next_waits_duplicates_drop() {
    let mut queue = MemberQueue::default();
    assert_eq!(queue.push(Some("a".into()), 1), Some(1));
    assert_eq!(queue.push(Some("b".into()), 2), None);
    assert_eq!(queue.push(Some("a".into()), 9), None);
    assert_eq!(queue.push(Some("b".into()), 9), None);
    assert_eq!(queue.finish(), Some(2));
    assert_eq!(queue.finish(), None);
    assert!(!queue.is_running());
    assert_eq!(queue.push(None, 3), Some(3));
  }

  #[test]
  fn take_pending_keeps_running_turn() {
    let mut queue = MemberQueue::default();
    queue.push(None, 1);
    queue.push(None, 2);
    queue.push(None, 3);
    assert_eq!(queue.take_pending(), vec![2, 3]);
    assert!(queue.is_running());
    assert_eq!(queue.finish(), None);
  }
}
