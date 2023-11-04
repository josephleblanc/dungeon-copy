use bevy::prelude::*;

use crate::{plugins::player::control::ActionPriority, resources::equipment::weapon::Weapon};

use super::ac_modifier::{ACModEvent, ACModList};

#[derive(Clone, Event)]
pub struct ACBonusEvent {
    pub attacker: Entity,
    pub defender: Entity,
    pub attacker_weapon: Weapon,
}

#[derive(Clone, Event)]
pub struct ACBonusSumEvent {
    pub attacker: Entity,
    pub defender: Entity,
    pub total_ac_bonus: isize,
    pub attacker_weapon: Weapon,
}

/// Collects the various AC modifiers from the systems which manage those modifiers and send out
/// events with their individual bonuses.
pub fn sum_ac_modifiers(
    mut ac_mod_events: EventReader<ACModEvent>,
    attacker_query: Query<Entity, With<ActionPriority>>,
    mut ac_mod_finished: EventWriter<ACBonusSumEvent>,
) {
    let debug = true;
    let ac_mod_list: ACModList = ac_mod_events
        .into_iter()
        .map(|event| (**event).clone())
        .collect();
    if !ac_mod_list.is_empty() {
        if let Ok(attacker_entity) = attacker_query.get_single() {
            if debug {
                println!("debug | armor_class::sum_ac_modifiers | start");
            }
            let attacker = ac_mod_list.verified_attacker().unwrap();
            let defender = ac_mod_list.verified_defender().unwrap();
            let attacker_weapon = ac_mod_list.verified_weapon().unwrap();
            let sum_event = ACBonusSumEvent {
                attacker,
                defender,
                total_ac_bonus: ac_mod_list.sum_all(),
                attacker_weapon,
            };

            if attacker == attacker_entity {
                ac_mod_finished.send(sum_event);
            } else {
                panic!("Attacking entity does not have ActionPriority");
            }
        }
    }
}
