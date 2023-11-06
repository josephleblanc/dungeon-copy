use bevy::prelude::*;

use crate::components::attack_bonus::BaseAttackBonus;
use crate::plugins::combat::bonus::BonusSource;
use crate::resources::equipment::weapon::Weapon;
use crate::{
    components::{
        attributes::{Attribute, Strength},
        feats::combat_feats::WeaponFocus,
    },
    plugins::{combat::bonus::BonusType, player::control::ActionPriority},
};

use super::attack_roll::AttackBonusEvent;
use super::{AttackData, AttackDataEvent};

// TODO: Add a corresponding trait for this, then impl it for all the modifiers,
// and use that to make the systems to track them.
#[derive(Copy, Clone, Debug)]
pub struct AttackMod {
    pub val: isize,
    pub source: BonusSource,
    pub bonus_type: BonusType,
    pub attack_data: AttackData,
}

impl AttackMod {
    pub fn add_attribute_bonus<T>(&mut self, attribute: T)
    where
        T: Attribute,
        usize: std::convert::From<T>,
    {
        self.val += attribute.bonus();
    }
}

impl From<AttackMod> for usize {
    fn from(value: AttackMod) -> Self {
        value.val as usize
    }
}

impl From<AttackMod> for isize {
    fn from(value: AttackMod) -> Self {
        value.val
    }
}

impl From<AttackMod> for AttackModEvent {
    fn from(value: AttackMod) -> Self {
        AttackModEvent(value)
    }
}

#[derive(Event, Clone, Deref, DerefMut)]
pub struct AttackModEvent(AttackMod);

impl From<AttackModEvent> for AttackMod {
    fn from(value: AttackModEvent) -> Self {
        value.0
    }
}

pub fn base_attack_bonus(
    mut attack_data_event: EventReader<AttackDataEvent>,
    mut attack_bonus_event: EventReader<AttackBonusEvent>,
    bab_query: Query<&BaseAttackBonus>,
    mut event_writer: EventWriter<AttackModEvent>,
) {
    let debug = false;
    for (attack_data, _) in attack_data_event.iter().zip(attack_bonus_event.iter()) {
        if debug {
            println!("debug | attack_modifier::add_strength | start");
        }
        let attacker_bab = bab_query.get(attack_data.attacker).unwrap();
        let attack_modifier = AttackMod {
            val: **attacker_bab,
            source: BonusSource::Strength,
            bonus_type: BonusType::Untyped,
            attack_data: **attack_data,
        };
        if debug {
            debug_base_attack_bonus(attack_modifier);
        }

        event_writer.send(attack_modifier.into());
    }
}

fn debug_base_attack_bonus(attack_modifier: AttackMod) {
    println!(
        "{:>6}|{:>31}| attacker_bab: {}",
        " ", " ", attack_modifier.val
    );
}

pub fn add_strength(
    mut attack_data_event: EventReader<AttackDataEvent>,
    mut attack_bonus_event: EventReader<AttackBonusEvent>,
    mut event_writer: EventWriter<AttackModEvent>,
    query_attacker: Query<&Strength, With<ActionPriority>>,
) {
    let debug = false;
    for (attack_data, _attack_bonus) in attack_data_event.iter().zip(attack_bonus_event.iter()) {
        if debug {
            println!("debug | attack_modifier::add_strength | start");
        }
        if let Ok(strength) = query_attacker.get_single() {
            let mut attack_modifier = AttackMod {
                val: 0,
                source: BonusSource::Strength,
                bonus_type: BonusType::Untyped,
                attack_data: **attack_data,
            };
            attack_modifier.add_attribute_bonus(*strength);
            if debug {
                debug_add_strength(attack_modifier);
            }

            event_writer.send(attack_modifier.into());
        }
    }
}

fn debug_add_strength(attack_modifier: AttackMod) {
    println!(
        "{:>6}|{:>32}| strength bonus added: {}",
        "", "", attack_modifier.val
    );
}

pub fn add_weapon_focus(
    mut attack_data_event: EventReader<AttackDataEvent>,
    mut attack_bonus_event: EventReader<AttackBonusEvent>,
    mut event_writer: EventWriter<AttackModEvent>,
    query_attacker: Query<&WeaponFocus, With<ActionPriority>>,
    query_weapon: Query<&Weapon>,
) {
    let debug = false;
    for (attack_data, _bonus_event) in attack_data_event.iter().zip(attack_bonus_event.iter()) {
        println!("debug | attack_modifier::add_weapon_focus | start");
        if let Ok(weapon_focus) = query_attacker.get(attack_data.attacker) {
            let weapon = query_weapon.get(attack_data.weapon_slot.entity).unwrap();
            if weapon_focus.contains(&weapon.weapon_name) {
                let attack_modifier = weapon_focus.clone().to_atk_mod(**attack_data);

                if debug {
                    debug_add_weapon_focus(attack_modifier);
                }
                event_writer.send(attack_modifier.into());
            }
        }
    }
}

fn debug_add_weapon_focus(attack_modifier: AttackMod) {
    println!(
        "{:>6}|{:>36}| weapon_focus bonus added: {}",
        "", "", attack_modifier.val
    );
}

#[derive(Debug, Deref)]
pub struct AttackModList(Vec<AttackMod>);

impl AttackModList {
    fn new() -> AttackModList {
        AttackModList(Vec::new())
    }

    fn add(&mut self, elem: AttackMod) {
        self.0.push(elem);
    }

    fn sum_stackable(&self) -> isize {
        let debug = false;
        let mut total = 0;
        for bonus_type in BonusType::stackable() {
            total += self
                .iter()
                .filter(|atk_mod| atk_mod.bonus_type == bonus_type)
                .fold(0, |acc, x| acc + x.val);
            if debug {
                debug_sum_stackable(bonus_type, total);
            }
        }
        total
    }

    fn sum_non_stackable(&self) -> isize {
        let debug = false;
        let mut total = 0;
        for bonus_type in BonusType::non_stackable() {
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

    pub fn sum_all(&self) -> isize {
        self.sum_stackable() + self.sum_non_stackable()
    }

    pub fn verified_data(&self) -> Result<AttackData, &'static str> {
        if self.is_empty() {
            Err("Attempted to verify an empty list of AttackMods. \
                AttackModList must have at least one element")
        } else if self
            .iter()
            .any(|atk_mod| atk_mod.attack_data != self[0].attack_data)
        {
            Err("Mismatched data in AttackModList")
        } else {
            Ok(self[0].attack_data)
        }
    }

    // TODO: Delete these once I feel good about the refactor
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
    //         Some(self[0].attacker_weapon)
    //     }
    // }
}

fn debug_sum_non_stackable(bonus_type: BonusType, total: isize) {
    println!(
        "debug | attack_modifier::sum_non_stackable| bonus type: {:?}, total: {}",
        bonus_type, total
    );
}

fn debug_sum_stackable(bonus_type: BonusType, total: isize) {
    println!(
        "debug | attack_modifier::sum_stackable| bonus type: {:?}, total: {}",
        bonus_type, total
    );
}

impl FromIterator<AttackMod> for AttackModList {
    fn from_iter<I: IntoIterator<Item = AttackMod>>(iter: I) -> Self {
        let mut c = AttackModList::new();

        for i in iter {
            c.add(i);
        }

        c
    }
}
