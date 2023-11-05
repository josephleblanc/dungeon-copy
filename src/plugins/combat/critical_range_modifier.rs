use bevy::prelude::*;

use crate::{
    components::{creature::Creature, feats::combat_feats::ImprovedCritical},
    plugins::player::control::ActionPriority,
    resources::equipment::weapon::Weapon,
};

use super::attack::{AttackOutcome, AttackRollEvent};

#[derive(Copy, Clone, Debug)]
pub struct CritThreatMod {
    pub val: usize,
    pub source: CritThreatBonusSource,
    pub bonus_type: CritThreatBonusType,
    pub attacker: Entity,
    pub defender: Entity,
    pub attacker_weapon: Entity,
}

impl CritThreatMod {
    pub fn base(
        attacker: Entity,
        defender: Entity,
        attacker_weapon: Entity,
        attacker_weapon_stats: &Weapon,
    ) -> Self {
        Self {
            val: attacker_weapon_stats.crit_threat_lower(),
            source: CritThreatBonusSource::default(),
            bonus_type: CritThreatBonusType::default(),
            attacker,
            defender,
            attacker_weapon,
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
pub struct CritThreatModEvent(CritThreatMod);

#[derive(Debug, Deref)]
pub struct CritThreatModList(Vec<CritThreatMod>);

impl CritThreatModList {
    fn new() -> CritThreatModList {
        CritThreatModList(Vec::new())
    }

    fn add(&mut self, elem: CritThreatMod) {
        self.0.push(elem);
    }

    // There is only one stackable modifier to critical threat range, and it is a level 20 capstone
    // for one archetype of Swashbuckler (Inspired Blade). I'll implement this later.
    // TODO: Implement this
    // fn sum_stackable(&self) -> isize {
    //     let debug = true;
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
        let debug = true;
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

    pub fn verified_weapon(&self) -> Option<Entity> {
        if self.is_empty()
            || self
                .iter()
                .any(|atk_mod| atk_mod.attacker_weapon != self[0].attacker_weapon)
        {
            None
        } else {
            Some(self[0].attacker_weapon.clone())
        }
    }
}

fn debug_sum_non_stackable(bonus_type: CritThreatBonusType, total: usize) {
    println!(
        "debug | attack_modifier::sum_non_stackable| bonus type: {:?}, total: {}",
        bonus_type, total
    );
}

impl FromIterator<CritThreatMod> for CritThreatModList {
    fn from_iter<I: IntoIterator<Item = CritThreatMod>>(iter: I) -> Self {
        let mut c = CritThreatModList::new();

        for i in iter {
            c.add(i);
        }

        c
    }
}

/// Adds the base critical threat range for the weapon used in an attack.
/// This system exists to make sure `critical_range::sum_crit_range_mods` has at least one
/// `CritThreatModEvent` to receive and run.
pub fn base(
    mut attack_roll_reader: EventReader<AttackRollEvent>,
    attacker_query: Query<Entity, With<ActionPriority>>,
    defender_query: Query<Entity, With<Creature>>,
    weapon_query: Query<&Weapon>,
    mut crit_mod_writer: EventWriter<CritThreatModEvent>,
) {
    for roll_hit_event in attack_roll_reader
        .iter()
        .filter(|roll| roll.attack_outcome == AttackOutcome::Hit)
    {
        println!("debug | critical_range_mod::base | start base");
        let attacker = attacker_query.get(roll_hit_event.attacker).unwrap();
        let defender = defender_query.get(roll_hit_event.defender).unwrap();
        let weapon = weapon_query.get(roll_hit_event.attacker_weapon).unwrap();

        crit_mod_writer.send(CritThreatModEvent(CritThreatMod::base(
            attacker,
            defender,
            roll_hit_event.attacker_weapon,
            weapon,
        )));
    }
}

/// Adds the weapon crit threat range increase for the `combat_feats::ImprovedCritical` feat.
/// This will only run if the attacker entity has the `ImporovedCritical` feat as a component.
pub fn improved_critical(
    attacker_query: Query<&ImprovedCritical, With<ActionPriority>>,
    weapon_query: Query<&Weapon>,
    mut attack_roll_reader: EventReader<AttackRollEvent>,
    mut crit_mod_writer: EventWriter<CritThreatModEvent>,
) {
    for roll_hit_event in attack_roll_reader
        .iter()
        .filter(|roll| roll.attack_outcome == AttackOutcome::Hit)
    {
        println!("debug | critical_range_mod::base | start improved_critical");
        let weapon = weapon_query.get(roll_hit_event.attacker_weapon).unwrap();
        if let Ok(improved_critical) = attacker_query.get(roll_hit_event.attacker) {
            if let Some(modifier) = improved_critical.to_crit_range_mod(
                roll_hit_event.attacker,
                roll_hit_event.defender,
                roll_hit_event.attacker_weapon,
                weapon,
            ) {
                crit_mod_writer.send(CritThreatModEvent(modifier));
            }
        }
    }
}