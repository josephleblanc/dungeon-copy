use serde::{Deserialize, Serialize};
use std::ops::Range;
use std::slice::Iter;

use bevy::prelude::*;
use rand::prelude::*;

use crate::{
    config::TILE_SIZE,
    plugins::{
        combat::{
            attack::crit_multiplier::CritMultiplier,
            attack_damage::{damage_reduction::DRTypes, damage_reduction_modifier::DRModList},
        },
        item::equipment::weapon,
    },
    resources::dice::Dice,
};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Copy)]
pub enum WeaponName {
    Longsword,
}

#[derive(Component, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Weapon {
    pub weapon_name: WeaponName,
    attack_bonus: isize,
    damage_bonus: isize,
    pub damage_dice: Dice,
    pub dice_rolls: usize,
    crit_threat_range: [usize; 2],
    reach: Reach,
    pub crit_multiplier: CritMultiplier,
    melee: bool,
    thrown: bool,
    // TODO: maybe change this to an array with bool values to make Weapon `Copy`
    racial_group: Option<RacialWeapon>,
    martial_group: Proficiency,
    pub weapon_damage_types: WeaponDamageTypes,
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

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy, Deref)]
/// The types of damage this weapon can do.
pub struct WeaponDamageTypes([(self::DamageType, bool); 3]);

impl WeaponDamageTypes {
    pub fn best_dr_val(self, dr_list: DRModList) -> Option<usize> {
        self.iter()
            .map(|wpn_dmg_type| dr_list.sum_type(wpn_dmg_type.0))
            .min()
    }

    pub fn best_dr_type(self, dr_list: DRModList) -> self::DamageType {
        self.iter()
            .map(|wpn_dmg_type| (wpn_dmg_type.0, dr_list.sum_type(wpn_dmg_type.0)))
            .min_by(|x, y| x.1.cmp(&y.1))
            .unwrap()
            .0
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
/// The types of damage, e.g. Slashing, Piercing, etc.
pub enum DamageType {
    Slashing,
    Piercing,
    Blunt,
}

impl DamageType {
    pub fn iterator() -> Iter<'static, Self> {
        [Self::Blunt, Self::Slashing, Self::Piercing].iter()
    }
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
