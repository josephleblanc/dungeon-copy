use bevy::prelude::*;

use crate::{
    components::{armor_class::ArmorClass, attack_bonus::BaseAttackBonus, player::PlayerComponent},
    plugins::{
        interact::{InteractingPos, InteractingType},
        player::control::ActionPriority,
    },
};

use super::{
    attack_modifiers::{AttackModifier, AttackModifierEvent, AttackModifierList},
    bonus::BonusType,
};

#[derive(Clone, Event)]
pub struct AttackRollEvent {
    base_attack_bonus: BaseAttackBonus,
    pub attacker: Entity,
    pub defender: Entity,
}

#[derive(Clone, Event)]
pub struct AttackBonusSumEvent {
    attacker: Entity,
    total_attack_bonus: isize,
    defender: Entity,
}

impl AttackRollEvent {
    pub fn new(base_attack_bonus: BaseAttackBonus, attacker: Entity, defender: Entity) -> Self {
        Self {
            base_attack_bonus,
            attacker,
            defender,
        }
    }
}

#[derive(Clone, Event)]
pub struct AttackRollComplete();

#[derive(Event)]
pub struct StartAttack;

pub fn check_attack_conditions(
    interacting_pos: Res<InteractingPos>,
    mut event_writer: EventWriter<StartAttack>,
    button: Res<Input<MouseButton>>,
) {
    if interacting_pos.interacting_type == InteractingType::Enemy
        && interacting_pos.entity.is_some()
        && button.just_pressed(MouseButton::Left)
    {
        // TODO: Check if target is in range
        event_writer.send(StartAttack);
    }
}

pub fn start_attack(
    mut start_attack_events: EventReader<StartAttack>,
    mut attack_roll_event: EventWriter<AttackRollEvent>,
    query_player: Query<(Entity, &BaseAttackBonus), (With<ActionPriority>)>,
    interacting_pos: Res<InteractingPos>,
) {
    if !start_attack_events.is_empty() {
        start_attack_events.clear();
        let enemy_entity = interacting_pos.entity.unwrap();
        let (attacking_player_entity, base_attack_bonus) = query_player.get_single().unwrap();
        let event = AttackRollEvent {
            base_attack_bonus: *base_attack_bonus,
            attacker: attacking_player_entity,
            defender: enemy_entity,
        };
        attack_roll_event.send(event);
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
