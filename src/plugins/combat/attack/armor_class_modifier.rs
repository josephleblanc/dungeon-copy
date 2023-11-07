use bevy::prelude::*;

use crate::{
    components::attributes::{Attribute, Dexterity},
    plugins::combat::bonus::{BonusSource, BonusType},
};

use super::{AttackData, AttackDataEvent};

#[derive(Copy, Clone, Debug)]
pub struct ACMod {
    pub val: isize,
    pub source: BonusSource,
    pub bonus_type: BonusType,
    pub attack_data: AttackData,
}

impl ACMod {
    pub fn add_attribute_bonus<T>(&mut self, attribute: T)
    where
        T: Attribute,
        usize: std::convert::From<T>,
    {
        self.val += attribute.bonus();
    }
}

impl From<ACMod> for usize {
    fn from(value: ACMod) -> Self {
        value.val as usize
    }
}

impl From<ACMod> for isize {
    fn from(value: ACMod) -> Self {
        value.val
    }
}

impl From<ACMod> for ACModEvent {
    fn from(value: ACMod) -> Self {
        ACModEvent(value)
    }
}

#[derive(Event, Clone, Deref, DerefMut)]
pub struct ACModEvent(ACMod);

impl From<ACModEvent> for ACMod {
    fn from(value: ACModEvent) -> Self {
        value.0
    }
}

/// The base modifer of 0 is sent to ensure that `armor_class::sum_armor_class_modifiers` has at
/// least one event so it will run.
pub fn base(
    mut attack_data_event: EventReader<AttackDataEvent>,
    mut event_writer: EventWriter<ACModEvent>,
) {
    let debug = false;
    for attack_data in attack_data_event.into_iter() {
        if debug {
            println!("debug | armor_class_modifier::base | start");
        }
        let armor_class_modifier = ACMod {
            val: 0,
            source: BonusSource::Base,
            bonus_type: BonusType::Untyped,
            attack_data: **attack_data,
        };

        event_writer.send(armor_class_modifier.into());
    }
}

/// Add the dexterity modifier to armor class, if applicable.
pub fn add_dexterity(
    mut attack_data_event: EventReader<AttackDataEvent>,
    mut event_writer: EventWriter<ACModEvent>,
    defender_query: Query<&Dexterity>,
) {
    let debug = false;
    for attack_data in attack_data_event.into_iter() {
        if debug {
            println!("debug | armor_class_modifier::add_dexterity | start");
        }
        if let Ok(dexterity) = defender_query.get(attack_data.defender) {
            let mut armor_class_modifier = ACMod {
                val: 0,
                source: BonusSource::Dexterity,
                bonus_type: BonusType::Untyped,
                attack_data: **attack_data,
            };
            armor_class_modifier.add_attribute_bonus(*dexterity);
            if debug {
                debug_add_dexterity(armor_class_modifier);
            }

            event_writer.send(armor_class_modifier.into());
        }
    }
}

fn debug_add_dexterity(armor_class_modifier: ACMod) {
    println!(
        "{:>6}|{:>28}| dexterity bonus added: {}",
        "", "", armor_class_modifier.val
    );
}

#[derive(Debug, Deref)]
pub struct ACModList(Vec<ACMod>);

impl ACModList {
    fn new() -> ACModList {
        ACModList(Vec::new())
    }

    fn add(&mut self, elem: ACMod) {
        self.0.push(elem);
    }

    /// Sum up the modifiers of stackable types, such as Dodge and Untyped.
    fn sum_stackable(&self) -> isize {
        let mut total = 0;
        for bonus_type in BonusType::stackable() {
            total += self
                .iter()
                .filter(|ac_mod| ac_mod.bonus_type == bonus_type)
                .fold(0, |acc, x| acc + x.val);
        }
        total
    }

    /// Sum up the modifiers of non-stackable types, only applying the highest modifier of each
    /// type. Examples of non-stackable types are Size, Morale, and Strength.
    fn sum_non_stackable(&self) -> isize {
        let mut total = 0;
        for bonus_type in BonusType::non_stackable() {
            if let Some(highest_modifier) = self
                .iter()
                .filter(|ac_mod| ac_mod.bonus_type == bonus_type)
                .max_by(|x, y| x.val.cmp(&y.val))
            {
                total += highest_modifier.val;
            }
        }
        total
    }

    pub fn sum_all(&self) -> isize {
        self.sum_stackable() + self.sum_non_stackable()
    }

    /// The `verified_data` method goes through the list of ACMods and compares the attack_data of
    /// each to ensure they are all from the same attack.
    pub fn verified_data(&self) -> Result<AttackData, &'static str> {
        if self.is_empty() {
            Err("Attempted to verify an empty list of ACMods. \
                ACModList must have at least one element")
        } else if self
            .iter()
            .any(|ac_mod| ac_mod.attack_data != self[0].attack_data)
        {
            Err("Mismatched data in ACModList")
        } else {
            Ok(self[0].attack_data)
        }
    }

    // pub fn verified_attacker(&self) -> Option<Entity> {
    //     if self.is_empty()
    //         || self
    //             .iter()
    //             .any(|ac_mod| ac_mod.attacker != self[0].attacker)
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
    //             .any(|ac_mod| ac_mod.defender != self[0].defender)
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

impl FromIterator<ACMod> for ACModList {
    fn from_iter<I: IntoIterator<Item = ACMod>>(iter: I) -> Self {
        let mut c = ACModList::new();

        for i in iter {
            c.add(i);
        }

        c
    }
}
