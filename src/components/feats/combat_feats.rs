use bevy::prelude::*;

use crate::{
    plugins::combat::{
        attack_modifier::AttackMod,
        bonus::{BonusSource, BonusType},
        critical_range_modifier::{CritThreatBonusSource, CritThreatBonusType, CritThreatMod},
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
        attacker_weapon: Entity,
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

#[derive(Component, Clone, Deref, DerefMut)]
pub struct ImprovedCritical {
    weapons: Vec<WeaponName>,
}

impl From<&ImprovedCritical> for CritThreatBonusSource {
    fn from(_val: &ImprovedCritical) -> Self {
        CritThreatBonusSource::ImprovedCritical
    }
}

impl From<&ImprovedCritical> for CritThreatBonusType {
    fn from(_val: &ImprovedCritical) -> Self {
        CritThreatBonusType::DoubleRange
    }
}

impl ImprovedCritical {
    pub fn new(weapons: Vec<WeaponName>) -> Self {
        Self { weapons }
    }

    pub fn to_crit_range_mod(
        &self,
        attacker: Entity,
        defender: Entity,
        attacker_weapon: Entity,
        weapon: &Weapon,
    ) -> Option<CritThreatMod> {
        if self.as_slice().contains(&weapon.weapon_name) {
            let original_range = weapon.crit_threat_range();
            let crit_range_n = original_range[1] - original_range[0] + 1;
            let bonus_type: CritThreatBonusType = self.into();
            let source: CritThreatBonusSource = self.into();
            Some(CritThreatMod {
                val: crit_range_n,
                source,
                bonus_type,
                attacker,
                defender,
                attacker_weapon,
            })
        } else {
            None
        }
    }
}
