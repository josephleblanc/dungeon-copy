use bevy::prelude::*;

use self::{
    ac_modifier::ACModifierEvent,
    armor_class::{ACBonusEvent, ACBonusSumEvent},
    attack::{
        check_attack_conditions, start_attack, sum_attack_modifier, AttackBonusEvent,
        AttackBonusSumEvent, AttackRollEvent, StartAttack,
    },
    attack_modifier::AttackModifierEvent,
};
use crate::plugins::combat::armor_class::sum_ac_modifiers;
use crate::plugins::combat::attack::attack_roll;
use crate::plugins::combat::attack::debug_attack_roll_event;
use crate::scenes::SceneState;

pub mod ac_modifier;
pub mod armor_class;
pub mod attack;
pub mod attack_modifier;
pub mod bonus;
pub mod damage;
pub mod damage_modifier;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartAttack>()
            .add_event::<ACBonusEvent>()
            .add_event::<ACModifierEvent>()
            .add_event::<ACBonusSumEvent>()
            .add_event::<AttackBonusEvent>()
            .add_event::<AttackModifierEvent>()
            .add_event::<AttackBonusSumEvent>()
            .add_event::<AttackRollEvent>();

        app.add_systems(
            Update,
            (
                check_attack_conditions,
                start_attack.after(check_attack_conditions),
                // .after(check_attack_conditions),
                (
                    // This is where all of the systems that listen for AttackBonusEvent should go.
                    // This way they can run in parallel, and all of the events they emit as
                    // `AttackModifierEvent` can be added together in the attack_modifier_sum system
                    // that runs next.
                    attack_modifier::add_strength,
                    attack_modifier::add_weapon_focus,
                )
                    .run_if(on_event::<AttackBonusEvent>()),
                // This is where all of the systems that listen for `ACBonusEvent` should go. These
                // are all systems that modify the AC of the attacked creature or player. This way
                // they can all run in parallel, and each of the `ACModifierEvent`s they send can be
                // summed together in `sum_ac_modifiers` below.
                ac_modifier::add_dexterity.run_if(on_event::<ACBonusEvent>()),
                // sum_ac_modifiers should run after all of the systems in ac_modifier, e.g.
                // ac_modifier::add_dexterity
                sum_ac_modifiers.after(ac_modifier::add_dexterity),
                // sum_attack_modifier should run after all of the systems in attack_modifier,
                // e.g. attack_modifier::add_weapon_focus
                sum_attack_modifier
                    .after(attack_modifier::add_strength)
                    .after(attack_modifier::add_weapon_focus),
                // attack_roll should run after `sum_ac_modifiers` and `sum_attack_modifier`, as it
                // uses the events both of these systems emit to calculate the outcome of the attack
                // roll.
                attack_roll
                    .after(sum_attack_modifier)
                    .after(sum_ac_modifiers),
                debug_attack_roll_event.after(attack_roll),
            )
                .run_if(in_state(SceneState::InGameClassicMode)),
        );
    }
}
