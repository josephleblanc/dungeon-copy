use bevy::prelude::*;

use crate::plugins::item::equipment::weapon::EquippedWeapons;
use crate::resources::dice::Dice;
use crate::resources::equipment::weapon::Weapon;
use crate::{
    components::{armor_class::ArmorClass, attack_bonus::BaseAttackBonus, player::PlayerComponent},
    plugins::{
        interact::{InteractingPos, InteractingType},
        player::control::ActionPriority,
    },
};

use super::{
    armor_class::{ACBonusEvent, ACBonusSumEvent},
    attack_modifier::{AttackMod, AttackModEvent, AttackModList},
    bonus::BonusType,
};

#[derive(Clone, Event)]

/// AttackBonusEvent is sent by `start_attack` and listened for by all of the systems in the
/// `attack_modifier` mod. This event is the signal for all of the systems which check to see if
/// they can apply an attack modifier to an attack should be run.
/// Since this event is sent by one system and listened to by many, it is important for the system
/// scheduling to ensure that all of the systems which listen for it run after this event is sent.
/// If not, logic errors could pop up - for example, if two attacks different attacks occured in
/// consecutive frames, then the bonuses from one attack might try to be added to another.
pub struct AttackBonusEvent {
    pub attacker: Entity,
    pub defender: Entity,
    pub attacker_weapon: Weapon,
}

#[derive(Event)]
/// This event is sent by `check_attack_conditions` and listened to by `start_attack`. This
/// singleton event is the signal that all of the conditions have been met for the attack roll and
/// damage systems to run without causing a `panic`.
pub struct StartAttack;

#[derive(Clone, Event)]
pub struct AttackBonusSumEvent {
    attacker: Entity,
    total_attack_bonus: isize,
    defender: Entity,
    pub attacker_weapon: Weapon,
}

/// This is where the attack roll process begins. Once all of the conditions have been met this
/// function will send an event that lets `start_attack` begin.
/// Any conditions that must be met for an attack to begin should be put here, along with anything
/// that could cause a `panic` in the systems which compose the process of attacking and applying
/// damage.
pub fn check_attack_conditions(
    interacting_pos: Res<InteractingPos>,
    mut attack_event_writer: EventWriter<StartAttack>,
    button: Res<Input<MouseButton>>,
) {
    let debug = true;
    if debug && button.just_pressed(MouseButton::Left) {
        println!(
            "debug | check_attack_conditions | \
        interacting_pos.entity.is_some(): {}",
            interacting_pos.entity.is_some()
        );
    }
    if interacting_pos.interacting_type == InteractingType::Enemy
        && interacting_pos.entity.is_some()
        && button.just_pressed(MouseButton::Left)
    {
        // TODO: Check if target is in range
        attack_event_writer.send(StartAttack);
    }
}

