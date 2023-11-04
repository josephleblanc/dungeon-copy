use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::resources::equipment::weapon::Weapon;

#[derive(Component, Serialize, Deserialize, PartialEq, Clone)]
pub struct EquippedWeapons {
    pub main_hand: Weapon,
    pub off_hand: Vec<Weapon>,
}
