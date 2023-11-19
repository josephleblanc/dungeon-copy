#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// The type of bonus of a modifier. Some stack, some do not. If the buffs do
/// not stack, then the greatest is selected.
pub enum BonusType {
    Morale,
    Size,
    Dodge,
    Strength,
    Dexterity,
    Untyped, // more here
}

impl BonusType {
    pub fn stackable() -> [Self; 2] {
        [Self::Dodge, Self::Untyped]
    }

    pub fn non_stackable() -> [Self; 4] {
        [Self::Morale, Self::Strength, Self::Dexterity, Self::Size]
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
