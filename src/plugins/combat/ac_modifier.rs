use bevy::prelude::*;

use crate::components::attributes::Attribute;
use crate::components::attributes::Dexterity;
use crate::plugins::combat::armor_class::ACBonusEvent;
use crate::plugins::combat::bonus::BonusSource;
use crate::plugins::combat::bonus::BonusType;
use crate::resources::equipment::weapon::Weapon;

#[derive(Copy, Clone, Debug)]
pub struct ACMod {
    pub val: isize,
    pub source: BonusSource,
    pub bonus_type: BonusType,
    pub attacker: Entity,
    pub defender: Entity,
    pub attacker_weapon: Entity,
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

pub fn add_dexterity(
    mut ac_event: EventReader<ACBonusEvent>,
    mut event_writer: EventWriter<ACModEvent>,
    defender_query: Query<&Dexterity>,
) {
    let debug = true;
    // TODO: This could be .into_iter().next() to avoid the clone. Mess around with it.
    for ac in ac_event.into_iter() {
        if debug {
            println!("debug | ac_modifier::add_dexterity | start");
        }
        if let Ok(dexterity) = defender_query.get(ac.defender) {
            let mut ac_modifier = ACMod {
                val: 0,
                source: BonusSource::Dexterity,
                bonus_type: BonusType::Untyped,
                attacker: ac.attacker,
                defender: ac.defender,
                attacker_weapon: ac.attacker_weapon.clone(),
            };
            ac_modifier.add_attribute_bonus(*dexterity);
            if debug {
                debug_add_dexterity(ac_modifier.clone());
            }

            event_writer.send(ac_modifier.into());
        }
    }
}

fn debug_add_dexterity(ac_modifier: ACMod) {
    println!(
        "{:>6}|{:>28}| dexterity bonus added: {}",
        "", "", ac_modifier.val
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

    pub fn verified_attacker(&self) -> Option<Entity> {
        if self.is_empty()
            || self
                .iter()
                .any(|ac_mod| ac_mod.attacker != self[0].attacker)
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
                .any(|ac_mod| ac_mod.defender != self[0].defender)
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
            Some(self[0].attacker_weapon)
        }
    }
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
