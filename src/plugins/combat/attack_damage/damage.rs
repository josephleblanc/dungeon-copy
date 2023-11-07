#![allow(dead_code)]
use bevy::prelude::*;

use crate::plugins::combat::CompleteAttackEvent;

use super::damage_modifier::AttackDamageModEvent;

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app;
    }
}

#[derive(Copy, Clone)]
pub struct AttackDamageSum {
    val: isize,
}

#[derive(Event, Copy, Clone)]
pub struct AttackDamageSumEvent(AttackDamageSum);

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DamageBonusSource {
    Strength,
    Base,
}

impl DamageBonusSource {
    pub fn stackable() -> [Self; 0] {
        []
    }
    pub fn non_stackable() -> [Self; 1] {
        [Self::Strength]
    }
}

pub fn sum_damage_mod(
    mut damage_mod_writer: EventReader<AttackDamageModEvent>,
    mut damage_sum_event: EventWriter<AttackDamageSumEvent>,
) {
}
