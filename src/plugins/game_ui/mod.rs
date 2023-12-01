use bevy::prelude::*;

use crate::scenes::SceneState;

use self::{
    action_bar::{submenu_button::SelectedSubMenu, SelectedAction},
    combat_mode::CombatModeRes,
    map::pathing::PathSpriteEvent,
};

use super::{actions::TurnActionEvent, combat_mode::turn::action::CurrentTurn};

pub mod action_bar;
pub mod combat_mode;
pub mod map;
pub mod translate;
pub mod turn_actions;
pub mod turn_mode;
pub mod ui_root;

pub struct IngameUiPlugin;

impl Plugin for IngameUiPlugin {
    fn build(&self, app: &mut App) {
        app
            // TODO: Move TurnActionEvent into the action plugin
            .add_event::<TurnActionEvent>()
            .init_resource::<SelectedSubMenu>()
            .add_systems(
                OnEnter(SceneState::InGameClassicMode),
                (
                    map::setup,
                    ui_root::setup,
                    apply_deferred,
                    combat_mode::setup,
                    turn_mode::setup,
                    turn_actions::setup,
                    action_bar::setup,
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
                turn_actions::TurnActionButton::update_color.run_if(on_event::<TurnActionEvent>()),
                turn_actions::TurnActionButton::reset_color
                    .run_if(resource_exists_and_changed::<CurrentTurn>()),
                action_bar::handle_buttons.run_if(resource_exists::<SelectedAction>()),
                action_bar::handle_button_borders
                    .run_if(resource_exists_and_changed::<SelectedAction>()),
                action_bar::handle_submenu_select,
                map::pathing::despawn_move_path.run_if(on_event::<PathSpriteEvent>()),
                action_bar::submenu_button::handle_submenu_display,
                action_bar::submenu_button::handle_submenu_buttons,
                action_bar::submenu_button::handle_submenu_border
                    .run_if(resource_exists_and_changed::<SelectedSubMenu>()),
            )
                .run_if(in_state(SceneState::InGameClassicMode)),
        );

        app.add_systems(
            Update,
            (combat_mode::debug_buttons).run_if(resource_exists_and_changed::<CombatModeRes>()),
        );

        app.add_systems(
            OnExit(SceneState::InGameClassicMode),
            (
                map::cleanup,
                action_bar::cleanup,
                combat_mode::cleanup,
                turn_mode::cleanup,
            ),
        );
    }
}
