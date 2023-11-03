use bevy::prelude::*;

use crate::{components::armor_class::ArmorClass, plugins::player::control::ActionPriority};

use super::attack::AttackRollEvent;

#[derive(Event, Copy, Clone)]
pub struct DamageStartEvent {
    attacker: Entity,
    defender: Entity,
}
