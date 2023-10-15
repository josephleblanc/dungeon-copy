use bevy::prelude::*;

use crate::scenes::SceneState;

pub mod animation;
mod cleanup;
pub mod collisions;
pub mod initiate;

pub struct PlayerPlugin;

#[derive(Resource)]
pub struct PlayerEntity {
    pub entity: Entity,
}

pub const PLAYER_SIZE_WIDTH: f32 = 16.0 * 3.5;
pub const PLAYER_SIZE_HEIGHT: f32 = 28.0 * 3.5;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(SceneState::PreClassicMode),
            initiate::initiate_player,
        );

        app.add_systems(
            Update,
            (animation::player_animation_system,).run_if(in_state(SceneState::InGameClassicMode)),
        );

        app.add_systems(
            OnExit(SceneState::InGameClassicMode),
            (
                cleanup::cleanup_player,
                // ui::cleanup
            ),
        );
    }
}
