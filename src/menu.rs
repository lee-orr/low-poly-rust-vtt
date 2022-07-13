use crate::{room_info::RoomInfo, GameState};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, FontId, RichText},
    EguiContext, EguiPlugin,
};

pub struct MenuPlugin;

#[derive(Default)]
pub struct MainMenuState {
    game_id: String,
}

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup_menu_state))
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(clear_menu_state))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(menu));
    }
}

fn setup_menu_state(mut commands: Commands) {
    commands.init_resource::<MainMenuState>();
}

fn clear_menu_state(mut commands: Commands) {
    commands.remove_resource::<MainMenuState>()
}

fn menu(
    mut egui_context: ResMut<EguiContext>,
    mut menu_state: ResMut<MainMenuState>,
    mut state: ResMut<State<GameState>>,
    mut commands: Commands,
) {
    egui::panel::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        ui.with_layout(egui::Layout::left_to_right(), |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.label(RichText::new("VTT Menu").font(FontId::proportional(90.0)));
                ui.add_space(10.0);
                egui::TextEdit::singleline(&mut menu_state.game_id)
                    .font(FontId::proportional(50.0))
                    .margin([10.0, 10.0].into())
                    .show(ui);
                ui.add_space(10.0);
                if ui
                    .add(egui::Button::new(
                        RichText::new("Start Game").font(FontId::proportional(50.0)),
                    ))
                    .clicked()
                {
                    commands.insert_resource(RoomInfo::new(&menu_state.game_id));
                    let _ = state.set(GameState::Playing);
                }
            });
        });
    });
}
