use bevy::prelude::*;

use crate::plugins::combat::bonus::{BonusSource, BonusType};

use super::StartInitiative;

#[derive(Copy, Clone, Debug)]
pub struct InitiativeMod {
    pub val: isize,
    pub source: BonusSource,
    pub bonus_type: BonusType,
    pub entity: Entity,
}

impl From<InitiativeMod> for usize {
    fn from(value: InitiativeMod) -> Self {
        value.val as usize
    }
}

impl From<InitiativeMod> for isize {
    fn from(value: InitiativeMod) -> Self {
        value.val
    }
}

#[derive(Event, Clone, Deref, DerefMut)]
pub struct InitiativeModEvent(InitiativeMod);

impl From<InitiativeMod> for InitiativeModEvent {
    fn from(value: InitiativeMod) -> Self {
        InitiativeModEvent(value)
    }
}

impl From<InitiativeModEvent> for InitiativeMod {
    fn from(value: InitiativeModEvent) -> Self {
        value.0
    }
}

pub fn base_initiative(query_dexterity: EventReader<StartInitiative>) {}
