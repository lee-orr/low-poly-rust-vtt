// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::{App, ClearColor, Color, Msaa, WindowDescriptor};
use bevy::DefaultPlugins;
use low_poly_vtt::GamePlugin;

fn main() {
    let mut app = App::new();
    app.insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin);
    app.insert_resource(WindowDescriptor {
        width: 800.,
        height: 600.,
        title: "Low Poly VTT".to_string(), // ToDo
        ..Default::default()
    });

    app.run();
}
