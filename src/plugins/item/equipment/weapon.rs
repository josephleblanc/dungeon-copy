use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, PartialEq, Clone)]
pub struct EquippedWeapons {
    pub main_hand: Entity,
    pub off_hand: Vec<Entity>,
}
