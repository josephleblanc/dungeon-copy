use bevy::prelude::*;

use self::{
    damage::{sum_damage_mod, AttackDamageSumEvent},
    damage_modifier::{add_strength, base, weapon, AttackDamageModEvent},
    damage_reduction::{debug_sum_damage_reduction, sum_damage_reduction, DRTotalEvent},
    damage_reduction_modifier::DRModEvent,
};

use super::{AttackModifier, DebugSet, SumModifier};

pub mod crit_damage;
pub mod crit_damage_modifier;
pub mod damage;
pub mod damage_modifier;
pub mod damage_reduction;
pub mod damage_reduction_modifier;
pub mod immunity;
pub mod immunity_modifier;

pub struct AttackDamagePlugin;

impl Plugin for AttackDamagePlugin {
    fn build(&self, app: &mut App) {
        app
            // Attack Damage related events
            .add_event::<AttackDamageModEvent>()
            .add_event::<AttackDamageSumEvent>()
            // Damage Reduction related events
            .add_event::<DRModEvent>()
            .add_event::<DRTotalEvent>()
            // Attack Damage related systems
            .add_systems(Update, (base, add_strength, weapon).in_set(AttackModifier))
            // Damage Reduction related systems
            .add_systems(
                Update,
                (damage_reduction_modifier::barbarian).in_set(AttackModifier),
            )
            .add_systems(Update, sum_damage_mod.in_set(SumModifier))
            .add_systems(Update, sum_damage_reduction.in_set(SumModifier))
            .add_systems(Update, debug_sum_damage_reduction.in_set(DebugSet));
    }
}
