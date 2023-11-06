use bevy::prelude::*;

use crate::plugins::player::control::ActionPriority;

use super::{
    ac_modifier::{ACModEvent, ACModList},
    AttackData,
};

#[derive(Copy, Clone, Event)]
pub struct ACBonusEvent;
// pub attacker: Entity,
// pub defender: Entity,
// pub attack_data: AttackData,
// }

#[derive(Copy, Clone, Event, Deref)]
pub struct ACBonusSumEvent {
    pub attack_data: AttackData,
    #[deref]
    pub total_ac_bonus: isize,
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
            let attack_data = ac_mod_list.verified_data().unwrap();
            let sum_event = ACBonusSumEvent {
                attack_data,
                total_ac_bonus: ac_mod_list.sum_all(),
            };

            // if attacker == attacker_entity {
            ac_mod_finished.send(sum_event);
            // } else {
            //     panic!("Attacking entity does not have ActionPriority");
            // }
        }
    }
}
