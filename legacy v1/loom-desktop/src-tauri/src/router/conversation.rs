use crate::adapter::AgentAdapter;
use crate::router::StopReason;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Instant;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub speaker: String,
    pub text: String,
    pub timestamp_ms: u64,
    pub cost_cents: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TurnPolicy {
    RoundRobin,
    Mention,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RoomStatus {
    Idle,
    Running,
    Paused,
    Stopped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationGuards {
    pub max_turns: u32,
    pub timeout_seconds: u64,
    pub budget_cents: u32,
}

impl Default for ConversationGuards {
    fn default() -> Self {
        Self {
            max_turns: 6,
            timeout_seconds: 300,
            budget_cents: 100,
        }
    }
}

#[derive(Clone)]
pub struct Participant {
    pub agent: Arc<dyn AgentAdapter + Send + Sync>,
    pub role_prompt: String,
    pub is_writer: bool,
    pub spawn_config: crate::adapter::SpawnConfig,
}

pub struct ConversationRoom {
    pub id: String,
    pub goal_prompt: String,
    pub participants: Vec<Participant>,
    pub transcript: Vec<Message>,
    pub turn_policy: TurnPolicy,
    pub guards: ConversationGuards,
    pub status: RoomStatus,
    pub start_time: Option<Instant>,
    pub total_cost: u32,
    pub turns: u32,
}

impl ConversationRoom {
    pub fn new(goal_prompt: String, participants: Vec<Participant>, guards: ConversationGuards) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            goal_prompt,
            participants,
            transcript: Vec::new(),
            turn_policy: TurnPolicy::RoundRobin,
            guards,
            status: RoomStatus::Idle,
            start_time: None,
            total_cost: 0,
            turns: 0,
        }
    }

    pub fn check_guards(&self) -> Option<StopReason> {
        if self.turns >= self.guards.max_turns {
            return Some(StopReason::MaxTurns { limit: self.guards.max_turns });
        }
        if self.total_cost > self.guards.budget_cents {
            return Some(StopReason::BudgetExceeded {
                spent: self.total_cost,
                limit: self.guards.budget_cents,
            });
        }
        if let Some(start) = self.start_time {
            if start.elapsed() > std::time::Duration::from_secs(self.guards.timeout_seconds) {
                return Some(StopReason::Timeout { seconds: self.guards.timeout_seconds });
            }
        }
        
        // Loop detector: if the last 3 messages are identical
        if self.transcript.len() >= 3 {
            let n = self.transcript.len();
            let msg1 = &self.transcript[n - 1].text;
            let msg2 = &self.transcript[n - 2].text;
            let msg3 = &self.transcript[n - 3].text;
            if msg1 == msg2 && msg2 == msg3 {
                return Some(StopReason::Error { message: "Loop detected: repeated messages".to_string() });
            }
        }
        
        None
    }

    pub fn append_message(&mut self, speaker: String, text: String, cost_cents: u32) -> Message {
        let timestamp_ms = self.start_time
            .map(|s| s.elapsed().as_millis() as u64)
            .unwrap_or(0);
            
        let msg = Message {
            id: Uuid::new_v4().to_string(),
            speaker,
            text,
            timestamp_ms,
            cost_cents,
        };
        
        self.transcript.push(msg.clone());
        self.total_cost += cost_cents;
        msg
    }
}
