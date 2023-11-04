use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Clone, Copy, Debug, Serialize, Deserialize)]
/// A marker struct for creatures.
/// This can be helpful for queries on entities that can be attacked or interacted with as
/// creatures, as opposed to items or ground tiles.
pub struct Creature;
