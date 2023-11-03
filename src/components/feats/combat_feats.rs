use bevy::prelude::*;

use crate::plugins::combat::{
    attack_modifiers::{AttackModifier, AttackModifierSource},
    bonus::BonusType,
};

#[derive(Component, Clone, Copy, Deref, DerefMut)]
pub struct WeaponFocus(usize);

impl WeaponFocus {
    pub fn to_atk_mod(self, attacker: Entity, defender: Entity) -> AttackModifier {
        AttackModifier {
            val: *self as isize,
            source: AttackModifierSource::WeaponFocus,
            bonus_type: BonusType::Untyped,
            attacker,
            defender,
        }
    }
}
