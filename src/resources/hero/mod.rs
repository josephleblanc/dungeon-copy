use serde::{Deserialize, Serialize};

pub mod gender;
pub mod hero_class;

use hero_class::HeroClass;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hero {
    pub hero_class: HeroClass,
}
