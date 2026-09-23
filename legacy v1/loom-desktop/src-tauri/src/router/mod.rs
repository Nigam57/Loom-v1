pub mod conversation;

use crate::adapter::{AgentAdapter, AgentEvent, AgentMessage, SpawnConfig};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// A2A Router configuration with safety guards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouterConfig {
    pub max_turns: u32,
    pub timeout_seconds: u64,
    pub budget_cents: u32,
}

impl Default for RouterConfig {
    fn default() -> Self {
        Self {
            max_turns: 5,
            timeout_seconds: 300,
            budget_cents: 100,
        }
    }
}

/// An event in the router trace log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceEvent {
    pub timestamp_ms: u64,
    pub event_type: String,
    pub agent: String,
    pub detail: String,
}

/// Reason the router stopped
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StopReason {
    Completed,
    MaxTurns { limit: u32 },
    Timeout { seconds: u64 },
    BudgetExceeded { spent: u32, limit: u32 },
    Error { message: String },
    NeedsApproval { next_agent: String, proposed_message: String },
    Cancelled,
}

/// Result of a routing session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouterResult {
    pub stop_reason: StopReason,
    pub turns_used: u32,
    pub total_cost_cents: u32,
    pub trace: Vec<TraceEvent>,
}

/// The A2A message router with loop, timeout, and budget guards
pub struct Router {
    config: RouterConfig,
    trace: Vec<TraceEvent>,
    start_time: Option<Instant>,
    total_cost: u32,
    turns: u32,
}

impl Router {
    pub fn new(config: RouterConfig) -> Self {
        Self {
            config,
            trace: Vec::new(),
            start_time: None,
            total_cost: 0,
            turns: 0,
        }
    }

    fn log(&mut self, event_type: &str, agent: &str, detail: &str) {
        let elapsed = self.start_time
            .map(|s| s.elapsed().as_millis() as u64)
            .unwrap_or(0);
        self.trace.push(TraceEvent {
            timestamp_ms: elapsed,
            event_type: event_type.to_string(),
            agent: agent.to_string(),
            detail: detail.to_string(),
        });
    }

    fn check_guards(&self) -> Option<StopReason> {
        if self.turns >= self.config.max_turns {
            return Some(StopReason::MaxTurns { limit: self.config.max_turns });
        }
        if self.total_cost > self.config.budget_cents {
            return Some(StopReason::BudgetExceeded {
                spent: self.total_cost,
                limit: self.config.budget_cents,
            });
        }
        if let Some(start) = self.start_time {
            if start.elapsed() > Duration::from_secs(self.config.timeout_seconds) {
                return Some(StopReason::Timeout { seconds: self.config.timeout_seconds });
            }
        }
        None
    }

