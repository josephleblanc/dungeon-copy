use bevy::prelude::*;

use crate::scenes::SceneState;

use self::{
    attack::{
        check_attack_conditions, start_attack, sum_attack_modifiers, AttackBonusEvent,
        AttackBonusSumEvent, StartAttack,
    },
    attack_modifiers::AttackModifierEvent,
};
use crate::plugins::combat::armor_class::sum_ac_modifiers;

pub mod ac_modifier;
pub mod armor_class;
pub mod attack;
pub mod attack_modifiers;
pub mod bonus;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartAttack>()
            .add_event::<AttackBonusEvent>()
            .add_event::<AttackModifierEvent>()
            .add_event::<AttackBonusSumEvent>();

        app.add_systems(
            Update,
            (
                check_attack_conditions,
                start_attack,
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
                (sum_attack_modifiers, sum_ac_modifiers),
            )
                .chain()
                .run_if(in_state(SceneState::InGameClassicMode)),
        );
    }
}
