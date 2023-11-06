use bevy::prelude::*;

use crate::components::creature::Creature;
use crate::resources::dice::Dice;
use crate::{components::attack_bonus::BaseAttackBonus, plugins::player::control::ActionPriority};

use crate::plugins::combat::attack::{
    armor_class::ACBonusSumEvent,
    attack_roll_modifier::{AttackModEvent, AttackModList},
    {AttackData, AttackDataEvent},
};

#[derive(Copy, Clone, Event)]
/// AttackBonusEvent is sent by `start_attack` and listened for by all of the systems in the
/// `attack_modifier` mod. This event is the signal for all of the systems which check to see if
/// they can apply an attack modifier to an attack should be run.
/// Since this event is sent by one system and listened to by many, it is important for the system
/// scheduling to ensure that all of the systems which listen for it run after this event is sent.
/// If not, logic errors could pop up - for example, if two attacks different attacks occured in
/// consecutive frames, then the bonuses from one attack might try to be added to another.
pub struct AttackBonusEvent;
// pub struct AttackBonusEvent {
//     pub attacker: Entity,
//     pub defender: Entity,
//     pub attacker_weapon: Entity,
// }

#[derive(Copy, Clone, Event, Deref)]
pub struct AttackBonusSumEvent {
    pub attack_data: AttackData,
    #[deref]
    pub total_attack_bonus: isize,
}

/// `sum_attack_modifier` adds together all of the modifiers in the `attack_modifier` mod. It
/// listens for the event `AttackModEvent`, which should have been sent out by each of the
/// systems deciding whether a modifier should be applied to the attack.
/// Because this is a system which listens for an event which is sent out by many systems, it is
/// important to use explicit system scheduling to ensure all of the systems in `attack_modifier`
/// have run before this system. Otherwise some attack modifiers may not be applied to the attack
/// which prompted the modifier system to run, and could prompt a `panic` or logical error when
/// they are attempted to be summed with the modifiers from another attack.
pub fn sum_attack_modifier(
    mut atk_mod_events: EventReader<AttackModEvent>,
    attacker_query: Query<Entity, With<ActionPriority>>,
    mut atk_mod_finished: EventWriter<AttackBonusSumEvent>,
) {
    let atk_mod_list: AttackModList = atk_mod_events
        .into_iter()
        .map(|event| (**event).clone())
        .collect();
    if !atk_mod_list.is_empty() {
        println!("debug | attack::sum_attack_modifier | start");
        if let Ok(attacker_entity) = attacker_query.get_single() {
            let attack_data = atk_mod_list.verified_data().unwrap();
            let sum_event = AttackBonusSumEvent {
                attack_data,
                total_attack_bonus: atk_mod_list.sum_all(),
            };

            // if attacker == attacker_entity {
            atk_mod_finished.send(sum_event);
            // } else {
            //     panic!("Attacking entity does not have ActionPriority");
            // }
        }
    }
}

// #[derive(Debug, Event, Copy, Clone)]
// pub struct AttackRollEvent {
//     pub attack_data: AttackData,
//     pub attack_roll_raw: usize,
//     pub attack_roll_total: isize,
//     pub total_attack_modifier: isize,
//     pub total_defender_ac: isize,
//     pub attack_outcome: AttackOutcome,
// }

// TODO: Decide how to handle the conclusion of the attack as a whole.
// pub fn attack_roll(
//     mut attack_data_event: EventReader<AttackDataEvent>,
//     mut ac_mod_finished: EventReader<ACBonusSumEvent>,
//     mut atk_mod_finished: EventReader<AttackBonusSumEvent>,
//     mut attack_roll_event_writer: EventWriter<AttackRollEvent>,
//     attacker_query: Query<(Entity, &BaseAttackBonus), With<ActionPriority>>,
//     defender_query: Query<Entity, With<Creature>>,
// ) {
//     for ((ac_event, atk_event), attack_data) in ac_mod_finished
//         .into_iter()
//         .zip(atk_mod_finished.into_iter())
//         .zip(attack_data_event.into_iter())
//         .filter(|((&ac_event, &atk_event), &attack_data)| {
//             ac_event.attack_data == atk_event.attack_data && *attack_data == ac_event.attack_data
//         })
//     {
//         let debug = true;
//         println!("debug | attack::attack_roll | start");
//         let defender = defender_query.get(attack_data.defender).unwrap();
//         let (attacker, attacker_bab) = attacker_query.get(attack_data.attacker).unwrap();
//
//         let mut rng = rand::thread_rng();
//         let attack_roll_raw = Dice::D20.roll_once(&mut rng);
//         let total_attack_modifier = atk_event.total_attack_bonus + **attacker_bab;
//         let total_defender_ac = 10 + ac_event.total_ac_bonus;
//
//         let attack_roll_total: isize = attack_roll_raw as isize + total_attack_modifier;
//         let attack_outcome = if attack_roll_raw == 20 {
//             AttackOutcome::CritHit
//         } else if attack_roll_raw == 1 {
//             AttackOutcome::CritMiss
//         } else if total_defender_ac <= attack_roll_total {
//             AttackOutcome::Hit
//         } else {
//             AttackOutcome::Miss
//         };
//
//         // let attacker_weapon: Option<Entity> =
//         //     if ac_event.attacker_weapon == atk_event.attacker_weapon {
//         //         Some(ac_event.attacker_weapon)
//         //     } else {
//         //         None
//         //     };
//
//         if debug {
//             debug_attack_roll(
//                 attack_roll_raw,
//                 attacker_bab,
//                 atk_event,
//                 ac_event,
//                 attack_roll_total,
//                 attack_outcome,
//                 **attack_data,
//             );
//         }
//
//         attack_roll_event_writer.send(AttackRollEvent {
//             attack_data: **attack_data,
//             attack_outcome,
//             attack_roll_raw,
//             attack_roll_total,
//             total_attack_modifier,
//             total_defender_ac,
//         });
//     }
// }
