use bevy::prelude::*;

use crate::{
    plugins::combat::{
        attack_modifier::AttackMod,
        bonus::{BonusSource, BonusType},
    },
    resources::equipment::weapon::{Weapon, WeaponName},
};

#[derive(Component, Clone)]
pub struct WeaponFocus {
    val: u8,
    weapons: Vec<WeaponName>,
}

impl WeaponFocus {
    pub fn new(val: u8, weapons: Vec<WeaponName>) -> Self {
        Self { val, weapons }
    }
    pub fn bonus(&self) -> isize {
        self.val as isize
    }

    pub fn to_atk_mod(
        &self,
        attacker: Entity,
        defender: Entity,
        attacker_weapon: Weapon,
    ) -> AttackMod {
        AttackMod {
            val: self.bonus(),
            source: BonusSource::WeaponFocus,
            bonus_type: BonusType::Untyped,
            attacker,
            defender,
            attacker_weapon,
        }
    }

    pub fn contains(&self, other: &WeaponName) -> bool {
        self.weapons.as_slice().contains(other)
    }
}
