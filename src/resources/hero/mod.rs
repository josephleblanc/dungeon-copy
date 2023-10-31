use serde::{Deserialize, Serialize};

pub mod gender;
pub mod hero_class;
pub mod stats;

use hero_class::HeroClass;

use crate::components::{attack_bonus::BaseAttackBonus, attributes::AttributeBundle};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hero {
    pub hero_class: HeroClass,
    pub stats: stats::Stats,
    pub attributes: AttributeBundle,
    pub base_attack_bonus: BaseAttackBonus,
}
