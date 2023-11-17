use bevy::prelude::*;

use crate::scenes::SceneState;

pub mod combat_mode;
pub mod map;
pub mod translate;
pub mod turn_mode;
pub mod ui_root;

pub struct IngameUiPlugin;

impl Plugin for IngameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(SceneState::InGameClassicMode),
            (
                map::setup,
                ui_root::setup,
                apply_deferred,
                combat_mode::setup,
                turn_mode::setup,
                apply_deferred,
                map::pathing::setup,
            )
                .chain(),
        );

        app.add_systems(
            Update,
            (
                turn_mode::button_handle_system,
                combat_mode::button_handle_system,
                map::focus_box::mouse_handle_system,
                map::pathing::spawn_move_path,
                // map::pathing::despawn_move_path,
                map::pathing::despawn_on_move,
            )
                .run_if(in_state(SceneState::InGameClassicMode)),
        );

        app.add_systems(OnExit(SceneState::InGameClassicMode), map::cleanup);
    }
}
