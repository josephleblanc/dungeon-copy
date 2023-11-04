use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::creature::Creature;
use crate::components::hitpoints::HitPoints;
use crate::resources::monster::ArmorClass;
use crate::resources::monster::AttributeBundle;
use crate::resources::monster::Monster;

#[derive(Bundle, Clone, Serialize, Deserialize)]
pub struct MonsterStats {
    pub monster: Monster,
    pub hp: HitPoints,
    pub attributes: AttributeBundle,
    pub label: Creature,
    // pub armor_class: ArmorClass,
}
