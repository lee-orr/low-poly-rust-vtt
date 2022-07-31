use bevy::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::shared_lib::protocol::Protocol"]
pub struct Auth {
    pub player: Property<String>,
    pub game: Property<String>,
}

impl Auth {
    pub fn new(player: &str, game: &str) -> Self {
        Auth::new_complete(player.to_string(), game.to_string())
    }
}
