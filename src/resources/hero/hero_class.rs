use bevy_inspector_egui::InspectorOptions;
use serde::{Deserialize, Serialize};
use std::slice::Iter;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, InspectorOptions)]
pub enum HeroClass {
    Fighter,
    Wizard,
}

impl HeroClass {
    pub fn iterator() -> Iter<'static, HeroClass> {
        [HeroClass::Fighter, HeroClass::Wizard].iter()
    }
}
