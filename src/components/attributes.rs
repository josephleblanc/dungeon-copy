use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Bundle, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct AttributeBundle {
    strength: Strength,
    dexterity: Dexterity,
    // constitution: Constitution,
    // intelligence: Intelligence,
    // wisdom: Wisdom,
    // charisma: Charisma,
}

#[derive(Component, Clone, Copy, Debug, Deref, DerefMut, Serialize, Deserialize)]
pub struct Strength(usize);
impl From<Strength> for usize {
    fn from(value: Strength) -> Self {
        *value
    }
}
impl Attribute for Strength {}

#[derive(Component, Clone, Copy, Debug, Deref, DerefMut, Serialize, Deserialize)]
pub struct Dexterity(usize);
impl From<Dexterity> for usize {
    fn from(value: Dexterity) -> Self {
        *value
    }
}
impl Attribute for Dexterity {}

#[derive(Component, Clone, Copy, Debug, Deref, DerefMut, Serialize, Deserialize)]
pub struct Constitution(usize);
impl From<Constitution> for usize {
    fn from(value: Constitution) -> Self {
        *value
    }
}
impl Attribute for Constitution {}

#[derive(Component, Clone, Copy, Debug, Deref, DerefMut, Serialize, Deserialize)]
pub struct Intelligence(usize);
impl From<Intelligence> for usize {
    fn from(value: Intelligence) -> Self {
        *value
    }
}
impl Attribute for Intelligence {}

#[derive(Component, Clone, Copy, Debug, Deref, DerefMut, Serialize, Deserialize)]
pub struct Wisdom(usize);
impl From<Wisdom> for usize {
    fn from(value: Wisdom) -> Self {
        *value
    }
}
impl Attribute for Wisdom {}

#[derive(Component, Clone, Copy, Debug, Deref, DerefMut, Serialize, Deserialize)]
pub struct Charisma(usize);
impl From<Charisma> for usize {
    fn from(value: Charisma) -> Self {
        *value
    }
}
impl Attribute for Charisma {}

pub trait Attribute: Clone + Copy
where
    usize: std::convert::From<Self>,
{
    fn bonus(&self) -> isize {
        let i_num: isize = usize::from(*self) as isize;
        i_num / 2 - 5
    }
}

// Examples of how to use the above trait and structs
pub fn test_attribute<T>(attribute: T)
where
    T: Attribute,
    usize: std::convert::From<T>,
{
    let bonus = attribute.bonus();
}

pub fn test_strength(str: Strength) {
    let bonus = str.bonus();
}
