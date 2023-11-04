use bevy::prelude::*;

use crate::plugins::combat::bonus::BonusSource;
use crate::resources::equipment::weapon::Weapon;
use crate::{
    components::{
        attributes::{Attribute, Strength},
        feats::combat_feats::WeaponFocus,
    },
    plugins::{combat::bonus::BonusType, player::control::ActionPriority},
};

use super::attack::AttackBonusEvent;

// TODO: Add a corresponding trait for this, then impl it for all the modifiers,
// and use that to make the systems to track them.
#[derive(Clone, Debug)]
pub struct AttackMod {
    pub val: isize,
    pub source: BonusSource,
    pub bonus_type: BonusType,
    pub attacker: Entity,
    pub defender: Entity,
    pub attacker_weapon: Weapon,
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

pub fn add_strength(
    mut attack_roll_event: EventReader<AttackBonusEvent>,
    mut event_writer: EventWriter<AttackModEvent>,
    query_attacker: Query<&Strength, With<ActionPriority>>,
) {
    let debug = true;
    for attack_roll in attack_roll_event.iter() {
        if debug {
            println!("debug | attack_modifier::add_strength | start");
        }
        if let Ok(strength) = query_attacker.get_single() {
            let mut attack_modifier = AttackMod {
                val: 0,
                source: BonusSource::Strength,
                bonus_type: BonusType::Untyped,
                attacker: attack_roll.attacker,
                defender: attack_roll.defender,
                attacker_weapon: attack_roll.attacker_weapon.clone(),
            };
            attack_modifier.add_attribute_bonus(*strength);
            if debug {
                debug_add_strength(attack_modifier.clone());
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
    mut attack_roll_event: EventReader<AttackBonusEvent>,
    mut event_writer: EventWriter<AttackModEvent>,
    query_attacker: Query<&WeaponFocus, With<ActionPriority>>,
) {
    let debug = true;
    for attack_roll in attack_roll_event.iter() {
        println!("debug | attack_modifier::add_weapon_focus | start");
        if let Ok(weapon_focus) = query_attacker.get_single() {
            if weapon_focus.contains(&attack_roll.attacker_weapon.weapon_name) {
                let attack_modifier = weapon_focus.clone().to_atk_mod(
                    attack_roll.attacker,
                    attack_roll.defender,
                    attack_roll.attacker_weapon.clone(),
                );

                if debug {
                    debug_add_weapon_focus(attack_modifier.clone());
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
        let debug = true;
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
        let debug = true;
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

    pub fn verified_weapon(&self) -> Option<Weapon> {
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
