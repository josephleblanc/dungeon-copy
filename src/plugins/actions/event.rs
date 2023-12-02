use bevy::prelude::*;

use crate::plugins::game_ui::action_bar::submenu_button::SelectedSubMenu;

use super::{ActionStatus, TurnAction, TurnActionStatus};

#[derive(Event, Copy, Clone, Debug)]
pub struct MoveActionEvent;

#[derive(Event, Copy, Clone, Debug, PartialEq, Eq)]
pub struct TurnActionEvent {
    pub turn_action: TurnAction,
    pub status: TurnActionStatus,
}

/// Read events from MoveActionEvent and send TurnActionEvent to update the status of available
/// turn actions.
pub fn update_move_actions(
    selected_submenu: Res<SelectedSubMenu>,
    mut event_reader: EventReader<MoveActionEvent>,
    mut event_writer: EventWriter<TurnActionEvent>,
) {
    for _event in event_reader.iter() {
        event_writer.send(TurnActionEvent {
            turn_action: TurnAction::from(selected_submenu.move_submenu),
            status: TurnActionStatus::Used,
        })
    }
}

pub fn update_turn_actions(
    mut event_reader: EventReader<TurnActionEvent>,
    mut action_status: ResMut<ActionStatus>,
) {
    for event in event_reader.into_iter() {
        match event.turn_action {
            TurnAction::Move => action_status.move_action = event.status,
            TurnAction::Standard => action_status.standard = event.status,
            TurnAction::Immediate => action_status.immediate = event.status,
            TurnAction::FiveFootStep => action_status.five_foot_step = event.status,
            TurnAction::FullRound => action_status.full_round = event.status,
        }
    }
}
