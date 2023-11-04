use bevy::prelude::*;

use crate::{
    components::{armor_class::ArmorClass, feats::combat_feats::ImprovedCritical},
    plugins::player::control::ActionPriority,
};

use super::{
    attack::{AttackOutcome, AttackRollEvent},
    bonus::{BonusSource, BonusType},
};

#[derive(Copy, Clone, Debug)]
pub struct CritThreatMod {
    pub val: usize,
    pub source: CritThreatBonusSource,
    pub bonus_type: CritThreatBonusType,
    pub attacker: Entity,
    pub defender: Entity,
}
#[derive(Copy, Clone, Debug)]
pub enum CritThreatBonusSource {
    ImprovedCritical, // more here
}

#[derive(Copy, Clone, Debug)]
pub enum CritThreatBonusType {
    DoubleRange,
}

#[derive(Event, Copy, Clone, Deref, DerefMut)]
pub struct CritThreatModEvent(CritThreatMod);

pub fn improved_critical(
    mut attack_roll_reader: EventReader<AttackRollEvent>,
    attacker_query: Query<&ImprovedCritical, With<ActionPriority>>,
    defender_query: Query<Entity, With<ArmorClass>>,
    mut crit_mod_writer: EventWriter<CritThreatModEvent>,
) {
    for roll_hit_event in attack_roll_reader
        .iter()
        .filter(|roll| roll.attack_outcome == AttackOutcome::Hit)
    {
        if let Ok(improved_critical) = attacker_query.get(roll_hit_event.attacker) {
            if let Some(modifier) = improved_critical.to_crit_threat_mod(
                roll_hit_event.attacker,
                roll_hit_event.defender,
                &roll_hit_event.attacker_weapon,
            ) {
                crit_mod_writer.send(CritThreatModEvent(modifier));
            }
        }
    }
}
