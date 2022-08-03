mod actions;
mod audio;
mod client_state;
mod connection;
mod loading;
mod menu;
mod player;
mod room_info;
mod settings;
pub mod web_resize;

use crate::client_lib::actions::ActionsPlugin;
use crate::client_lib::audio::InternalAudioPlugin;
use crate::client_lib::loading::LoadingPlugin;
use crate::client_lib::menu::MenuPlugin;
use crate::client_lib::player::PlayerPlugin;

use crate::client_lib::web_resize::FullViewportPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

use self::client_state::ClientState;
use connection::ConnectionPlugin;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(ClientState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(FullViewportPlugin)
            .add_plugin(ConnectionPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
