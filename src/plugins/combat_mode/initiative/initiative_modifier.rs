use bevy::prelude::*;

use crate::components::attributes::Attribute;
use crate::{
    components::{attributes::Dexterity, creature::Creature},
    plugins::combat::bonus::{BonusSource, BonusType},
};

use super::StartInitiative;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InitiativeBonusSource {
    Dexterity,
}

#[derive(Copy, Clone, Debug)]
pub struct InitiativeMod {
    pub val: isize,
    pub source: InitiativeBonusSource,
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

#[derive(Debug, Event, Clone, Copy, Deref, DerefMut)]
pub struct InitiativeModEvent(InitiativeMod);

impl InitiativeModEvent {
    pub fn from(initiative_mod: InitiativeMod) -> Self {
        Self(initiative_mod)
    }
}

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

pub fn base_initiative(
    mut event_reader: EventReader<StartInitiative>,
    mut event_writer: EventWriter<InitiativeModEvent>,
    query_dexterity: Query<&Dexterity, With<Creature>>,
) {
    for creature in event_reader.into_iter() {
        if let Ok(dexterity) = query_dexterity.get(**creature) {
            let initiative_event = InitiativeModEvent::from(InitiativeMod {
                val: dexterity.bonus(),
                source: InitiativeBonusSource::Dexterity,
                bonus_type: BonusType::Untyped,
                entity: **creature,
            });
            event_writer.send(initiative_event);
            println!(
                "debug | initiative_modifier::base_initiative | sending initiative event: {:?}",
                initiative_event
            );
        } else {
            panic!(
                "Cannot have a creature without dexterity roll initiative. \
            Every Creature must have Dexterity."
            )
        }
    }
}
