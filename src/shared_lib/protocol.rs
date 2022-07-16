use serde::{Deserialize, Serialize};

use super::chat::ChatMessage;

#[derive(Clone, Serialize, Deserialize)]
pub enum Message {
    Chat(ChatMessage),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ProtocolMessage {
    pub message: Message,
    pub id: u32,
}
