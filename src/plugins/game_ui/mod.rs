use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

use crate::scenes::SceneState;

pub mod turn_mode;

// #[derive(Resource)]
// pub struct IngameUiData {
//     user_interface_root: Entity,
// }

pub struct IngameUiPlugin;

impl Plugin for IngameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::InGameClassicMode), turn_mode::setup);

        app.add_systems(
            Update,
            turn_mode::button_handle_system.run_if(in_state(SceneState::InGameClassicMode)),
        );

        app.add_systems(OnExit(SceneState::InGameClassicMode), turn_mode::cleanup);
    }
}
