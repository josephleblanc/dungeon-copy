use bevy::prelude::*;

use crate::resources::dice::Dice;
use crate::{
    components::{armor_class::ArmorClass, attack_bonus::BaseAttackBonus, player::PlayerComponent},
    plugins::{
        interact::{InteractingPos, InteractingType},
        player::control::ActionPriority,
    },
};

use super::{
    armor_class::{ACBonusEvent, ACBonusSumEvent},
    attack_modifiers::{AttackModifier, AttackModifierEvent, AttackModifierList},
    bonus::BonusType,
};

#[derive(Clone, Event)]
pub struct AttackBonusEvent {
    pub attacker: Entity,
    pub defender: Entity,
}

#[derive(Event)]
pub struct StartAttack;

#[derive(Clone, Event)]
pub struct AttackBonusSumEvent {
    attacker: Entity,
    total_attack_bonus: isize,
    defender: Entity,
}

#[derive(Clone, Event)]
pub struct AttackRollComplete();

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
/// and then send an event listened to by `sum_attack_modifiers`.
pub fn start_attack(
    mut start_attack_events: EventReader<StartAttack>,
    mut attack_event_writer: EventWriter<AttackBonusEvent>,
    mut ac_event_writer: EventWriter<ACBonusEvent>,
    query_player: Query<Entity, With<ActionPriority>>,
    interacting_pos: Res<InteractingPos>,
) {
    if !start_attack_events.is_empty() {
        start_attack_events.clear();

        println!("debug | attack::start_attack | start");

        let attacker = query_player.get_single().unwrap();
        let defender = interacting_pos.entity.unwrap();
        attack_event_writer.send(AttackBonusEvent { attacker, defender });
        ac_event_writer.send(ACBonusEvent { attacker, defender });
    }
}

pub fn sum_attack_modifiers(
    mut atk_mod_events: EventReader<AttackModifierEvent>,
    attacker_query: Query<Entity, With<ActionPriority>>,
    mut atk_mod_finished: EventWriter<AttackBonusSumEvent>,
) {
    let atk_mod_list: AttackModifierList = atk_mod_events
        .into_iter()
        .map(|&event| event.into())
        .collect();
    if !atk_mod_list.is_empty() {
        println!("debug | attack::sum_attack_modifiers | start");
        if let Ok(attacker_entity) = attacker_query.get_single() {
            let attacker = atk_mod_list.verified_attacker().unwrap();
            let defender = atk_mod_list.verified_defender().unwrap();
            let sum_event = AttackBonusSumEvent {
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
}

pub struct DamageStartEvent {
    attacker: Entity,
    defender: Entity,
}

#[derive(Debug, Copy, Clone)]
pub enum AttackOutcome {
    CriticalHit,
    Hit,
    Miss,
    CriticalMiss,
}

#[derive(Debug, Event, Copy, Clone)]
pub struct AttackRollEvent {
    attacker: Entity,
    defender: Entity,
    attack_outcome: AttackOutcome,
}

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
        println!("debug | attack::attack_roll | start");
        let defender = defender_query.get(ac_event.defender).unwrap();
        let (attacker, attacker_bab) = attacker_query.get(atk_event.attacker).unwrap();

        let mut rng = rand::thread_rng();
        let attack_roll = Dice::D20.roll_once(&mut rng);

        let attack_roll_total: isize =
            attack_roll as isize + atk_event.total_attack_bonus + **attacker_bab;
        let attack_outcome = if attack_roll == 20 {
            AttackOutcome::CriticalHit
        } else if attack_roll == 1 {
            AttackOutcome::CriticalMiss
        } else if 10 + ac_event.total_ac_bonus <= attack_roll_total {
            AttackOutcome::Hit
        } else {
            AttackOutcome::Miss
        };

        attack_roll_event_writer.send(AttackRollEvent {
            attacker,
            defender,
            attack_outcome,
        });
    }
}

pub fn debug_attack_roll_event(mut attack_roll_event_reader: EventReader<AttackRollEvent>) {
    for event in attack_roll_event_reader.iter() {
        println!("attack roll outcome: {:?}", event.attack_outcome);
    }
}
