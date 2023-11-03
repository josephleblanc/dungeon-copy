use bevy::prelude::*;

use crate::plugins::player::control::ActionPriority;

use super::ac_modifier::{ACModifierEvent, ACModifierList};

#[derive(Clone, Event)]
pub struct ACBonusEvent {
    pub attacker: Entity,
    pub defender: Entity,
}

#[derive(Clone, Event)]
pub struct ACBonusSumEvent {
    attacker: Entity,
    total_attack_bonus: isize,
    defender: Entity,
}

/// Collects the various AC modifiers from the systems which manage those modifiers and send out
/// events with their individual bonuses.
pub fn sum_ac_modifiers(
    mut atk_mod_events: EventReader<ACModifierEvent>,
    attacker_query: Query<Entity, With<ActionPriority>>,
    mut atk_mod_finished: EventWriter<ACBonusSumEvent>,
) {
    if let Ok(attacker_entity) = attacker_query.get_single() {
        let atk_mod_list: ACModifierList = atk_mod_events
            .into_iter()
            .map(|&event| event.into())
            .collect();

        let attacker = atk_mod_list.verified_attacker().unwrap();
        let defender = atk_mod_list.verified_defender().unwrap();
        let sum_event = ACBonusSumEvent {
            attacker,
            defender,
            total_attack_bonus: atk_mod_list.sum_all(),
        };

        if attacker == attacker_entity {
            atk_mod_finished.send(sum_event);
        } else {
            panic!("Attacking entity does not have ActionPriority");
        }
    }
}
