use crate::scenes::SceneState;
use bevy::prelude::*;

pub mod cleanup;
pub mod feature;
pub mod movement;

use movement::click_move;

use self::movement::click_move::MovementPathEvent;

pub struct InputHandlePlugin;

impl Plugin for InputHandlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<movement::Movement>();

        app.add_event::<MovementPathEvent>();

        app.add_systems(
            OnEnter(SceneState::InGameClassicMode),
            cleanup::cleanup_mouse,
        );

        app.add_systems(
            Update,
            (movement::player_movement_system,).run_if(in_state(SceneState::InGameClassicMode)),
        );

        app.add_systems(
            Update,
            (
                click_move::handle_click.before(click_move::move_event_system),
                click_move::move_path_system.before(click_move::move_event_system),
                click_move::move_event_system,
            )
                .run_if(in_state(SceneState::InGameClassicMode)),
        );
    }
}
