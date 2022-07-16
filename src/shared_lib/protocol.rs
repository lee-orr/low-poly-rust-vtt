use serde::{Deserialize, Serialize};

use super::chat::ChatMessage;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Message {
    Chat(ChatMessage),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ProtocolMessage {
    pub message: Message,
    pub id: u32,
}
