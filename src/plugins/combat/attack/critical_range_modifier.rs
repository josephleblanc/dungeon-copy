use bevy::prelude::*;

use crate::{
    components::feats::combat_feats::ImprovedCritical, plugins::player::control::ActionPriority,
    resources::equipment::weapon::Weapon,
};

use crate::plugins::combat::{AttackData, AttackDataEvent};

#[derive(Copy, Clone, Debug)]
pub struct CritRangeMod {
    pub val: usize,
    pub source: CritThreatBonusSource,
    pub bonus_type: CritThreatBonusType,
    pub attack_data: AttackData,
}

impl CritRangeMod {
    pub fn base(attack_data: AttackData, attacker_weapon_stats: &Weapon) -> Self {
        Self {
            val: attacker_weapon_stats.crit_threat_lower(),
            source: CritThreatBonusSource::default(),
            bonus_type: CritThreatBonusType::default(),
            attack_data,
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub enum CritThreatBonusSource {
    ImprovedCritical,
    #[default]
    None, // more here
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum CritThreatBonusType {
    DoubleRange,
    #[default]
    None,
}

impl CritThreatBonusType {
    pub fn non_stackable() -> [CritThreatBonusType; 2] {
        [Self::DoubleRange, Self::None]
    }
}

#[derive(Event, Clone, Copy, Deref, DerefMut)]
pub struct CritRangeModEvent(CritRangeMod);

#[derive(Debug, Deref)]
pub struct CritRangeModList(Vec<CritRangeMod>);

impl CritRangeModList {
    fn new() -> CritRangeModList {
        CritRangeModList(Vec::new())
    }

    fn add(&mut self, elem: CritRangeMod) {
        self.0.push(elem);
    }

    // There is only one stackable modifier to critical threat range, and it is a level 20 capstone
    // for one archetype of Swashbuckler (Inspired Blade). I'll implement this later.
    // TODO: Implement this
    // fn sum_stackable(&self) -> isize {
    //     let debug = false;
    //     let mut total = 0;
    //     for bonus_type in CritThreatBonusType::stackable() {
    //         total += self
    //             .iter()
    //             .filter(|atk_mod| atk_mod.bonus_type == bonus_type)
    //             .fold(0, |acc, x| acc + x.val);
    //         if debug {
    //             debug_sum_stackable(bonus_type, total);
    //         }
    //     }
    //     total
    // }
    // pub fn sum_all(&self) -> isize {
    //     self.sum_stackable() + self.sum_non_stackable()
    // }

    pub fn sum_non_stackable(&self) -> usize {
        let debug = false;
        let mut total = 0;
        for bonus_type in CritThreatBonusType::non_stackable() {
            if let Some(highest_modifier) = self
                .iter()
                .filter(|atk_mod| atk_mod.bonus_type == bonus_type)
                .max_by(|x, y| x.val.cmp(&y.val))
            {
                total += highest_modifier.val;
                if debug {
                    debug_sum_non_stackable(bonus_type, total);
                }
            }
        }
        total
    }

    pub fn verified_data(&self) -> Result<AttackData, &'static str> {
        if self.is_empty() {
            Err("Attempted to verify an empty list of CritRangeModList. \
                CritRangeModList must have at least one element")
        } else if self
            .iter()
            .any(|crit_threat_mod| crit_threat_mod.attack_data != self[0].attack_data)
        {
            Err("Mismatched data in CritRangeModList")
        } else {
            Ok(self[0].attack_data)
        }
    }

    // pub fn verified_attacker(&self) -> Option<Entity> {
    //     if self.is_empty()
    //         || self
    //             .iter()
    //             .any(|atk_mod| atk_mod.attacker != self[0].attacker)
    //     {
    //         None
    //     } else {
    //         Some(self[0].attacker)
    //     }
    // }
    //
    // pub fn verified_defender(&self) -> Option<Entity> {
    //     if self.is_empty()
    //         || self
    //             .iter()
    //             .any(|atk_mod| atk_mod.defender != self[0].defender)
    //     {
    //         None
    //     } else {
    //         Some(self[0].defender)
    //     }
    // }
    //
    // pub fn verified_weapon(&self) -> Option<Entity> {
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

fn debug_sum_non_stackable(bonus_type: CritThreatBonusType, total: usize) {
    println!(
        "debug | attack_modifier::sum_non_stackable| bonus type: {:?}, total: {}",
        bonus_type, total
    );
}

impl FromIterator<CritRangeMod> for CritRangeModList {
    fn from_iter<I: IntoIterator<Item = CritRangeMod>>(iter: I) -> Self {
        let mut c = CritRangeModList::new();

        for i in iter {
            c.add(i);
        }

        c
    }
}

/// Adds the base critical threat range for the weapon used in an attack.
/// This system exists to make sure `critical_range::sum_crit_range_mods` has at least one
/// `CritRangeModEvent` to receive and run.
pub fn base(
    mut attack_data_event: EventReader<AttackDataEvent>,
    mut crit_mod_writer: EventWriter<CritRangeModEvent>,
    weapon_query: Query<&Weapon>,
) {
    for attack_data in attack_data_event.into_iter() {
        // println!("debug | critical_range_mod::base | start base");

        let weapon = weapon_query.get(attack_data.weapon_slot.entity).unwrap();

        crit_mod_writer.send(CritRangeModEvent(CritRangeMod::base(**attack_data, weapon)));
    }
}

/// Adds the weapon crit threat range increase for the `combat_feats::ImprovedCritical` feat.
/// This will only run if the attacker entity has the `ImporovedCritical` feat as a component.
pub fn improved_critical(
    mut attack_data_event: EventReader<AttackDataEvent>,
    mut crit_mod_writer: EventWriter<CritRangeModEvent>,
    attacker_query: Query<&ImprovedCritical, With<ActionPriority>>,
    weapon_query: Query<&Weapon>,
) {
    let debug = false;
    for attack_data in attack_data_event.iter() {
        if debug {
            println!("debug | critical_range_mod::base | start improved_critical");
        }
        let weapon = weapon_query.get(attack_data.weapon_slot.entity).unwrap();
        if let Ok(improved_critical) = attacker_query.get(attack_data.attacker) {
            if let Some(modifier) = improved_critical.to_crit_range_mod(**attack_data, weapon) {
                crit_mod_writer.send(CritRangeModEvent(modifier));
            }
        }
    }
}
