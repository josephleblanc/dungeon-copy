use bevy::prelude::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// The type of bonus of a modifier. Some stack, some do not. If the buffs do
/// not stack, then the greatest is selected.
pub enum BonusType {
    Morale,
    Size,
    Dodge,
    Untyped, // more here
}

impl BonusType {
    pub fn stackable() -> [BonusType; 2] {
        [BonusType::Dodge, BonusType::Untyped]
    }

    pub fn non_stackable() -> [BonusType; 2] {
        [BonusType::Morale, BonusType::Untyped]
    }
}
