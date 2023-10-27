use crate::scenes::SceneState;
use bevy::prelude::*;

pub mod cleanup;
pub mod feature;
pub mod movement;

use movement::click_move;

use self::movement::click_move::MovementPathEvent;

use super::{game_ui::map::pathing::PathSpriteEvent, interact::InteractingPosEvent};
use crate::plugins::input::click_move::PathListEvent;

pub struct InputHandlePlugin;

impl Plugin for InputHandlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<movement::Movement>();
        app.init_resource::<movement::click_move::Paths>();

        app.add_event::<MovementPathEvent>();
        app.add_event::<PathListEvent>();
        app.add_event::<PathSpriteEvent>();
        app.add_event::<InteractingPosEvent>();

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
                click_move::move_path_system.before(click_move::move_event_system),
                click_move::handle_path.before(click_move::move_event_system),
                (
                    click_move::start_path_list,
                    click_move::repath.after(click_move::start_path_list),
                    click_move::path_list_cleanup,
                    // click_move::path_list_cleanup,
                )
                    .run_if(on_event::<movement::click_move::PathListEvent>())
                    .after(click_move::handle_path),
                click_move::move_event_system,
            )
                .run_if(in_state(SceneState::InGameClassicMode)),
        );
    }
}
