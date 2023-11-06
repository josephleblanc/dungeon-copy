use bevy::prelude::*;

use super::{
    critical_range_modifier::{CritThreatModEvent, CritThreatModList},
    AttackData,
};

#[derive(Copy, Clone, Event, Deref)]
pub struct CritThreatModSumEvent {
    pub attack_data: AttackData,
    #[deref]
    pub total_crit_range: [usize; 2],
}

impl CritThreatModSumEvent {
    pub fn lower_crit(&self) -> usize {
        self.total_crit_range[0]
    }
}

pub fn sum_crit_range_mods(
    mut crit_range_mods_reader: EventReader<CritThreatModEvent>,
    mut crit_range_mod_finished: EventWriter<CritThreatModSumEvent>,
) {
    let debug = false;
    let crit_range_mod_list: CritThreatModList = crit_range_mods_reader
        .into_iter()
        .map(|event| (**event))
        .collect();
    if !crit_range_mod_list.is_empty() {
        if debug {
            println!("debug | attack::sum_crit_range_mods | start");
        }
        let attack_data = crit_range_mod_list.verified_data().unwrap();

        let total_crit_range = [21 - crit_range_mod_list.sum_non_stackable(), 20];
        if debug {
            println!(
                "debug | attack::sum_crit_range_mods | total_crit_range: {:?}",
                total_crit_range
            );
        }
        let sum_event = CritThreatModSumEvent {
            attack_data,
            total_crit_range,
        };

        // if attacker == attacker_entity {
        crit_range_mod_finished.send(sum_event);
        // } else {
        //     panic!("Attacking entity does not have ActionPriority");
        // }
    }
}
