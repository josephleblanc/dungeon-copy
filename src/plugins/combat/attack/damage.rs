use bevy::prelude::*;

#[derive(Event, Copy, Clone)]
pub struct DamageStartEvent {
    attacker: Entity,
    defender: Entity,
}
