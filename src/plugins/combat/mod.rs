use bevy::prelude::*;

use self::{
    ac_modifier::ACModifierEvent,
    armor_class::{ACBonusEvent, ACBonusSumEvent},
    attack::{
        check_attack_conditions, start_attack, sum_attack_modifiers, AttackBonusEvent,
        AttackBonusSumEvent, AttackRollEvent, StartAttack,
    },
    attack_modifiers::AttackModifierEvent,
};
use crate::plugins::combat::armor_class::sum_ac_modifiers;
use crate::plugins::combat::attack::attack_roll;
use crate::plugins::combat::attack::debug_attack_roll_event;
use crate::scenes::SceneState;

pub mod ac_modifier;
pub mod armor_class;
pub mod attack;
pub mod attack_modifiers;
pub mod bonus;

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
                    // This is where all if the systems that listen for AttackBonusEvent
                    // should go. This way they can run in parallel, and all of the events
                    // they emit as `AttackModifierEvent` can be added together in the
                    // attack_modifier_sum system that runs next.
                    attack_modifiers::add_strength,
                    attack_modifiers::add_weapon_focus,
                )
                    // .after(start_attack)
                    .run_if(on_event::<AttackBonusEvent>()),
                ac_modifier::add_dexterity.run_if(on_event::<ACBonusEvent>()),
                sum_ac_modifiers.after(ac_modifier::add_dexterity),
                sum_attack_modifiers
                    .after(attack_modifiers::add_strength)
                    .after(attack_modifiers::add_weapon_focus),
                attack_roll
                    .after(sum_attack_modifiers)
                    .after(sum_ac_modifiers),
                debug_attack_roll_event.after(attack_roll),
            )
                .run_if(in_state(SceneState::InGameClassicMode)),
        );
    }
}
