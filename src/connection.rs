use bevy::{asset::Asset, prelude::*, tasks::IoTaskPool};
use matchbox_socket::WebRtcNonBlockingSocket;

use crate::{loading::SettingsAssets, room_info::RoomInfo, settings::Settings, GameState};

pub struct ConnectionPlugin;

impl Plugin for ConnectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(start_matchbox_socket),
        );
    }
}

fn start_matchbox_socket(
    mut commands: Commands,
    settings_asset: Res<SettingsAssets>,
    settings: Res<Assets<Settings>>,
    room_info: Res<RoomInfo>,
    task_pool: Res<IoTaskPool>,
) {
    if let Some(settings) = settings.get(settings_asset.settings.clone()) {
        let url = format!(
            "wss://{}/{}",
            settings.signaling_url,
            room_info.main_channel_name()
        );

        info!("Connecting to matchbox {}", url);

        let (socket, message_loop) = WebRtcNonBlockingSocket::new(url);
        task_pool.spawn(message_loop).detach();
        commands.insert_resource(Some(socket));
    } else {
        error!("Couldn't connect - no settings found");
    }
}
