use bevy::prelude::*;

use crate::scenes::SceneState;

pub mod map;
pub mod translate;
pub mod turn_mode;

pub struct IngameUiPlugin;

impl Plugin for IngameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(SceneState::InGameClassicMode),
            (turn_mode::setup, map::setup),
        );

        app.add_systems(
            Update,
            (turn_mode::button_handle_system, map::mouse_handle_system)
                .run_if(in_state(SceneState::InGameClassicMode)),
        );

        app.add_systems(
            OnExit(SceneState::InGameClassicMode),
            (turn_mode::cleanup, map::cleanup),
        );
    }
}
