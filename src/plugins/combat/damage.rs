use bevy::prelude::*;

use crate::{components::armor_class::ArmorClass, plugins::player::control::ActionPriority};

use super::attack::AttackRollEvent;

#[derive(Event, Copy, Clone)]
pub struct DamageStartEvent {
    attacker: Entity,
    defender: Entity,
}

pub fn start_damage(
    mut attack_roll_event_writer: EventReader<AttackRollEvent>,
    attacker_query: Query<Entity, With<ActionPriority>>,
    defender_query: Query<Entity, With<ArmorClass>>,
) {
}
