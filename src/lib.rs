mod actions;
mod audio;
mod loading;
mod menu;
mod player;
mod settings;
pub mod web_resize;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;

use crate::web_resize::FullViewportPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(FullViewportPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}

// fn start_matchbox_socket(mut commands: Commands, task_pool: Res<IoTaskPool>) {
//     let host =
//         web_sys::window()
//         .unwrap()
//         .location()
//         .host()
//         .unwrap()
//         .replace("8080", "3536");
//     let room_url = format!("wss://{}/next_2", host);
//     info!("connecting to matchbox server: {:?}", room_url);
//     let (socket, message_loop) = WebRtcNonBlockingSocket::new(room_url);

//     // The message loop needs to be awaited, or nothing will happen.
//     // We do this here using bevy's task system.
//     task_pool.spawn(message_loop).detach();

//     commands.insert_resource(Some(socket));
// }

// fn wait_for_players(mut socket: ResMut<Option<WebRtcNonBlockingSocket>>) {
//     let socket = socket.as_mut();

//     // If there is no socket we've already started the game
//     if socket.is_none() {
//         return;
//     }

//     // Check for new connections
//     socket.as_mut().unwrap().accept_new_connections();
//     let players = socket.as_ref().unwrap().players();

//     let num_players = 2;
//     if players.len() < num_players {
//         return; // wait for more players
//     }

//     info!("All peers have joined, going in-game");
//     // TODO
// }
