#![allow(dead_code)]
use bevy::prelude::*;
use std::cmp::Ordering;

use crate::{
    components::attributes::{Attribute, Strength},
    plugins::{
        combat::{attack::AttackData, bonus::BonusType, CompleteAttackEvent},
        player::equipment::WeaponSlotName,
    },
    resources::dice::Dice,
};

use super::damage::DamageBonusSource;

#[derive(Copy, Clone, Deref, DerefMut)]
pub struct AttackDamageMod {
    #[deref]
    val: isize,
    attack_data: AttackData,
    bonus_type: BonusType,
    bonus_source: DamageBonusSource,
    damage_crit_type: DamageCritType,
    damage_dice: Option<DamageDice>,
}

#[derive(Copy, Clone)]
pub struct DamageDice {
    dice: Dice,
    dice_rolls: usize,
    bonus_per_roll: isize,
}

impl DamageDice {
    pub fn roll(self) -> usize {
        let mut rng = rand::thread_rng();
        match self.dice_rolls.cmp(&1) {
            Ordering::Greater => self.dice.roll_n(&mut rng, self.dice_rolls),
            Ordering::Equal => self.dice.roll_once(&mut rng),
            Ordering::Less => panic!("Attempted to roll 0 dice. Must roll at least 1 dice."),
        }
    }

    pub fn roll_with_bonus(self) -> isize {
        self.roll() as isize + self.bonus_per_roll * self.dice_rolls as isize
    }
}

#[derive(Copy, Clone)]
/// The way this damage interacts with criticals hits.
pub enum DamageCritType {
    /// The damage is only applied on a critical hit.
    OnlyOn,
    /// The damage can be multiplied on a critical hit.
    CanMultiply,
    /// The damage cannot be multiplied on a critical hit.
    CannotMultiply,
}

impl AttackDamageMod {
    pub fn get_data(&self) -> AttackData {
        self.attack_data
    }
}

#[derive(Event, Copy, Clone)]
pub struct AttackDamageModEvent(AttackDamageMod);

pub fn base(
    mut attack_reader: EventReader<CompleteAttackEvent>,
    mut damage_mod_writer: EventWriter<AttackDamageModEvent>,
) {
    for attack in attack_reader.into_iter() {
        let damage_mod = AttackDamageMod {
            val: 0,
            attack_data: attack.attack_data,
            bonus_type: BonusType::Untyped,
            bonus_source: DamageBonusSource::Base,
            damage_crit_type: DamageCritType::CannotMultiply,
            damage_dice: None,
        };
        damage_mod_writer.send(AttackDamageModEvent(damage_mod));
    }
}

pub fn add_strength(
    mut attack_reader: EventReader<CompleteAttackEvent>,
    mut damage_mod_writer: EventWriter<AttackDamageModEvent>,
    query_attacker: Query<&Strength>,
) {
    for attack in attack_reader.into_iter() {
        let mut damage_mod = AttackDamageMod {
            val: 0,
            attack_data: attack.attack_data,
            bonus_type: BonusType::Strength,
            bonus_source: DamageBonusSource::Strength,
            damage_crit_type: DamageCritType::CanMultiply,
            damage_dice: None,
        };
        let strength = query_attacker.get(attack.attack_data.attacker).unwrap();
        *damage_mod = match attack.attack_data.weapon_slot.slot {
            WeaponSlotName::TwoHanded | WeaponSlotName::NaturalOnly => {
                strength.bonus() + strength.bonus() / 2
            }
            WeaponSlotName::OffHand | WeaponSlotName::NaturalSecondary => strength.bonus() / 2,
            _ => strength.bonus(),
        };
        damage_mod_writer.send(AttackDamageModEvent(damage_mod));
    }
}

#[derive(Deref, DerefMut)]
pub struct AttackDamageModList(Vec<AttackDamageMod>);

impl AttackDamageModList {
    fn new() -> AttackDamageModList {
        AttackDamageModList(Vec::new())
    }

    fn add(&mut self, elem: AttackDamageMod) {
        self.0.push(elem);
    }

    fn sum_stackable(&self) -> isize {
        let debug = false;
        let mut total = 0;
        for bonus_type in BonusType::stackable() {
            total += self
                .iter()
                .filter(|dmg_mod| dmg_mod.bonus_type == bonus_type)
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
                .filter(|dmg_mod| dmg_mod.bonus_type == bonus_type)
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
            Err("Attempted to verify an empty list of AttackDamageMods. \
                AttackDamageModList must have at least one element")
        } else if self
            .iter()
            .any(|atk_mod| atk_mod.attack_data != self[0].attack_data)
        {
            Err("Mismatched data in AttackDamageModList")
        } else {
            Ok(self[0].attack_data)
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

impl FromIterator<AttackDamageMod> for AttackDamageModList {
    fn from_iter<I: IntoIterator<Item = AttackDamageMod>>(iter: I) -> Self {
        let mut c = AttackDamageModList::new();

        for i in iter {
            c.add(i);
        }

        c
    }
}
