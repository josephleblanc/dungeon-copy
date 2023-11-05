use serde::{Deserialize, Serialize};
use std::ops::Range;

use bevy::prelude::*;
use rand::prelude::*;

use crate::{config::TILE_SIZE, resources::dice::Dice};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Copy)]
pub enum WeaponName {
    Longsword,
}

#[derive(Component, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Weapon {
    pub weapon_name: WeaponName,
    attack_bonus: isize,
    damage_bonus: isize,
    base_damage_dice: Dice,
    crit_threat_range: [usize; 2],
    reach: Reach,
    crit_multiplier: u8,
    melee: bool,
    thrown: bool,
    // TODO: maybe change this to an array with bool values to make Weapon `Copy`
    racial_group: Option<RacialWeapon>,
    martial_group: Proficiency,
    weapon_damage_types: WeaponDamageTypes,
    // TODO: maybe change this to an array with bool values to make Weapon `Copy`
    weapon_groups: Option<Vec<WeaponGroup>>,
}

impl Weapon {
    pub fn crit_threat_range(&self) -> [usize; 2] {
        self.crit_threat_range
    }

    pub fn crit_threat_lower(&self) -> usize {
        self.crit_threat_range[1] - self.crit_threat_range[0] + 1
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
/// The types of damage this weapon can do.
pub struct WeaponDamageTypes(Vec<DamageType>);

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
/// The types of damage, e.g. Slashing, Piercing, etc.
pub enum DamageType {
    Slashing,
    Piercing,
    Blunt,
}

/// The `WeaponGroup` of a weapon is used for class features like Fighter's
/// Weapon Mastery, and some feats.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
pub enum WeaponGroup {
    HeavyBlades,
    // more here
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone)]
// TODO: fill out the possible racial weapon values
/// Placeholder for now
/// The racial weapon group with which the weapon is associated.
/// For example, Elves gain proficiency in a group of weapons (Longsword, Longbow, etc)
pub enum RacialWeapon {
    Elf,
    Orc,
    // More here
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Proficiency {
    Simple,
    Martial,
    Exotic,
}

#[derive(Debug, Deref, DerefMut, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
/// The reach of a weapon, in feet. This value can be easily converted into
/// pixels using its `.to_pixels()` method, which multiplies by the constant
/// TILE_SIZE.
pub struct Reach(usize);

impl Reach {
    pub fn to_pixels(self) -> f32 {
        (*self / 5_usize) as f32 * TILE_SIZE
    }
}

#[derive(Bundle)]
pub struct WeaponBundle {
    pub weapon: Weapon,
    // more here, e.g. enchantments
}
