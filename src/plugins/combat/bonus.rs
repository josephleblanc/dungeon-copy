#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// The type of bonus of a modifier. Some stack, some do not. If the buffs do
/// not stack, then the greatest is selected.
pub enum BonusType {
    Morale,
    Size,
    Dodge,
    Strength,
    Untyped, // more here
}

impl BonusType {
    pub fn stackable() -> [Self; 2] {
        [Self::Dodge, Self::Untyped]
    }

    pub fn non_stackable() -> [Self; 2] {
        [Self::Morale, Self::Strength]
    }
}

#[derive(Copy, Clone, Debug)]
pub enum BonusSource {
    Base,
    Strength,
    Dexterity,
    WeaponFocus,
    BaseAttackBonus, // more here
}
