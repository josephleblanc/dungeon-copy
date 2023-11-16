#![allow(dead_code)]
use bevy::prelude::*;
use std::cmp::Ordering;

use crate::{
    components::attributes::{Attribute, Strength},
    plugins::{
        combat::{bonus::BonusType, damage::DamageType, AttackData, AttackDataEvent},
        player::equipment::WeaponSlotName,
    },
    resources::{
        dice::Dice,
        equipment::weapon::{self, Weapon},
    },
};

use super::damage::DamageBonusSource;

#[derive(Debug, Copy, Clone)]
pub struct AttackDamageMod {
    pub val: isize,
    pub attack_data: AttackData,
    pub bonus_type: BonusType,
    pub damage_type: DamageType,
    pub bonus_source: DamageBonusSource,
    pub on_crit: OnCrit,
    pub damage_dice: Option<DamageDice>,
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// The way this damage interacts with criticals hits.
pub enum OnCrit {
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

#[derive(Event, Copy, Clone, Deref, DerefMut)]
pub struct AttackDamageModEvent(AttackDamageMod);

pub fn base(
    mut attack_reader: EventReader<AttackDataEvent>,
    mut damage_mod_writer: EventWriter<AttackDamageModEvent>,
) {
    for attack in attack_reader.read() {
        let damage_mod = AttackDamageMod {
            damage_type: DamageType::Weapon,
            val: 0,
            attack_data: **attack,
            bonus_type: BonusType::Untyped,
            bonus_source: DamageBonusSource::Base,
            on_crit: OnCrit::CannotMultiply,
            damage_dice: None,
        };
        damage_mod_writer.send(AttackDamageModEvent(damage_mod));
    }
}

pub fn weapon(
    mut attack_reader: EventReader<AttackDataEvent>,
    mut damage_mod_writer: EventWriter<AttackDamageModEvent>,
    weapon_query: Query<&Weapon>,
) {
    for data in attack_reader.read() {
        let weapon = weapon_query.get(data.weapon_slot.entity).unwrap();
        let damage_dice = DamageDice {
            dice: weapon.damage_dice,
            dice_rolls: weapon.dice_rolls,
            bonus_per_roll: 0,
        };

        // TODO: Add a way for weapons with variable damage types, like Slashing/Piercing, to do
        // the type of damage which the opponent does not have any DR for, once DR is implemented.
        let damage_mod = AttackDamageMod {
            damage_type: DamageType::Weapon,
            val: 0,
            attack_data: **data,
            bonus_type: BonusType::Untyped,
            bonus_source: DamageBonusSource::Weapon,
            on_crit: OnCrit::CanMultiply,
            damage_dice: Some(damage_dice),
        };
        damage_mod_writer.send(AttackDamageModEvent(damage_mod));
    }
}

pub fn add_strength(
    mut attack_reader: EventReader<AttackDataEvent>,
    mut damage_mod_writer: EventWriter<AttackDamageModEvent>,
    query_attacker: Query<&Strength>,
) {
    for attack in attack_reader.read() {
        let mut damage_mod = AttackDamageMod {
            damage_type: DamageType::Weapon,
            val: 0,
            attack_data: **attack,
            bonus_type: BonusType::Strength,
            bonus_source: DamageBonusSource::Strength,
            on_crit: OnCrit::CanMultiply,
            damage_dice: None,
        };
        let strength = query_attacker.get(attack.attacker).unwrap();
        damage_mod.val = match attack.weapon_slot.slot {
            WeaponSlotName::TwoHanded | WeaponSlotName::NaturalOnly => {
                strength.bonus() + strength.bonus() / 2
            }
            WeaponSlotName::OffHand | WeaponSlotName::NaturalSecondary => strength.bonus() / 2,
            _ => strength.bonus(),
        };
        // println!("||| strength mod | damage_mod.val: {}", damage_mod.val);
        damage_mod_writer.send(AttackDamageModEvent(damage_mod));
    }
}

#[derive(Deref, DerefMut, Clone, Debug)]
pub struct AttackDamageModList(Vec<AttackDamageMod>);

impl AttackDamageModList {
    fn new() -> AttackDamageModList {
        AttackDamageModList(Vec::new())
    }

    fn add(&mut self, elem: AttackDamageMod) {
        self.0.push(elem);
    }
    pub fn sum_all(&self) -> isize {
        let mut total_stackable: isize = 0;
        let mut total_non_stackable: isize = 0;
        for bonus_type in BonusType::stackable() {
            total_stackable += self
                .iter()
                // .inspect(|dmg_mod| {
                //     println!(
                //         "debug | damage::SumStackable::sum_stackable | checking {:?}\
                //         val: {}\
                //         dice: {:?}\
                //         dice.roll(): {:?}",
                //         dmg_mod.bonus_type,
                //         dmg_mod.val,
                //         dmg_mod.damage_dice,
                //         if dmg_mod.damage_dice.is_some() {
                //             Some(dmg_mod.damage_dice.unwrap().roll())
                //         } else {
                //             None
                //         }
                //     )
                // })
                .filter(|dmg_mod| {
                    // println!(
                    //     "||| dmg_mod.bonus_type == bonus_type: {}, \
                    // bonus_type: {:?}, dmg_mod.bonus_type: {:?}",
                    //     dmg_mod.bonus_type == bonus_type,
                    //     bonus_type,
                    //     dmg_mod.bonus_type,
                    // );
                    dmg_mod.bonus_type == bonus_type
                })
                .fold(0, |acc, x| {
                    let new_val = acc
                        + x.val
                        + if let Some(dice) = x.damage_dice {
                            dice.roll() as isize
                        } else {
                            0
                        };
                    // println!("|||> sum_stackable new_val: {}", new_val);
                    new_val
                });
        }
        for bonus_type in BonusType::non_stackable() {
            total_non_stackable += if let Some(total) = self
                .iter()
                // .inspect(|dmg_mod| {
                //     println!(
                //         "debug | damage_modifier::DamageModList::sum_non_stackable iterator | checking {:?}",
                //         dmg_mod.bonus_type
                //     )
                // })
                .filter(|dmg_mod| {
                    // println!(
                    //     "||| dmg_mod.bonus_type == bonus_type: {}, \
                    // bonus_type: {:?}, dmg_mod.bonus_type: {:?}",
                    //     dmg_mod.bonus_type == bonus_type,
                    //     bonus_type,
                    //     dmg_mod.bonus_type,
                    // );
                    dmg_mod.bonus_type == bonus_type
                })
                .map(|x| {
                    let new_val = x.val
                        + if let Some(dice) = x.damage_dice {
                            dice.roll() as isize
                        } else {
                            0
                        };
                    // println!("new_val: {}", new_val);
                    new_val
                })
                .max_by(|x, y| x.cmp(y))
            {
                total
            } else {
                0
            };
        }
        total_stackable + total_non_stackable
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

impl FromIterator<AttackDamageMod> for AttackDamageModList {
    fn from_iter<I: IntoIterator<Item = AttackDamageMod>>(iter: I) -> Self {
        let mut c = AttackDamageModList::new();

        for i in iter {
            c.add(i);
        }

        c
    }
}

impl<'a> FromIterator<&'a AttackDamageMod> for AttackDamageModList {
    fn from_iter<I: IntoIterator<Item = &'a AttackDamageMod>>(iter: I) -> Self {
        let mut c = AttackDamageModList::new();

        for i in iter {
            c.add(*i);
        }

        c
    }
}
