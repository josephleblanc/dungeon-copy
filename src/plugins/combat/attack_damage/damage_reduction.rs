#![allow(dead_code)]
use bevy::prelude::*;

use crate::{
    components::alignment,
    plugins::combat::AttackData,
    resources::equipment::{
        weapon::{self, WeaponDamageTypes},
        Enhancement,
    },
};

use super::damage_reduction_modifier::{DRModEvent, DRModList};

#[derive(Debug, Copy, Clone)]
pub enum DRSource {
    Untyped,
    Barbarian,
    Stalwart,
    ImprovedStalwart,
}

impl DRSource {
    pub fn can_stack_with(self, other: Self) -> bool {
        use DRSource::*;
        matches!(
            (self, other),
            (Stalwart, Barbarian)
                | (Barbarian, Stalwart)
                | (ImprovedStalwart, Barbarian)
                | (Barbarian, ImprovedStalwart)
                | (ImprovedStalwart, Stalwart)
                | (Stalwart, ImprovedStalwart)
        )
    }
}

#[derive(Debug, Copy, Clone)]
pub enum DROvercome {
    ColdIron,
    Silver,
    Adamantine,
    Alignment(alignment::Alignment),
}

impl DROvercome {
    pub fn or_enhancement(self) -> Enhancement {
        match self {
            Self::ColdIron => Enhancement::Plus3,
            Self::Silver => Enhancement::Plus3,
            Self::Adamantine => Enhancement::Plus4,
            Self::Alignment(_) => Enhancement::Plus5,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct DamageReduction {
    pub dr_val: usize,
    pub reduction_source: DRSource,
    pub damage_types: DRTypes,
    pub overcome: Option<DROvercome>,
}

#[derive(Debug, Copy, Clone, Deref)]
pub struct DRTypes([(weapon::DamageType, bool); 3]);

impl DRTypes {
    pub fn all() -> Self {
        Self([
            (weapon::DamageType::Slashing, true),
            (weapon::DamageType::Piercing, true),
            (weapon::DamageType::Blunt, true),
        ])
    }

    pub fn from_single(dmg_type: weapon::DamageType) -> Self {
        Self([
            (
                weapon::DamageType::Slashing,
                weapon::DamageType::Slashing == dmg_type,
            ),
            (
                weapon::DamageType::Piercing,
                weapon::DamageType::Piercing == dmg_type,
            ),
            (
                weapon::DamageType::Blunt,
                weapon::DamageType::Blunt == dmg_type,
            ),
        ])
    }

    pub fn from_weapon_damage_types(weapon_damage_types: WeaponDamageTypes) -> Self {
        Self(*weapon_damage_types)
    }
}

impl DRTypes {
    pub fn does_reduce(self, other: weapon::DamageType) -> bool {
        self.contains(&(other, true))
    }

    pub fn reduces_all(self) -> bool {
        !self.iter().any(|(_dmg_type, is_reduced)| !is_reduced)
    }
}

#[derive(Debug, Clone, Event, Deref)]
pub struct DRTotalEvent {
    #[deref]
    pub dr_total: DRTotal,
    pub attack_data: AttackData,
}

#[derive(Debug, Clone)]
pub struct DRTotal {
    piercing: Option<DRModList>,
    slashing: Option<DRModList>,
    blunt: Option<DRModList>,
    piercing_val: usize,
    slashing_val: usize,
    blunt_val: usize,
}

impl DRTotal {
    pub fn new_from(
        piercing: Option<DRModList>,
        slashing: Option<DRModList>,
        blunt: Option<DRModList>,
    ) -> Self {
        let mut piercing_val = 0;
        let mut piercing_wrapped: Option<DRModList> = None;
        if let Some(piercing) = piercing {
            piercing_val = piercing.sum();
            piercing_wrapped = Some(piercing);
        }
        let mut slashing_val = 0;
        let mut slashing_wrapped: Option<DRModList> = None;
        if let Some(slashing) = slashing {
            slashing_val = slashing.sum();
            slashing_wrapped = Some(slashing);
        }
        let mut blunt_val = 0;
        let mut blunt_wrapped: Option<DRModList> = None;
        if let Some(blunt) = blunt {
            blunt_val = blunt.sum();
            blunt_wrapped = Some(blunt);
        }
        Self {
            piercing_val,
            slashing_val,
            blunt_val,
            piercing: piercing_wrapped,
            slashing: slashing_wrapped,
            blunt: blunt_wrapped,
        }
    }

    pub fn min_vs_weapon(&self, weapon_damage_types: WeaponDamageTypes) -> Option<usize> {
        weapon_damage_types
            .iter()
            .filter(|dmg| dmg.1)
            .map(|dmg| match dmg.0 {
                weapon::DamageType::Blunt => self.blunt_val,
                weapon::DamageType::Slashing => self.slashing_val,
                weapon::DamageType::Piercing => self.piercing_val,
            })
            .min()
    }
}

pub fn sum_damage_reduction(
    mut dr_events: EventReader<DRModEvent>,
    mut dr_total_writer: EventWriter<DRTotalEvent>,
) {
    let debug = true;
    if !dr_events.is_empty() {
        let list_in: DRModList = dr_events.into_iter().map(|dr_event| **dr_event).collect();

        // debug
        debug_sum_damage_reduction_inner(debug, &list_in);
        //

        let piercing: Option<DRModList> = list_in.sum_stackable_type(weapon::DamageType::Piercing);
        let slashing: Option<DRModList> = list_in.sum_stackable_type(weapon::DamageType::Slashing);
        let blunt: Option<DRModList> = list_in.sum_stackable_type(weapon::DamageType::Blunt);

        let attack_data = list_in.verified_data().unwrap();

        let dr_total = DRTotal::new_from(piercing, slashing, blunt);
        dr_total_writer.send(DRTotalEvent {
            dr_total,
            attack_data,
        });
    }
}

fn debug_sum_damage_reduction_inner(debug: bool, list_in: &DRModList) {
    if debug {
        for dr_item in list_in.iter() {
            println!(
                "debug | damage_reduction::sum_damage_reduction | dr_item: {:?}",
                dr_item
            );
        }
    }
}

pub fn debug_sum_damage_reduction(mut dr_total_reader: EventReader<DRTotalEvent>) {
    for event in dr_total_reader.iter() {
        println!("debug | debug_sum_damage_reduction | event = {:?}", event);
    }
}