    /// Run a two-agent handoff loop
    pub async fn run_handoff(
        &mut self,
        agent_a: &dyn AgentAdapter,
        agent_b: &dyn AgentAdapter,
        initial_prompt: &str,
        spawn_a: SpawnConfig,
        spawn_b: SpawnConfig,
    ) -> RouterResult {
        self.start_time = Some(Instant::now());
        self.turns = 0;
        self.total_cost = 0;
        self.trace.clear();

        let prompt_preview = &initial_prompt[..initial_prompt.len().min(100)];
        self.log("start", "router", &format!("Starting handoff: {}", prompt_preview));

        // Spawn both agents
        if let Err(e) = agent_a.spawn(spawn_a).await {
            let msg = e.to_string();
            self.log("error", agent_a.name(), &msg);
            return self.make_result(StopReason::Error { message: msg });
        }
        if let Err(e) = agent_b.spawn(spawn_b).await {
            let msg = e.to_string();
            self.log("error", agent_b.name(), &msg);
            return self.make_result(StopReason::Error { message: msg });
        }

        let mut current_message = initial_prompt.to_string();
        let mut current_is_a = true;

        loop {
            if let Some(reason) = self.check_guards() {
                self.log("guard", "router", &format!("Stopped: {:?}", reason));
                return self.make_result(reason);
            }

            self.turns += 1;
            let msg_preview = &current_message[..current_message.len().min(80)];

            if current_is_a {
                self.log("send", agent_a.name(), &format!("Turn {}: {}", self.turns, msg_preview));
                if let Err(e) = agent_a.send_message(AgentMessage {
                    content: current_message.clone(),
                    sender: "router".to_string(),
                }).await {
                    let msg = e.to_string();
                    self.log("error", agent_a.name(), &msg);
                    return self.make_result(StopReason::Error { message: msg });
                }
            } else {
                self.log("send", agent_b.name(), &format!("Turn {}: {}", self.turns, msg_preview));
                if let Err(e) = agent_b.send_message(AgentMessage {
                    content: current_message.clone(),
                    sender: agent_a.name().to_string(),
                }).await {
                    let msg = e.to_string();
                    self.log("error", agent_b.name(), &msg);
                    return self.make_result(StopReason::Error { message: msg });
                }
            }

            tokio::time::sleep(Duration::from_millis(100)).await;

            let mut response_text = String::new();
            let mut got_done = false;
            let current_name = if current_is_a { agent_a.name() } else { agent_b.name() };

            for _ in 0..50 {
                let event_result = if current_is_a {
                    agent_a.recv_event().await
                } else {
                    agent_b.recv_event().await
                };

                match event_result {
                    Ok(AgentEvent::Output { text }) => {
                        response_text.push_str(&text);
                    }
                    Ok(AgentEvent::Result { content, cost_cents }) => {
                        response_text = content;
                        self.total_cost += cost_cents;
                    }
                    Ok(AgentEvent::Done { cost_cents }) => {
                        self.total_cost += cost_cents;
                        self.log("done", current_name, &format!("cost={}c", self.total_cost));
                        got_done = true;
                        break;
                    }
                    Ok(AgentEvent::Error { message }) => {
                        self.log("error", current_name, &message);
                        return self.make_result(StopReason::Error { message });
                    }
                    Ok(AgentEvent::ApprovalRequest { .. }) => {}
                    Err(_) => {
                        tokio::time::sleep(Duration::from_millis(50)).await;
                    }
                }
            }

            if let Some(reason) = self.check_guards() {
                self.log("guard", "router", &format!("Stopped: {:?}", reason));
                return self.make_result(reason);
            }

            if got_done && response_text.is_empty() {
                return self.make_result(StopReason::Completed);
            }

            if response_text.is_empty() {
                return self.make_result(StopReason::Error {
                    message: format!("{} no response", current_name),
                });
            }

            current_message = response_text;
            current_is_a = !current_is_a;
        }
    }