/// start_attack runs when the conditions for an attack have been met in `check_attack_conditions`.
/// Then it sends two events, one to start the systems calculating the total of the attack modifiers,
/// another to the systems calculating the ac of the defender.
/// There are multiple systems which check whether each modifier should be applied to ac and attack,
/// and then send an event listened to by `sum_attack_modifier`.
pub fn start_attack(
    mut start_attack_events: EventReader<StartAttack>,
    mut attack_event_writer: EventWriter<AttackBonusEvent>,
    mut ac_event_writer: EventWriter<ACBonusEvent>,
    query_attacker: Query<(Entity, &EquippedWeapons), With<ActionPriority>>,
    interacting_pos: Res<InteractingPos>,
) {
    let debug = true;
    if !start_attack_events.is_empty() {
        start_attack_events.clear();

        if debug {
            println!("debug | attack::start_attack | start");
        }

        let (attacker, equipped_weapons) = query_attacker.get_single().unwrap();
        // TODO: Once I set up a different system to start the attacks (like prompting the attack
        // with a button), change `attacker_weapon` to instead use a value sent by the event that
        // prompts `start_attack`.
        let attacker_weapon = equipped_weapons.main_hand.clone();
        let defender = interacting_pos.entity.unwrap();
        attack_event_writer.send(AttackBonusEvent {
            attacker,
            defender,
            attacker_weapon: attacker_weapon.clone(),
        });
        ac_event_writer.send(ACBonusEvent {
            attacker,
            defender,
            attacker_weapon,
        });
    }
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
            let attacker = atk_mod_list.verified_attacker().unwrap();
            let defender = atk_mod_list.verified_defender().unwrap();
            let attacker_weapon = atk_mod_list.verified_weapon().unwrap();
            let sum_event = AttackBonusSumEvent {
                attacker,
                defender,
                total_attack_bonus: atk_mod_list.sum_all(),
                attacker_weapon,
            };

            if attacker == attacker_entity {
                atk_mod_finished.send(sum_event);
            } else {
                panic!("Attacking entity does not have ActionPriority");
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// AttackOutcome is the enum which describes the outcome of an attack roll, with modifiers
/// applied, against a valid target.
/// attack_roll --> AttackRollEvent --> Crit_threat_modifer::*
pub enum AttackOutcome {
    CritHit,
    Hit,
    Miss,
    CritMiss,
}

#[derive(Debug, Event, Clone)]
/// AttackRollEvent is the event sent out by `attack_roll`, and includes the outcome of an attack
/// against a valid target. This event is listened to by `start_damage`, which is the gatekeeper
/// for the systems which calculate the attack's damage.
pub struct AttackRollEvent {
    pub attacker: Entity,
    pub defender: Entity,
    pub attack_roll_raw: usize,
    pub attack_roll_total: isize,
    pub total_attack_modifier: isize,
    pub total_defender_ac: isize,
    pub attack_outcome: AttackOutcome,
    pub attacker_weapon: Weapon,
}

/// `attack_roll` is the system which sums all of the attack modifiers and armor class modifiers,
/// rolls the attack die, and then determines the outcome of the attack. An attack can `Hit`, `Miss`,
/// but can also `CritHit` or `CritMiss`.
/// If the attack is a `Hit` or a `CritHit`, the `damage::start_damage` system will start
/// calculating the damage of the attack.
// TODO: Decide how to handle the conclusion of the attack as a whole.
pub fn attack_roll(
    mut ac_mod_finished: EventReader<ACBonusSumEvent>,
    mut atk_mod_finished: EventReader<AttackBonusSumEvent>,
    mut attack_roll_event_writer: EventWriter<AttackRollEvent>,
    attacker_query: Query<(Entity, &BaseAttackBonus), With<ActionPriority>>,
    defender_query: Query<Entity, With<ArmorClass>>,
) {
    for (ac_event, atk_event) in ac_mod_finished
        .into_iter()
        .zip(atk_mod_finished.into_iter())
        .filter(|(ac_event, atk_event)| {
            ac_event.attacker == atk_event.attacker && ac_event.defender == atk_event.defender
        })
    {
        let debug = true;
        println!("debug | attack::attack_roll | start");
        let defender = defender_query.get(ac_event.defender).unwrap();
        let (attacker, attacker_bab) = attacker_query.get(atk_event.attacker).unwrap();

        let mut rng = rand::thread_rng();
        let attack_roll_raw = Dice::D20.roll_once(&mut rng);
        let total_attack_modifier = atk_event.total_attack_bonus + **attacker_bab;
        let total_defender_ac = 10 + ac_event.total_ac_bonus;

        let attack_roll_total: isize = attack_roll_raw as isize + total_attack_modifier;
        let attack_outcome = if attack_roll_raw == 20 {
            AttackOutcome::CritHit
        } else if attack_roll_raw == 1 {
            AttackOutcome::CritMiss
        } else if total_defender_ac <= attack_roll_total {
            AttackOutcome::Hit
        } else {
            AttackOutcome::Miss
        };

        let attacker_weapon: Option<Weapon> =
            if &ac_event.attacker_weapon == &atk_event.attacker_weapon {
                Some(ac_event.attacker_weapon.clone())
            } else {
                None
            };

        if debug {
            debug_attack_roll(
                attack_roll_raw,
                attacker_bab,
                atk_event,
                ac_event,
                attack_roll_total,
                attack_outcome,
                attacker_weapon.clone().unwrap(),
            );
        }

        attack_roll_event_writer.send(AttackRollEvent {
            attacker,
            defender,
            attack_outcome,
            attack_roll_raw,
            attack_roll_total,
            total_attack_modifier,
            total_defender_ac,
            attacker_weapon: attacker_weapon.unwrap(),
        });
    }
}

fn debug_attack_roll(
    attack_roll: usize,
    attacker_bab: &BaseAttackBonus,
    atk_event: &AttackBonusSumEvent,
    ac_event: &ACBonusSumEvent,
    attack_roll_total: isize,
    attack_outcome: AttackOutcome,
    attacker_weapon: Weapon,
) {
    println!("      |                     | D20 roll: {}", attack_roll);
    println!("      |                     | BAB: {}", **attacker_bab);
    println!(
        "      |                     | summed attack modifiers: {}",
        atk_event.total_attack_bonus
    );
    println!(
        "      |                     | attack bonus total: {}",
        atk_event.total_attack_bonus + **attacker_bab
    );
    println!(
        "      |                     | defender AC bonus: {}",
        ac_event.total_ac_bonus
    );
    println!(
        "      |                     | defender AC total: {}",
        10 + ac_event.total_ac_bonus
    );
    println!(
        "      |                     | total attack roll with bonuses: {}",
        attack_roll_total
    );
    println!(
        "      |                     | attack outcome: {:?}",
        attack_outcome
    );
    println!(
        "      |                     | attacker_weapon: {:?}",
        attacker_weapon
    );
}

pub fn debug_attack_roll_event(mut attack_roll_event_reader: EventReader<AttackRollEvent>) {
    let debug = true;
    if debug {
        for event in attack_roll_event_reader.iter() {
            println!("attack roll outcome: {:?}", event.attack_outcome);
        }
    }
}
