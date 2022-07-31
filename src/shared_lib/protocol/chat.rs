use bevy::prelude::Component;
use naia_shared::{Property, Replicate};

use crate::shared_lib::chat::ChatMessage;

#[derive(Component, Replicate)]
#[protocol_path = "crate::shared_lib::protocol::Protocol"]
pub struct Chat {
    pub message: Property<ChatMessage>,
}

impl Chat {
    pub fn new(msg: ChatMessage) -> Self {
        Chat::new_complete(msg)
    }
}
