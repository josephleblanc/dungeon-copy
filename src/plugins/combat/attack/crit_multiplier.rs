use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::slice::Iter;

use crate::{plugins::combat::AttackData, resources::equipment::weapon::Weapon};

use super::crit_multiplier_modifier::{CritMultiplierModEvent, CritMultiplierModList};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CritMultiplier {
    X2,
    X3,
    X4,
    X5,
    X6,
}

#[derive(Event, Copy, Clone, Deref)]
pub struct CritMultiplierSumEvent {
    #[deref]
    pub val: CritMultiplier,
    pub attack_data: AttackData,
}

pub fn sum_crit_multiplier(
    mut crit_mod_reader: EventReader<CritMultiplierModEvent>,
    mut sum_modifier_writer: EventWriter<CritMultiplierSumEvent>,
    weapon_query: Query<&Weapon>,
) {
    let crit_mod_list: CritMultiplierModList =
        crit_mod_reader.into_iter().map(|event| **event).collect();
    if !crit_mod_list.is_empty() {
        let attack_data = crit_mod_list.verified_data().unwrap();
        let base_crit = weapon_query
            .get(attack_data.weapon_slot.entity)
            .unwrap()
            .crit_multiplier;
        let sum_event = CritMultiplierSumEvent {
            attack_data,
            val: crit_mod_list.sum_all(base_crit),
        };

        sum_modifier_writer.send(sum_event);
    }
}

impl CritMultiplier {
    pub fn iterator() -> Iter<'static, CritMultiplier> {
        [
            CritMultiplier::X2,
            CritMultiplier::X3,
            CritMultiplier::X4,
            CritMultiplier::X5,
            CritMultiplier::X6,
        ]
        .iter()
    }
    pub fn size(self) -> usize {
        let (size, _) = CritMultiplier::iterator()
            .enumerate()
            .find(|(_i, val)| self == **val)
            .unwrap();
        size + 2
    }
    pub fn add(self, other: Self) -> Self {
        let index = self.size() + other.size();
        let (_, crit) = CritMultiplier::iterator()
            .enumerate()
            .find(|(i, _val)| index == *i)
            .unwrap();
        *crit
    }
    pub fn unchecked_add_one(self) -> Self {
        match self {
            CritMultiplier::X2 => CritMultiplier::X3,
            CritMultiplier::X3 => CritMultiplier::X4,
            CritMultiplier::X4 => CritMultiplier::X5,
            CritMultiplier::X5 => CritMultiplier::X6,
            CritMultiplier::X6 => CritMultiplier::X6,
        }
    }

    pub fn checked_add_one(self) -> Result<Self, &'static str> {
        match self {
            CritMultiplier::X2 => Ok(CritMultiplier::X3),
            CritMultiplier::X3 => Ok(CritMultiplier::X4),
            CritMultiplier::X4 => Ok(CritMultiplier::X5),
            CritMultiplier::X5 => Ok(CritMultiplier::X6),
            CritMultiplier::X6 => Err("Cannot increase critical past x6 you madman!!!"),
        }
    }

    pub fn increase_by(self, n: u8) -> Self {
        let mut out = self;
        for _ in 0..n {
            out = out.unchecked_add_one();
        }
        out
    }

    pub fn increase_with_limit(self, limit: Option<Self>) -> Self {
        let increased = self.unchecked_add_one();
        if let Some(crit_limit) = limit {
            if increased >= crit_limit {
                crit_limit
            } else {
                increased
            }
        } else {
            increased
        }
    }
}
