use bevy::prelude::*;

use crate::components::attack_bonus::BaseAttackBonus;

#[derive(Clone)]
pub struct Attack {
    pub base_attack_bonus: BaseAttackBonus,
}

#[derive(Clone, Event)]
pub struct AttackRollEvent {
    attack: Attack,
}
