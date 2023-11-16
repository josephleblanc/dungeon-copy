use bevy::prelude::*;

use crate::plugins::combat::AttackData;

use super::armor_class_modifier::{ACModEvent, ACModList};

#[derive(Copy, Clone, Event, Deref)]
pub struct ACBonusSumEvent {
    pub attack_data: AttackData,
    #[deref]
    pub total_ac_bonus: isize,
}

/// Collects the various AC modifiers from the systems which manage those modifiers and send out
/// events with their individual bonuses.
pub fn sum_armor_class_modifiers(
    mut ac_mod_events: EventReader<ACModEvent>,
    mut ac_mod_finished: EventWriter<ACBonusSumEvent>,
) {
    let debug = false;
    let ac_mod_list: ACModList = ac_mod_events.read().map(|event| (**event)).collect();
    if !ac_mod_list.is_empty() {
        if debug {
            println!("debug | armor_class::sum_armor_class_modifiers | start");
        }
        let attack_data = ac_mod_list.verified_data().unwrap();
        let sum_event = ACBonusSumEvent {
            attack_data,
            total_ac_bonus: ac_mod_list.sum_all(),
        };

        ac_mod_finished.send(sum_event);
    }
}
