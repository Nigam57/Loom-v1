use crate::router::conversation::{ConversationGuards, ConversationRoom, Message, Participant};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct RoomManager {
    rooms: Mutex<HashMap<String, Arc<Mutex<ConversationRoom>>>>,
}

impl RoomManager {
    pub fn new() -> Self {
        Self {
            rooms: Mutex::new(HashMap::new()),
        }
    }

    pub fn create_room(&self, goal_prompt: String, guards: ConversationGuards) -> String {
        let room = ConversationRoom::new(goal_prompt, vec![], guards);
        let id = room.id.clone();
        self.rooms.lock().unwrap().insert(id.clone(), Arc::new(Mutex::new(room)));
        id
    }

    pub fn add_participant(&self, room_id: &str, participant: Participant) -> Result<(), String> {
        let mut rooms = self.rooms.lock().unwrap();
        if let Some(room) = rooms.get_mut(room_id) {
            room.lock().unwrap().participants.push(participant);
            Ok(())
        } else {
            Err(format!("Room {} not found", room_id))
        }
    }

    pub fn get_transcript(&self, room_id: &str) -> Result<Vec<Message>, String> {
        let rooms = self.rooms.lock().unwrap();
        if let Some(room) = rooms.get(room_id) {
            Ok(room.lock().unwrap().transcript.clone())
        } else {
            Err(format!("Room {} not found", room_id))
        }
    }

    pub fn get_room(&self, room_id: &str) -> Option<Arc<Mutex<ConversationRoom>>> {
        self.rooms.lock().unwrap().get(room_id).cloned()
    }

    pub fn list_rooms(&self) -> Vec<String> {
        self.rooms.lock().unwrap().keys().cloned().collect()
    }
}
