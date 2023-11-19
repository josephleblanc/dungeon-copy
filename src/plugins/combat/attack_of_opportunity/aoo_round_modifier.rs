use bevy::prelude::*;

use crate::{
    components::attributes::{Attribute, Dexterity},
    plugins::combat::bonus::BonusType,
};

use super::{AOOBonusSource, AOORoundStart};

#[derive(Copy, Clone, Debug)]
pub struct AOORoundMod {
    pub val: isize,
    pub bonus_type: BonusType,
    pub source: AOOBonusSource,
    pub attacker: Entity,
}

impl AOORoundMod {
    pub fn new(source: AOOBonusSource, attacker: Entity, bonus_type: BonusType) -> Self {
        Self {
            val: 1,
            bonus_type,
            source,
            attacker,
        }
    }

    pub fn add_attribute_bonus<T>(&mut self, attribute: T)
    where
        T: Attribute,
        usize: std::convert::From<T>,
    {
        self.val += attribute.bonus();
    }
}

impl From<AOORoundMod> for usize {
    fn from(value: AOORoundMod) -> Self {
        value.val as usize
    }
}

impl From<AOORoundMod> for isize {
    fn from(value: AOORoundMod) -> Self {
        value.val
    }
}

impl From<AOORoundMod> for AOORoundModEvent {
    fn from(value: AOORoundMod) -> Self {
        AOORoundModEvent(value)
    }
}

#[derive(Event, Clone, Deref, DerefMut)]
pub struct AOORoundModEvent(AOORoundMod);

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
/// Label struct for the combat reflexes feat.
pub struct CombatReflexes;

impl CombatReflexes {
    pub fn add_bonus(
        dex_query: Query<&Dexterity, With<CombatReflexes>>,
        mut start_event: EventReader<AOORoundStart>,
        mut mod_event: EventWriter<AOORoundModEvent>,
    ) {
        let debug = true;
        for entity in start_event.into_iter() {
            if debug {
                println!(
                    "debug | CombatReflexes::add_bonus | checking entity {:?} for combat reflexes",
                    **entity
                );
            }
            if let Ok(dexterity) = dex_query.get(**entity) {
                if debug {
                    println!(
                        "debug | CombatReflexes::add_bonus | aoos added to entity: {:?}",
                        **entity
                    );
                }
                let mut aoo_mod =
                    AOORoundMod::new(AOOBonusSource::Dexterity, **entity, BonusType::Dexterity);
                aoo_mod.add_attribute_bonus(*dexterity);

                mod_event.send(aoo_mod.into())
            }
        }
    }
}

pub fn base(
    mut start_event: EventReader<AOORoundStart>,
    mut mod_event: EventWriter<AOORoundModEvent>,
) {
    for entity in start_event.into_iter() {
        let aoo_mod = AOORoundMod::new(AOOBonusSource::Base, **entity, BonusType::Untyped);
        mod_event.send(aoo_mod.into())
    }
}