    pub async fn run_conversation(
        &mut self,
        room_arc: std::sync::Arc<std::sync::Mutex<conversation::ConversationRoom>>,
        app_handle: Option<tauri::AppHandle>,
    ) -> RouterResult {
        self.start_time = Some(Instant::now());
        self.turns = 0;
        self.total_cost = 0;
        self.trace.clear();

        let participants = {
            let mut room = room_arc.lock().unwrap();
            room.start_time = Some(Instant::now());
            if room.participants.is_empty() {
                return self.make_result(StopReason::Error { message: "No participants in room".to_string() });
            }
            room.participants.clone()
        };

        // Spawn all agents if they haven't been spawned
        for p in &participants {
            if let Err(e) = p.agent.spawn(p.spawn_config.clone()).await {
                let msg = format!("Failed to spawn agent {}: {}", p.agent.name(), e);
                self.log("error", p.agent.name(), &msg);
                return self.make_result(StopReason::Error { message: msg });
            }
        }

        let mut current_speaker_idx = 0;
        let mut last_speaker = "User".to_string();
        let mut pending_message;

        {
            let room = room_arc.lock().unwrap();
            pending_message = room.goal_prompt.clone();
            
            // If the transcript is not empty, resume from last state
            if let Some(last_msg) = room.transcript.last() {
                pending_message = last_msg.text.clone();
                last_speaker = last_msg.speaker.clone();
                // TODO: adjust current_speaker_idx based on turn policy
            }
        }

        loop {
            let (room_id, is_paused) = {
                let room = room_arc.lock().unwrap();
                if let Some(reason) = room.check_guards() {
                    self.log("guard", "router", &format!("Stopped: {:?}", reason));
                    return self.make_result(reason);
                }
                (room.id.clone(), room.status == conversation::RoomStatus::Paused)
            };

            if is_paused {
                tokio::time::sleep(Duration::from_millis(500)).await;
                continue;
            }

            self.turns += 1;
            {
                let mut room = room_arc.lock().unwrap();
                room.turns += 1;
            }
            
            let agent = participants[current_speaker_idx].agent.clone();
            
            // Check HITL Approval (assume true for now if supports_approval)
            if agent.capabilities().supports_approval {
                // Return NeedsApproval so UI can intercept and resume
                return self.make_result(StopReason::NeedsApproval {
                    next_agent: agent.name().to_string(),
                    proposed_message: pending_message,
                });
            }

            self.log("send", agent.name(), &format!("Turn {}: {}", self.turns, pending_message));
            
            // Format message based on ContextStrategy
            let msg_to_send = if agent.capabilities().context_strategy == crate::adapter::ContextStrategy::Stateless {
                // In stateless, we should really send the whole transcript, but for now just send the pending message
                // A full implementation would format room.transcript into a single prompt.
                format!("Speaker {}:\n{}", last_speaker, pending_message)
            } else {
                pending_message.clone()
            };

            if let Err(e) = agent.send_message(AgentMessage {
                content: msg_to_send,
                sender: last_speaker.clone(),
            }).await {
                let msg = e;
                self.log("error", agent.name(), &msg);
                return self.make_result(StopReason::Error { message: msg });
            }

            tokio::time::sleep(Duration::from_millis(100)).await;

            let mut response_text = String::new();
            let mut got_done = false;
            let current_name = agent.name();

            for _ in 0..100 {
                match agent.recv_event().await {
                    Ok(AgentEvent::Output { text }) => {
                        response_text.push_str(&text);
                    }
                    Ok(AgentEvent::Result { content, cost_cents }) => {
                        response_text = content;
                        self.total_cost += cost_cents;
                    }
                    Ok(AgentEvent::Done { cost_cents }) => {
                        self.total_cost += cost_cents;
                        self.log("done", current_name, &format!("cost={}c", self.total_cost));
                        got_done = true;
                        break;
                    }
                    Ok(AgentEvent::Error { message }) => {
                        self.log("error", current_name, &message);
                        return self.make_result(StopReason::Error { message });
                    }
                    Ok(AgentEvent::ApprovalRequest { id: _, tool, args }) => {
                        self.log("approval", current_name, &format!("Tool: {}, Args: {}", tool, args));
                        // For MVP, we just auto-approve or ignore. Let's ignore it for now.
                    }
                    Err(_) => {
                        tokio::time::sleep(Duration::from_millis(50)).await;
                    }
                }
            }

            if let Some(reason) = { room_arc.lock().unwrap().check_guards() } {
                self.log("guard", "router", &format!("Stopped: {:?}", reason));
                return self.make_result(reason);
            }

            if response_text.trim() == "DONE" {
                let msg = {
                    let mut room = room_arc.lock().unwrap();
                    room.append_message(current_name.to_string(), response_text, 0)
                };
                use tauri::Emitter;
                if let Some(app) = &app_handle {
                    let _ = app.emit("room://transcript-update", serde_json::json!({
                        "room_id": room_id,
                        "message": msg
                    }));
                }
                return self.make_result(StopReason::Completed);
            }

            if got_done && response_text.is_empty() {
                return self.make_result(StopReason::Completed);
            }

            if response_text.is_empty() {
                return self.make_result(StopReason::Error {
                    message: format!("{} no response", current_name),
                });
            }

            let msg = {
                let mut room = room_arc.lock().unwrap();
                room.append_message(current_name.to_string(), response_text.clone(), 0)
            };
            use tauri::Emitter;
            if let Some(app) = &app_handle {
                let _ = app.emit("room://transcript-update", serde_json::json!({
                    "room_id": room_id,
                    "message": msg
                }));
            }

            pending_message = response_text;
            last_speaker = current_name.to_string();
            
            // Round-robin
            current_speaker_idx = (current_speaker_idx + 1) % participants.len();
        }
    }

