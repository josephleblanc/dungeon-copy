use serde::{Deserialize, Serialize};
use std::ops::Range;

use bevy::prelude::*;
use rand::prelude::*;

use crate::{config::TILE_SIZE, resources::dice::Dice};

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum WeaponName {
    Longsword,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Weapon {
    attack_bonus: isize,
    damage_bonus: isize,
    damage_dice_range: Dice,
    crit_threat_range: [usize; 2],
    reach: Reach,
    crit_multiplier: isize,
    melee: bool,
    thrown: bool,
    racial_group: Option<RacialWeapon>,
    martial_group: Proficiency,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash)]
// TODO: fill out the possible racial weapon values
/// Placeholder for now
pub enum RacialWeapon {
    Elf,
    Orc,
    // More here
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum Proficiency {
    Simple,
    Martial,
    Exotic,
}

#[derive(Deref, DerefMut, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
/// The reach of a weapon, in feet. This value can be easily converted into
/// pixels using its `.to_pixels()` method, which multiplies by the constant
/// TILE_SIZE.
pub struct Reach(usize);

impl Reach {
    pub fn to_pixels(self) -> f32 {
        (*self / 5_usize) as f32 * TILE_SIZE
    }
}
