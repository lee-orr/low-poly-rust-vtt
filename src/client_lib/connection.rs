use bevy::{prelude::*};

use crate::client_lib::{loading::SettingsAssets, room_info::RoomInfo, settings::Settings, GameState};

pub struct ConnectionPlugin;

impl Plugin for ConnectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(start_matchbox_socket),
        );
    }
}

fn start_matchbox_socket(
    settings_asset: Res<SettingsAssets>,
    settings: Res<Assets<Settings>>,
    room_info: Res<RoomInfo>,
) {
    if let Some(settings) = settings.get(settings_asset.settings.clone()) {
        let url = format!(
            "wss://{}/{}",
            settings.signaling_url,
            room_info.main_channel_name()
        );

        info!("Connecting to matchbox {}", url);
    } else {
        error!("Couldn't connect - no settings found");
    }
}
