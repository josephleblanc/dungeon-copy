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
    pub attacker: Entity,
    pub defender: Entity,
    pub total_ac_bonus: isize,
}

/// Collects the various AC modifiers from the systems which manage those modifiers and send out
/// events with their individual bonuses.
pub fn sum_ac_modifiers(
    mut ac_mod_events: EventReader<ACModifierEvent>,
    attacker_query: Query<Entity, With<ActionPriority>>,
    mut ac_mod_finished: EventWriter<ACBonusSumEvent>,
) {
    let ac_mod_list: ACModifierList = ac_mod_events
        .into_iter()
        .map(|&event| event.into())
        .collect();
    if !ac_mod_list.is_empty() {
        if let Ok(attacker_entity) = attacker_query.get_single() {
            println!("debug | armor_class::sum_ac_modifiers | start");
            let attacker = ac_mod_list.verified_attacker().unwrap();
            let defender = ac_mod_list.verified_defender().unwrap();
            let sum_event = ACBonusSumEvent {
                attacker,
                defender,
                total_ac_bonus: ac_mod_list.sum_all(),
            };

            if attacker == attacker_entity {
                ac_mod_finished.send(sum_event);
            } else {
                panic!("Attacking entity does not have ActionPriority");
            }
        }
    }
}
