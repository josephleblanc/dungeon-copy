#![allow(dead_code)]
use crate::{
    plugins::combat::{damage::DamageType, AttackData},
    resources::equipment::weapon::{Weapon, WeaponDamageTypes},
};
use bevy::prelude::*;

use super::damage_modifier::{AttackDamageModEvent, AttackDamageModList, OnCrit};

#[derive(Debug, Copy, Clone)]
pub struct AttackDamageSum {
    pub attack_data: AttackData,
    pub multiply_on_crit: isize,
    pub no_multiply_on_crit: isize,
    pub only_on_crit: isize,
    pub weapon_damage: isize,
    pub weapon_damage_types: WeaponDamageTypes,
}

#[derive(Debug, Event, Copy, Clone, Deref)]
pub struct AttackDamageSumEvent(AttackDamageSum);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DamageBonusSource {
    Strength,
    Weapon,
    Base,
}

impl DamageBonusSource {
    pub fn stackable() -> [Self; 1] {
        [Self::Weapon]
    }
    pub fn non_stackable() -> [Self; 1] {
        [Self::Strength]
    }
}

pub fn sum_damage_mod(
    mut damage_mod_reader: EventReader<AttackDamageModEvent>,
    mut damage_sum_event: EventWriter<AttackDamageSumEvent>,
    weapon_query: Query<&Weapon>,
) {
    let dmg_mod_list = damage_mod_reader
        .into_iter()
        .map(|dmg_event| **dmg_event)
        .collect::<AttackDamageModList>();
    if !dmg_mod_list.is_empty() {
        let attack_data = dmg_mod_list.verified_data().unwrap();
        let weapon_damage_types = weapon_query
            .get(attack_data.weapon_slot.entity)
            .unwrap()
            .weapon_damage_types;
        let multiply_on_crit = dmg_mod_list
            .iter()
            .filter(|dmg_mod| dmg_mod.damage_type != DamageType::Weapon)
            .filter(|dmg_mod| dmg_mod.on_crit == OnCrit::CanMultiply)
            .collect::<AttackDamageModList>()
            .sum_all();
        println!("multipy_on_crit: {}", multiply_on_crit);
        let no_multiply_on_crit = dmg_mod_list
            .iter()
            .filter(|dmg_mod| dmg_mod.damage_type != DamageType::Weapon)
            .filter(|dmg_mod| dmg_mod.on_crit == OnCrit::CannotMultiply)
            .collect::<AttackDamageModList>()
            .sum_all();
        println!("no_multiply_on_crit: {}", no_multiply_on_crit);
        let only_on_crit = dmg_mod_list
            .iter()
            .filter(|dmg_mod| dmg_mod.damage_type != DamageType::Weapon)
            .filter(|dmg_mod| dmg_mod.on_crit == OnCrit::OnlyOn)
            .collect::<AttackDamageModList>()
            .sum_all();
        println!("only_on_crit: {}", only_on_crit);
        let weapon_damage = dmg_mod_list
            .iter()
            .filter(|dmg_mod| dmg_mod.damage_type == DamageType::Weapon)
            .collect::<AttackDamageModList>()
            .sum_all();
        println!("weapon_damage: {}", weapon_damage);
        damage_sum_event.send(AttackDamageSumEvent(AttackDamageSum {
            attack_data,
            multiply_on_crit,
            no_multiply_on_crit,
            only_on_crit,
            weapon_damage,
            weapon_damage_types,
        }));
    }
}
