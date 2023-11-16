use bevy::prelude::*;

use crate::components::creature::Creature;

pub mod initiative_modifier;

#[derive(Event, Clone, Copy, Deref, DerefMut)]
pub struct StartInitiative(Entity);

#[derive(Clone, Copy, Deref, DerefMut)]
pub struct Initiative(isize);

pub fn start_initiative(query_creatures: Query<Entity, With<Creature>>) {}
