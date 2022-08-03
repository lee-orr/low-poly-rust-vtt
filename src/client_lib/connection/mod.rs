use bevy::prelude::*;
use naia_bevy_client::{Client, ClientConfig, Plugin as NaiaPlugin};

use crate::{
    client_lib::{
        client_state::ClientState, loading::SettingsAssets, room_info::RoomInfo, settings::Settings,
    },
    shared_lib::{
        protocol::{auth::Auth, channels::Channels, Protocol},
        shared_config,
    },
};

pub struct ConnectionPlugin;

impl Plugin for ConnectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(NaiaPlugin::<Protocol, Channels>::new(
            ClientConfig::default(),
            shared_config(),
        ))
        .add_system_set(SystemSet::on_enter(ClientState::Playing).with_system(init));
    }
}

fn init(
    settings_asset: Res<SettingsAssets>,
    settings: Res<Assets<Settings>>,
    room_info: Res<RoomInfo>,
    _commands: Commands,
    mut client: Client<Protocol, Channels>,
) {
    if let Some(settings) = settings.get(settings_asset.settings.clone()) {
        let mut url = settings.signaling_url.clone();

        if let Some(room_url) = &room_info.url {
            url = room_url.clone();
        }

        let mut player = "anonymous".to_owned();

        if let Some(room_player) = &room_info.player {
            player = room_player.clone();
        }

        let game = room_info.room_name.clone();

        info!("Connecting to {} at {} as {}", &game, &url, player);
        client.auth(Auth::new(&player, &game));
        client.connect(&url);
    } else {
        error!("Couldn't connect - no settings found");
    }
}
