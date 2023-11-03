use serde::{Deserialize, Serialize};
use std::ops::Range;

use bevy::prelude::*;
use rand::prelude::*;

use crate::{config::TILE_SIZE, resources::dice::Dice};

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum WeaponName {
    Longsword,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Weapon {
    attack_bonus: isize,
    damage_bonus: isize,
    base_damage_dice: Dice,
    crit_threat_range: [usize; 2],
    reach: Reach,
    crit_multiplier: isize,
    melee: bool,
    thrown: bool,
    racial_group: Option<RacialWeapon>,
    martial_group: Proficiency,
    weapon_damage_types: WeaponDamageTypes,
    weapon_groups: Option<Vec<WeaponGroup>>,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone)]
/// The types of damage this weapon can do.
pub struct WeaponDamageTypes(Vec<DamageType>);

#[derive(Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
/// The types of damage, e.g. Slashing, Piercing, etc.
pub enum DamageType {
    Slashing,
    Piercing,
    Blunt,
}

/// The `WeaponGroup` of a weapon is used for class features like Fighter's
/// Weapon Mastery, and some feats.
#[derive(Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
pub enum WeaponGroup {
    HeavyBlades,
    // more here
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone)]
// TODO: fill out the possible racial weapon values
/// Placeholder for now
/// The racial weapon group with which the weapon is associated.
/// For example, Elves gain proficiency in a group of weapons (Longsword, Longbow, etc)
pub enum RacialWeapon {
    Elf,
    Orc,
    // More here
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone)]
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
