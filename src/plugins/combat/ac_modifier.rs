use bevy::prelude::*;

use crate::components::attributes::Attribute;
use crate::components::attributes::Dexterity;
use crate::plugins::combat::armor_class::ACBonusEvent;
use crate::plugins::combat::bonus::BonusSource;
use crate::plugins::combat::bonus::BonusType;
use crate::plugins::player::control::ActionPriority;

#[derive(Copy, Clone, Debug)]
pub struct ACModifier {
    pub val: isize,
    pub source: BonusSource,
    pub bonus_type: BonusType,
    pub attacker: Entity,
    pub defender: Entity,
}

impl ACModifier {
    pub fn add_attribute_bonus<T>(&mut self, attribute: T)
    where
        T: Attribute,
        usize: std::convert::From<T>,
    {
        self.val += attribute.bonus();
    }
}

impl From<ACModifier> for usize {
    fn from(value: ACModifier) -> Self {
        value.val as usize
    }
}

impl From<ACModifier> for isize {
    fn from(value: ACModifier) -> Self {
        value.val
    }
}

impl From<ACModifier> for ACModifierEvent {
    fn from(value: ACModifier) -> Self {
        ACModifierEvent(value)
    }
}

#[derive(Event, Copy, Clone, Deref, DerefMut)]
pub struct ACModifierEvent(ACModifier);

impl From<ACModifierEvent> for ACModifier {
    fn from(value: ACModifierEvent) -> Self {
        value.0
    }
}

pub fn add_dexterity(
    mut ac_event: EventReader<ACBonusEvent>,
    mut event_writer: EventWriter<ACModifierEvent>,
    query_attacker: Query<&Dexterity, With<ActionPriority>>,
) {
    for ac in ac_event.iter() {
        if let Ok(dexterity) = query_attacker.get_single() {
            let mut ac_modifier = ACModifier {
                val: 0,
                source: BonusSource::Dexterity,
                bonus_type: BonusType::Untyped,
                attacker: ac.attacker,
                defender: ac.defender,
            };
            ac_modifier.add_attribute_bonus(*dexterity);

            event_writer.send(ac_modifier.into());
        }
    }
}

#[derive(Debug, Deref)]
pub struct ACModifierList(Vec<ACModifier>);

impl ACModifierList {
    fn new() -> ACModifierList {
        ACModifierList(Vec::new())
    }

    fn add(&mut self, elem: ACModifier) {
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
        for bonus_type in BonusType::stackable() {
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
        if self
            .iter()
            .any(|ac_mod| ac_mod.attacker != self[0].attacker)
        {
            None
        } else {
            Some(self[0].attacker)
        }
    }

    pub fn verified_defender(&self) -> Option<Entity> {
        if self
            .iter()
            .any(|ac_mod| ac_mod.defender != self[0].attacker)
        {
            None
        } else {
            Some(self[0].defender)
        }
    }
}

impl FromIterator<ACModifier> for ACModifierList {
    fn from_iter<I: IntoIterator<Item = ACModifier>>(iter: I) -> Self {
        let mut c = ACModifierList::new();

        for i in iter {
            c.add(i);
        }

        c
    }
}
