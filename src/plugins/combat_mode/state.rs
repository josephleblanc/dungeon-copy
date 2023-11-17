use bevy::prelude::*;
use std::slice::Iter;

#[derive(Component, Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum CombatMode {
    InCombat,
    #[default]
    OutOfCombat,
}

impl CombatMode {
    pub fn iterator() -> Iter<'static, CombatMode> {
        [CombatMode::InCombat, CombatMode::OutOfCombat].iter()
    }
}
