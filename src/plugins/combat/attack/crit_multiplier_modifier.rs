#![allow(dead_code)]
use bevy::prelude::*;
use std::slice::Iter;

use super::crit_multiplier::CritMultiplier;

#[derive(Copy, Clone, Debug)]
pub struct CritMultiplierMod {
    attacker: Entity,
    defender: Entity,
    val: u8,
    source: CritMultiplierSource,
}

#[derive(Event, Copy, Clone, Debug, Deref)]
pub struct CritMultiplierModEvent(CritMultiplierMod);

#[derive(Debug, Deref)]
pub struct CritMultiplierModList(Vec<CritMultiplierMod>);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CritMultiplierSource {
    LethalAccuracy,
    MythicImprovedCritical,
}

impl CritMultiplierSource {
    pub fn iterator() -> Iter<'static, CritMultiplierSource> {
        [
            CritMultiplierSource::LethalAccuracy,
            CritMultiplierSource::MythicImprovedCritical,
        ]
        .iter()
    }
    pub fn stacks_with_others() -> [CritMultiplierSource; 1] {
        [CritMultiplierSource::LethalAccuracy]
    }

    // This is currently a vec, because when adding new sources it is possible to forget that they
    // have a limit, or include them in the wrong list. If all possible sources get implemented at
    // some point, then it will be possible to return and change this to an array.
    // TODO: Turn this into an array someday, see above note ^
    pub fn vec_sorted_by_limit() -> Vec<CritMultiplierSource> {
        let sorted_by_limit = [CritMultiplierSource::MythicImprovedCritical];
        let mut has_limit: Vec<CritMultiplierSource> = sorted_by_limit
            .iter()
            .copied()
            .filter(|source| source.limit().is_some())
            .collect();
        has_limit.sort_by_key(|x| x.limit().unwrap());
        has_limit
    }

    pub fn limit(self) -> Option<CritMultiplier> {
        match self {
            CritMultiplierSource::LethalAccuracy => None,
            CritMultiplierSource::MythicImprovedCritical => Some(CritMultiplier::X6),
        }
    }

    pub fn size(self) -> u8 {
        #[allow(clippy::match_single_binding)]
        match self {
            _ => 1,
            // more here
        }
    }
}

impl CritMultiplierModList {
    fn new() -> CritMultiplierModList {
        CritMultiplierModList(Vec::new())
    }

    fn add(&mut self, elem: CritMultiplierMod) {
        self.0.push(elem);
    }

    fn sum_with_limit(&self, base_crit: CritMultiplier) -> CritMultiplier {
        let debug = false;
        let mut sorting_vec: Vec<CritMultiplierMod> = (*self)
            .clone()
            .into_iter()
            .filter(|crit_mod| crit_mod.source.limit().is_some())
            .collect();
        sorting_vec
            .as_mut_slice()
            .sort_by(|x, y| x.source.limit().unwrap().cmp(&y.source.limit().unwrap()));
        let total = sorting_vec
            .into_iter()
            .fold(base_crit, |acc: CritMultiplier, crit_mod| {
                acc.increase_with_limit(crit_mod.source.limit())
            });
        if debug {
            debug_sum_stacks_with_others(base_crit, total);
        }
        total
    }

    fn sum_without_limit(&self, base_crit: CritMultiplier) -> CritMultiplier {
        let debug = false;
        let total: CritMultiplier = base_crit;
        (*self)
            .clone()
            .into_iter()
            .filter(|crit_mod| crit_mod.source.limit().is_none())
            .map(|crit_mod| crit_mod.source.size())
            .fold(base_crit, |acc, x| acc.increase_by(x));
        if debug {
            debug_sum_non_stackable(base_crit, total);
        }
        total
    }

    pub fn sum_all(&self, base_crit: CritMultiplier) -> CritMultiplier {
        let with_limit = self.sum_with_limit(base_crit);
        let without_limit = self.sum_without_limit(base_crit);
        if with_limit >= base_crit && without_limit <= base_crit {
            with_limit
        } else if with_limit <= base_crit && without_limit >= base_crit {
            without_limit
        } else {
            with_limit.add(without_limit)
        }
    }

    pub fn verified_attacker(&self) -> Option<Entity> {
        if self.is_empty()
            || self
                .iter()
                .any(|atk_mod| atk_mod.attacker != self[0].attacker)
        {
            None
        } else {
            Some(self[0].attacker)
        }
    }

    pub fn verified_defender(&self) -> Option<Entity> {
        if self.is_empty()
            || self
                .iter()
                .any(|atk_mod| atk_mod.defender != self[0].defender)
        {
            None
        } else {
            Some(self[0].defender)
        }
    }

    // should probably implement this, maybe do it later
    // pub fn verified_weapon(&self, weapon: &Weapon) -> Option<Weapon> {
    //     if self.is_empty()
    //         || self
    //             .iter()
    //             .any(|atk_mod| atk_mod.attacker_weapon != self[0].attacker_weapon)
    //     {
    //         None
    //     } else {
    //         Some(self[0].attacker_weapon.clone())
    //     }
    // }
}

fn debug_sum_non_stackable(base_crit: CritMultiplier, total: CritMultiplier) {
    println!(
        "debug | attack_modifier::sum_non_stackable| bonus type: {:?}, total: {:?}",
        base_crit, total
    );
}

fn debug_sum_stacks_with_others(bonus_type: CritMultiplier, total: CritMultiplier) {
    println!(
        "debug | crit_multiplier_modifier::sum_with_limit | base_crit: {:?}, total: {:?}",
        bonus_type, total
    );
}

impl FromIterator<CritMultiplierMod> for CritMultiplierModList {
    fn from_iter<I: IntoIterator<Item = CritMultiplierMod>>(iter: I) -> Self {
        let mut c = CritMultiplierModList::new();

        for i in iter {
            c.add(i);
        }

        c
    }
}