    fn make_result(&self, reason: StopReason) -> RouterResult {
        RouterResult {
            stop_reason: reason,
            turns_used: self.turns,
            total_cost_cents: self.total_cost,
            trace: self.trace.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapter::fake_cli::FakeCliAdapter;
    use std::collections::HashMap;

    fn default_spawn() -> SpawnConfig {
        SpawnConfig {
            cmd: "fake".to_string(),
            args: vec![],
            env: HashMap::new(),
            cwd: ".".to_string(),
        }
    }

    #[tokio::test]
    async fn test_max_turns_guard() {
        let agent_a = FakeCliAdapter::echo("agent-a");
        let agent_b = FakeCliAdapter::echo("agent-b");

        let mut router = Router::new(RouterConfig {
            max_turns: 3,
            timeout_seconds: 60,
            budget_cents: 1000,
        });

        let result = router.run_handoff(
            &agent_a, &agent_b,
            "start the loop",
            default_spawn(), default_spawn(),
        ).await;

        assert!(matches!(result.stop_reason, StopReason::MaxTurns { limit: 3 }));
        assert!(result.turns_used <= 3);
        assert!(!result.trace.is_empty());
    }

    #[tokio::test]
    async fn test_scripted_handoff() {
        let agent_a = FakeCliAdapter::scripted("agent-a", vec!["result from A".into()], 5);
        let agent_b = FakeCliAdapter::scripted("agent-b", vec![], 3);

        let mut router = Router::new(RouterConfig {
            max_turns: 10,
            timeout_seconds: 60,
            budget_cents: 1000,
        });

        let result = router.run_handoff(
            &agent_a, &agent_b,
            "do something",
            default_spawn(), default_spawn(),
        ).await;

        assert!(result.turns_used > 0);
        assert!(!result.trace.is_empty());
    }

    use crate::router::conversation::{ConversationRoom, ConversationGuards, Participant};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_transcript_persistence() {
        let mut room = ConversationRoom::new("Goal".to_string(), vec![], ConversationGuards::default());
        room.append_message("agent-a".to_string(), "message 1".to_string(), 10);
        assert_eq!(room.transcript.len(), 1);
        assert_eq!(room.total_cost, 10);
    }

    #[tokio::test]
    async fn test_loop_detector() {
        let mut room = ConversationRoom::new("Goal".to_string(), vec![], ConversationGuards::default());
        room.append_message("agent-a".to_string(), "Looks good".to_string(), 0);
        room.append_message("agent-b".to_string(), "Looks good".to_string(), 0);
        room.append_message("agent-a".to_string(), "Looks good".to_string(), 0);
        
        if let Some(StopReason::Error { message }) = room.check_guards() {
            assert!(message.contains("Loop detected"));
        } else {
            panic!("Loop detector failed to fire");
        }
    }

    #[tokio::test]
    async fn test_guard_cutoff() {
        let agent = Arc::new(FakeCliAdapter::echo("agent-loop"));
        let room = ConversationRoom::new(
            "Looping".to_string(),
            vec![Participant {
                agent,
                role_prompt: "".to_string(),
                is_writer: false,
                spawn_config: default_spawn(),
            }],
            ConversationGuards { max_turns: 2, timeout_seconds: 60, budget_cents: 1000 },
        );

        let mut router = Router::new(RouterConfig::default());
        let result = router.run_conversation(Arc::new(std::sync::Mutex::new(room)), None).await;
        
        assert!(matches!(result.stop_reason, StopReason::MaxTurns { limit: 2 }));
    }
    #[tokio::test]
    async fn test_fakecli_dialogue() {
        let agent_a = Arc::new(FakeCliAdapter::echo("agent-a"));
        let agent_b = Arc::new(FakeCliAdapter::echo("agent-b"));
        
        let room = ConversationRoom::new(
            "Hello".to_string(),
            vec![
                Participant { agent: agent_a, role_prompt: "".to_string(), is_writer: false, spawn_config: default_spawn() },
                Participant { agent: agent_b, role_prompt: "".to_string(), is_writer: false, spawn_config: default_spawn() },
            ],
            ConversationGuards { max_turns: 4, timeout_seconds: 60, budget_cents: 1000 },
        );

        let room_arc = Arc::new(std::sync::Mutex::new(room));
        let guards = {
            let room = room_arc.lock().unwrap();
            room.guards.clone()
        };
        let mut router = Router::new(RouterConfig {
            max_turns: guards.max_turns,
            timeout_seconds: guards.timeout_seconds,
            budget_cents: guards.budget_cents,
        });
        let result = router.run_conversation(room_arc.clone(), None).await;
        
        assert!(matches!(result.stop_reason, StopReason::MaxTurns { limit: 4 }));
        assert_eq!(room_arc.lock().unwrap().transcript.len(), 3);
    }
}
