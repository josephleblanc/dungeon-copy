#![allow(dead_code)]

use bevy::prelude::*;
use std::{ops::DerefMut, slice::Iter};

use super::{
    combat_mode::state::CombatMode,
    game_ui::{combat_mode::CombatModeRes, turn_actions::TurnActionButton},
};

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TurnActionEvent>()
            .add_systems(
                Update,
                setup.run_if(resource_exists_and_equals(CombatModeRes(
                    CombatMode::InCombat,
                ))),
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
}

impl TurnAction {
    pub fn iterator() -> Iter<'static, Self> {
        [
            TurnAction::Move,
            TurnAction::Standard,
            TurnAction::FiveFootStep,
            TurnAction::Immediate,
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
        }
    }
}

#[derive(Event, Copy, Clone, Debug, PartialEq, Eq)]
pub struct TurnActionEvent {
    pub turn_action: TurnAction,
    pub status: TurnActionStatus,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum TurnActionStatus {
    Used,
    #[default]
    Available,
    Planned,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deref, DerefMut, Resource, Default)]
pub struct MoveAction(TurnActionStatus);
impl Action for MoveAction {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deref, DerefMut, Resource, Default)]
pub struct StandardAction(TurnActionStatus);
impl Action for StandardAction {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deref, DerefMut, Resource, Default)]
pub struct ImmediateAction(TurnActionStatus);
impl Action for ImmediateAction {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deref, DerefMut, Resource, Default)]
pub struct FiveFootStep(TurnActionStatus);
impl Action for FiveFootStep {}

pub trait Action {
    fn reset(&mut self)
    where
        Self: std::marker::Sized + DerefMut<Target = TurnActionStatus>,
    {
        **self = TurnActionStatus::Available;
    }
}

pub fn setup(mut commands: Commands) {
    commands.init_resource::<MoveAction>();
    commands.init_resource::<StandardAction>();
    commands.init_resource::<ImmediateAction>();
    commands.init_resource::<FiveFootStep>();
}

pub fn cleanup(mut commands: Commands) {
    commands.remove_resource::<MoveAction>();
    commands.remove_resource::<StandardAction>();
    commands.remove_resource::<ImmediateAction>();
    commands.remove_resource::<FiveFootStep>();
}
