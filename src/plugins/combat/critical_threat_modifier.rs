use bevy::prelude::*;

use crate::{components::armor_class::ArmorClass, plugins::player::control::ActionPriority};

use super::{
    attack::{AttackOutcome, AttackRollEvent},
    bonus::{BonusSource, BonusType},
};

#[derive(Copy, Clone, Debug)]
pub struct CritThreatMod {
    pub val: isize,
    pub source: BonusSource,
    pub bonus_type: BonusType,
    pub attacker: Entity,
    pub defender: Entity,
}

#[derive(Event, Copy, Clone, Deref, DerefMut)]
pub struct CritThreatModEvent(CritThreatMod);

pub fn start(
    mut attack_roll_reader: EventReader<AttackRollEvent>,
    attacker_query: Query<Entity, With<ActionPriority>>,
    defender_query: Query<Entity, With<ArmorClass>>,
    mut crit_mod_writer: EventWriter<CritThreatModEvent>,
) {
    for &roll_hit_event in attack_roll_reader
        .iter()
        .filter(|roll| roll.attack_outcome == AttackOutcome::Hit)
    {}
}
