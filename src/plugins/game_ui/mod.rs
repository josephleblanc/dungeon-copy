use bevy::prelude::*;

use crate::{
    config::{RESOLUTION, TILE_SIZE, WINDOW_HEIGHT},
    scenes::SceneState,
};

pub mod map;
pub mod turn_mode;

// #[derive(Resource)]
// pub struct IngameUiData {
//     user_interface_root: Entity,
// }

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

pub fn cartesian_to_ui(x_in: f32, y_in: f32) -> (f32, f32) {
    let start_x = 0.0 - (WINDOW_HEIGHT * RESOLUTION - TILE_SIZE) / 2.0;
    let start_y = 0.0 + (WINDOW_HEIGHT + TILE_SIZE) / 2.0;

    let x_out = x_in - TILE_SIZE / 2.0 + (WINDOW_HEIGHT * RESOLUTION) / 2.0;
    let y_out = y_in - TILE_SIZE / 2.0 + WINDOW_HEIGHT / 2.0;

    (x_out, y_out)
}
