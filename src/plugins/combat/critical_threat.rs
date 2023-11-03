use bevy::prelude::*;

use crate::{components::armor_class::ArmorClass, plugins::player::control::ActionPriority};

use super::attack::AttackRollEvent;

pub fn start(
    mut attack_roll_reader: EventReader<AttackRollEvent>,
    attacker_query: Query<Entity, With<ActionPriority>>,
    defender_query: Query<Entity, With<ArmorClass>>,
) {
    for atk_roll_event in attack_roll_reader.iter() {}
}
