use bevy::prelude::*;

use crate::plugins::combat::bonus::BonusSource;
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
#[derive(Copy, Clone, Debug)]
pub struct AttackModifier {
    pub val: isize,
    pub source: BonusSource,
    pub bonus_type: BonusType,
    pub attacker: Entity,
    pub defender: Entity,
}

impl AttackModifier {
    pub fn add_attribute_bonus<T>(&mut self, attribute: T)
    where
        T: Attribute,
        usize: std::convert::From<T>,
    {
        self.val += attribute.bonus();
    }
}

impl From<AttackModifier> for usize {
    fn from(value: AttackModifier) -> Self {
        value.val as usize
    }
}

impl From<AttackModifier> for isize {
    fn from(value: AttackModifier) -> Self {
        value.val
    }
}

impl From<AttackModifier> for AttackModifierEvent {
    fn from(value: AttackModifier) -> Self {
        AttackModifierEvent(value)
    }
}

#[derive(Event, Copy, Clone, Deref, DerefMut)]
pub struct AttackModifierEvent(AttackModifier);

impl From<AttackModifierEvent> for AttackModifier {
    fn from(value: AttackModifierEvent) -> Self {
        value.0
    }
}

pub fn add_strength(
    mut attack_roll_event: EventReader<AttackBonusEvent>,
    mut event_writer: EventWriter<AttackModifierEvent>,
    query_attacker: Query<&Strength, With<ActionPriority>>,
) {
    for attack_roll in attack_roll_event.iter() {
        if let Ok(strength) = query_attacker.get_single() {
            let mut attack_modifier = AttackModifier {
                val: 0,
                source: BonusSource::Strength,
                bonus_type: BonusType::Untyped,
                attacker: attack_roll.attacker,
                defender: attack_roll.defender,
            };
            attack_modifier.add_attribute_bonus(*strength);

            event_writer.send(attack_modifier.into());
        }
    }
}

pub fn add_weapon_focus(
    mut attack_roll_event: EventReader<AttackBonusEvent>,
    mut event_writer: EventWriter<AttackModifierEvent>,
    query_attacker: Query<&WeaponFocus, With<ActionPriority>>,
) {
    for attack_roll in attack_roll_event.iter() {
        if let Ok(weapon_focus) = query_attacker.get_single() {
            let attack_modifier =
                weapon_focus.to_atk_mod(attack_roll.attacker, attack_roll.defender);

            event_writer.send(attack_modifier.into());
        }
    }
}

#[derive(Debug, Deref)]
pub struct AttackModifierList(Vec<AttackModifier>);

impl AttackModifierList {
    fn new() -> AttackModifierList {
        AttackModifierList(Vec::new())
    }

    fn add(&mut self, elem: AttackModifier) {
        self.0.push(elem);
    }

    fn sum_stackable(&self) -> isize {
        let mut total = 0;
        for bonus_type in BonusType::stackable() {
            total += self
                .iter()
                .filter(|atk_mod| atk_mod.bonus_type == bonus_type)
                .fold(0, |acc, x| acc + x.val);
        }
        total
    }

    fn sum_non_stackable(&self) -> isize {
        let mut total = 0;
        for bonus_type in BonusType::stackable() {
            if let Some(highest_modifier) = self
                .iter()
                .filter(|atk_mod| atk_mod.bonus_type == bonus_type)
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
            .any(|atk_mod| atk_mod.attacker != self[0].attacker)
        {
            None
        } else {
            Some(self[0].attacker)
        }
    }

    pub fn verified_defender(&self) -> Option<Entity> {
        if self
            .iter()
            .any(|atk_mod| atk_mod.defender != self[0].attacker)
        {
            None
        } else {
            Some(self[0].defender)
        }
    }
}

impl FromIterator<AttackModifier> for AttackModifierList {
    fn from_iter<I: IntoIterator<Item = AttackModifier>>(iter: I) -> Self {
        let mut c = AttackModifierList::new();

        for i in iter {
            c.add(i);
        }

        c
    }
}
