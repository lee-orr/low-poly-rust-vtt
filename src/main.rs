// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{
    log::LogSettings,
    prelude::{App, ClearColor, Color, Msaa, WindowDescriptor}, DefaultPlugins,
};

#[cfg(feature = "client")]
use low_poly_vtt::client_lib::ClientPlugin;

#[cfg(feature = "server")]
use low_poly_vtt::server_lib::ServerPlugin;

fn main() {
    let mut app = App::new();

    app.insert_resource(LogSettings {
        filter: "info,wgpu_core=warn,wgpu_hal=warn,server_lib=debug,client_lib=debug".into(),
        level: bevy::log::Level::DEBUG,
    });

    #[cfg(not(feature = "client"))]
    {
        app.add_plugin(bevy::log::LogPlugin::default())
            .add_plugin(bevy::core::CorePlugin::default())
            .add_plugin(bevy::transform::TransformPlugin::default())
            .add_plugin(bevy::hierarchy::HierarchyPlugin::default())
            .add_plugin(bevy::diagnostic::DiagnosticsPlugin::default())
            .add_plugin(bevy::asset::AssetPlugin::default())
            .add_plugin(bevy::app::ScheduleRunnerPlugin::default());
    }

    #[cfg(feature = "client")]
    {
        app.insert_resource(Msaa { samples: 1 })
            .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
            .insert_resource(WindowDescriptor {
                width: 800.,
                height: 600.,
                title: "Low Poly VTT".to_string(), // ToDo
                ..Default::default()
            })
            .add_plugins(DefaultPlugins)
            .add_plugin(ClientPlugin);
    }

    #[cfg(feature = "server")]
    {
        app.add_plugin(ServerPlugin);
    }

    app.run();
}
