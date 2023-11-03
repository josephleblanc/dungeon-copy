use bevy::prelude::*;

use crate::{
    components::{armor_class::ArmorClass, attack_bonus::BaseAttackBonus, player::PlayerComponent},
    plugins::{
        interact::{InteractingPos, InteractingType},
        player::control::ActionPriority,
    },
};

use super::{
    armor_class::ACBonusEvent,
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
    if let Ok(attacker_entity) = attacker_query.get_single() {
        let atk_mod_list: AttackModifierList = atk_mod_events
            .into_iter()
            .map(|&event| event.into())
            .collect();

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
