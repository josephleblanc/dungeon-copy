#![allow(dead_code)]

use bevy::prelude::*;
use std::{ops::DerefMut, slice::Iter};

use self::event::{update_turn_actions, MoveActionEvent, TurnActionEvent};
use crate::plugins::actions::event::update_move_actions;

use super::{
    combat_mode::state::CombatMode,
    game_ui::{
        action_bar::submenu_button::{MoveButton, SelectedSubMenu},
        combat_mode::CombatModeRes,
        turn_actions::TurnActionButton,
    },
};

pub mod event;

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TurnActionEvent>()
            .add_event::<MoveActionEvent>()
            .add_systems(
                Update,
                (
                    setup.run_if(resource_exists_and_equals(CombatModeRes(
                        CombatMode::InCombat,
                    ))),
                    update_move_actions
                        .run_if(resource_exists::<SelectedSubMenu>())
                        .run_if(on_event::<MoveActionEvent>()),
                    update_turn_actions
                        .run_if(resource_exists::<ActionStatus>())
                        .run_if(on_event::<TurnActionEvent>()),
                ),
            )
            .add_systems(
                Update,
                cleanup.run_if(resource_exists_and_equals(CombatModeRes(
                    CombatMode::OutOfCombat,
                ))),
            );
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TurnAction {
    Move,
    Standard,
    FiveFootStep,
    Immediate,
    FullRound,
}

impl TurnAction {
    pub fn iterator() -> Iter<'static, Self> {
        [
            TurnAction::Move,
            TurnAction::Standard,
            TurnAction::FiveFootStep,
            TurnAction::Immediate,
            TurnAction::FullRound,
        ]
        .iter()
    }
}

impl From<TurnActionButton> for TurnAction {
    fn from(value: TurnActionButton) -> Self {
        match value {
            TurnActionButton::Immediate => TurnAction::Immediate,
            TurnActionButton::Move => TurnAction::Move,
            TurnActionButton::Standard => TurnAction::Standard,
            TurnActionButton::FiveFootStep => TurnAction::FiveFootStep,
            TurnActionButton::FullRound => TurnAction::FullRound,
        }
    }
}

impl From<MoveButton> for TurnAction {
    fn from(value: MoveButton) -> Self {
        match value {
            MoveButton::MoveAction => TurnAction::Move,
            MoveButton::StandardAction => TurnAction::Standard,
            MoveButton::FiveFootStep => TurnAction::FiveFootStep,
            MoveButton::FullMove => TurnAction::FullRound,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum TurnActionStatus {
    Used,
    #[default]
    Available,
    Planned,
}

impl TurnActionStatus {
    pub fn is_available(self) -> bool {
        self == TurnActionStatus::Available
    }
}

#[derive(Clone, Copy, Resource, Debug, PartialEq, Eq, Default)]
pub struct ActionStatus {
    pub move_action: TurnActionStatus,
    pub standard: TurnActionStatus,
    pub immediate: TurnActionStatus,
    pub five_foot_step: TurnActionStatus,
    pub full_round: TurnActionStatus,
}

impl ActionStatus {
    pub fn reset(&mut self) {
        self.move_action = TurnActionStatus::Available;
        self.standard = TurnActionStatus::Available;
        self.immediate = TurnActionStatus::Available;
        self.five_foot_step = TurnActionStatus::Available;
    }
}

pub fn setup(mut commands: Commands) {
    commands.init_resource::<ActionStatus>();
}

pub fn cleanup(mut commands: Commands) {
    commands.remove_resource::<ActionStatus>();
}
