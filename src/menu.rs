use crate::loading::FontAssets;
use crate::GameState;
use bevy::prelude::*;


pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app;
    }
}