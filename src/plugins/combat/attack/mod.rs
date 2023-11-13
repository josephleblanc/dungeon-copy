use bevy::prelude::*;

use self::armor_class::{sum_armor_class_modifiers, ACBonusSumEvent};
use self::attack_roll::{sum_attack_modifier, AttackBonusSumEvent};
use self::crit_multiplier::{sum_crit_multiplier, CritMultiplier, CritMultiplierSumEvent};
use self::critical_range::{sum_crit_range_mods, CritRangeModSumEvent};
use crate::plugins::combat::CompleteAttack;
use crate::plugins::item::equipment::weapon::EquippedWeapons;
use crate::plugins::player::attacks::IterativeAttack;
use crate::plugins::player::equipment::{WeaponSlot, WeaponSlotName};
use crate::plugins::{
    interact::{InteractingPos, InteractingType},
    player::control::ActionPriority,
};
use crate::resources::dice::Dice;
use crate::scenes::SceneState;

use super::{AttackDataEvent, AttackModifier, CompleteAttackEvent, SumModifier};

pub mod armor_class;
pub mod armor_class_modifier;
pub mod attack_roll;
pub mod attack_roll_modifier;
pub mod crit_multiplier;
pub mod crit_multiplier_modifier;
pub mod critical_range;
pub mod critical_range_modifier;

// #[derive(Clone, Hash, Debug, PartialEq, Eq, SystemSet)]
// pub struct AttackRollComplete;

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartAttack>()
            .add_event::<AttackDataEvent>()
            // AC-related Events
            .add_event::<armor_class::ACBonusSumEvent>()
            .add_event::<armor_class_modifier::ACModEvent>()
            // Attack roll related events
            .add_event::<attack_roll::AttackBonusSumEvent>()
            .add_event::<attack_roll_modifier::AttackModEvent>()
            // Critical Threat related events
            .add_event::<critical_range_modifier::CritRangeModEvent>()
            .add_event::<critical_range::CritRangeModSumEvent>()
            // Crit Multiplier related events
            .add_event::<crit_multiplier_modifier::CritMultiplierModEvent>()
            .add_event::<crit_multiplier::CritMultiplierSumEvent>();

        // app.configure_set(Update, AttackModifier.after(check_attack_conditions));
        // app.configure_set(Update, SumModifier.after(AttackModifier));
        // app.configure_set(Update, AttackRollComplete.after(SumRollModifier));

        // app.add_systems(
        //     Update,
        //     check_attack_conditions.run_if(in_state(SceneState::InGameClassicMode)),
        // );

        app.add_systems(
            Update,
            (
                attack_roll_modifier::base_attack_bonus,
                attack_roll_modifier::add_strength,
                attack_roll_modifier::add_weapon_focus,
                critical_range_modifier::base,
                critical_range_modifier::improved_critical,
                crit_multiplier_modifier::base,
                armor_class_modifier::base,
                armor_class_modifier::add_dexterity,
            )
                .in_set(AttackModifier),
        );

        app.add_systems(
            Update,
            (
                sum_armor_class_modifiers,
                sum_attack_modifier,
                sum_crit_range_mods,
                sum_crit_multiplier,
            )
                .in_set(SumModifier),
        );
    }
}

#[derive(Event)]
/// This event is sent by `check_attack_conditions` and listened to by `start_attack`. This
/// singleton event is the signal that all of the conditions have been met for the attack roll and
/// damage systems to run without causing a `panic`.
pub struct StartAttack;

#[derive(Debug, Copy, Clone, PartialEq)]
/// AttackOutcome is the enum which describes the outcome of an attack roll, with modifiers
/// applied, against a valid target.
/// attack_roll --> AttackRollEvent --> Crit_threat_modifer::*
pub enum AttackOutcome {
    CritHit,
    Hit,
    Miss,
    CritMiss,
}
