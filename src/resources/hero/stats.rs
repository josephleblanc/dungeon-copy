use bevy_inspector_egui::InspectorOptions;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, InspectorOptions)]
pub struct Stats {
    pub health_points: f32,
    pub speed: f32,
    // pub strength: f32,
    // pub intelligence: f32,
    // pub Crit_chance: f32,
    // pub dodge_chance: f32,
    // pub restore_chance: f32,
}
