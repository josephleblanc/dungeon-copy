use bevy::prelude::*;

use crate::plugins::combat::attack::{
    attack_roll_modifier::{AttackModEvent, AttackModList},
    AttackData,
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
    mut atk_mod_finished: EventWriter<AttackBonusSumEvent>,
) {
    let atk_mod_list: AttackModList = atk_mod_events.into_iter().map(|event| **event).collect();
    if !atk_mod_list.is_empty() {
        let attack_data = atk_mod_list.verified_data().unwrap();
        let sum_event = AttackBonusSumEvent {
            attack_data,
            total_attack_bonus: atk_mod_list.sum_all(),
        };

        atk_mod_finished.send(sum_event);
    }
}
