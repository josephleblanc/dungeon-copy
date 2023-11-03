use bevy::prelude::*;

use crate::plugins::combat::{
    attack_modifier::AttackMod,
    bonus::{BonusSource, BonusType},
};

#[derive(Component, Clone, Copy, Deref, DerefMut)]
pub struct WeaponFocus(usize);

impl WeaponFocus {
    pub fn to_atk_mod(self, attacker: Entity, defender: Entity) -> AttackMod {
        AttackMod {
            val: *self as isize,
            source: BonusSource::WeaponFocus,
            bonus_type: BonusType::Untyped,
            attacker,
            defender,
        }
    }
}
