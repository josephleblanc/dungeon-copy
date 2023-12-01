use crate::scenes::SceneState;
use bevy::prelude::*;

pub mod cleanup;
pub mod feature;
pub mod movement;

use movement::click_move;
use movement::path_list_event;

use super::game_ui::action_bar::SelectedAction;
use super::{game_ui::map::pathing::PathSpriteEvent, interact::InteractingPosEvent};
use crate::plugins::input::movement::move_event;
use crate::plugins::input::movement::move_event::MovementPathEvent;
use crate::plugins::input::movement::path_list_event::PathListEvent;
use crate::plugins::input::movement::path_move;

pub struct InputHandlePlugin;

impl Plugin for InputHandlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<movement::Movement>();
        app.init_resource::<movement::click_move::PathNodes>();
        app.init_resource::<movement::click_move::PathConditions>();

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
                path_move::path_move_system.before(move_event::move_event_system),
                click_move::handle_path.before(move_event::move_event_system),
                click_move::check_path_conditions.before(click_move::start_path_list),
                (
                    click_move::start_path_list,
                    click_move::add_path.after(click_move::start_path_list),
                    click_move::path_list_cleanup,
                    // click_move::path_list_cleanup,
                )
                    .run_if(on_event::<path_list_event::PathListEvent>())
                    .after(click_move::handle_path),
                move_event::move_event_system,
            )
                .run_if(in_state(SceneState::InGameClassicMode)),
        );

        app.add_systems(
            Update,
            click_move::reset_by_action_button
                .run_if(resource_exists_and_changed::<SelectedAction>())
                .run_if(in_state(SceneState::InGameClassicMode)),
        );
    }
}
