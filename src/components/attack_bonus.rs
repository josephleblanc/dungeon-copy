use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::attributes::Attribute;

#[derive(Component, Clone, Copy, Debug, Deref, DerefMut, Serialize, Deserialize)]
pub struct AttackBonus(isize);

#[derive(Component, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct BaseAttackBonus(isize);

impl AttackBonus {
    pub fn new() -> Self {
        AttackBonus(isize::default())
    }

    pub fn new_from_bab(bab: AttackBonus) -> Self {
        AttackBonus(*bab)
    }

    pub fn add_attribute_bonus<T>(&mut self, attribute: T)
    where
        T: Attribute,
        usize: std::convert::From<T>,
    {
        **self += attribute.bonus()
    }
}
