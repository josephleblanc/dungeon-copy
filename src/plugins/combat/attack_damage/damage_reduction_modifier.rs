use bevy::prelude::*;

use crate::{
    plugins::combat::{bonus::BonusType, damage, AttackData, AttackDataEvent},
    resources::equipment::weapon,
};

use super::damage_reduction::{DRSource, DamageReduction};

#[derive(Debug, Copy, Clone, Deref, DerefMut)]
pub struct DRMod {
    pub attack_data: AttackData,
    #[deref]
    pub val: DamageReduction,
}

#[derive(Event, Debug, Copy, Clone, Deref, DerefMut)]
pub struct DRModEvent(DRMod);

#[derive(Deref, Clone, Debug)]
pub struct DRModList(pub Vec<DRMod>);

#[derive(Debug, Component, Copy, Clone, Deref)]
/// Damage reduction due to a barbarian class feature. This includes archetypes as well as any
/// other bonus listed as being due to the barbarian class feature.
// TODO: Organize: Consider moving this struct somewhere with all the other class feature structs
// used for events like this.
pub struct BarbarianDR(DamageReduction);

impl BarbarianDR {
    pub fn new(val: DamageReduction) -> Self {
        Self(val)
    }
}

/// Apply the Damage Reduction from a barbarian class feature.
pub fn barbarian(
    mut attack_data_events: EventReader<AttackDataEvent>,
    mut dr_mod_writer: EventWriter<DRModEvent>,
    defender: Query<&BarbarianDR>,
) {
    for attack_data in attack_data_events.iter() {
        println!("debug | damage_reduction_modifier::barbarian | start");
        if let Ok(defender_dr) = defender.get(attack_data.defender) {
            println!(
                "debug | damage_reduction_modifier::barbarian | sending DRModEvent: {:?}",
                defender_dr
            );
            dr_mod_writer.send(DRModEvent(DRMod {
                attack_data: **attack_data,
                val: **defender_dr,
            }))
        }
    }
}

impl DRModList {
    pub fn new() -> DRModList {
        DRModList(Vec::new())
    }

    fn add(&mut self, elem: DRMod) {
        self.0.push(elem);
    }

    pub fn sum(&self) -> usize {
        self.iter().fold(0, |acc, x| acc + x.dr_val)
    }

    pub fn sum_type(&self, dmg_type: weapon::DamageType) -> usize {
        self.iter()
            .filter(|dmg_mod| dmg_mod.damage_types.contains(&(dmg_type, true)))
            .fold(0, |acc, x| acc + x.dr_val)
    }

    pub fn highest_reduces_single(&self, other: weapon::DamageType) -> Option<DamageReduction> {
        if self.is_empty() {
            Some(
                **self
                    .iter()
                    .filter(|dr_mod| dr_mod.damage_types.does_reduce(other))
                    .max_by(|x, y| x.dr_val.cmp(&y.dr_val))
                    .unwrap(),
            )
        } else {
            None
        }
    }

    pub fn verified_data(&self) -> Result<AttackData, &'static str> {
        if self.is_empty() {
            Err("Attempted to verify an empty list of DRMods. \
                DRModList must have at least one element")
        } else if self
            .iter()
            .any(|dr_mod| dr_mod.attack_data != self[0].attack_data)
        {
            Err("Mismatched data in AttackDamageModList")
        } else {
            Ok(self[0].attack_data)
        }
    }

    // pub fn highest_reduces(&self, other: weapon::DamageType) -> Option<Self> {
    //     if self.is_empty() {
    //         return None;
    //     }
    //     match self
    //         .highest_reduces_single(other)
    //         .unwrap()
    //         .dr_val
    //         .cmp(&self.sum_stackable_type(other).unwrap().sum())
    //     {
    //         std::cmp::Ordering::Less => ,
    //         std::cmp::Ordering::Equal => todo!(),
    //         std::cmp::Ordering::Greater => todo!(),
    //     }
    // }

    pub fn sum_stackable_type(&self, dmg_type: weapon::DamageType) -> Option<Self> {
        // println!(">>> sum_stackable_types: self is {:?}", self);
        if self.is_empty() {
            println!(">>> sum_stackable_types: self is empty");
            return None;
        } else if self.len() == 1 {
            return Some(self.clone());
        }
        self.iter()
            .inspect(|item| println!(">>> debug sum_stackable_type - inspecting item: {:?}", item))
            .filter(|dr_mod| dr_mod.damage_types.does_reduce(dmg_type))
            .map(|outer| {
                let stacks_list: DRModList = self
                    .iter()
                    .filter(|inner| inner.damage_types.does_reduce(dmg_type))
                    .filter(|inner| {
                        inner
                            .reduction_source
                            .can_stack_with(outer.reduction_source)
                    })
                    .collect();
                stacks_list
            })
            .max_by(|x, y| x.sum().cmp(&y.sum()))
    }
}

impl FromIterator<DRMod> for DRModList {
    fn from_iter<I: IntoIterator<Item = DRMod>>(iter: I) -> Self {
        let mut c = DRModList::new();

        for i in iter {
            c.add(i);
        }

        c
    }
}

impl<'a> FromIterator<&'a DRMod> for DRModList {
    fn from_iter<I: IntoIterator<Item = &'a DRMod>>(iter: I) -> Self {
        let mut c = DRModList::new();

        for i in iter {
            c.add(*i);
        }

        c
    }
}
