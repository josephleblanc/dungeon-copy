use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;

use crate::components::attributes::AttributeBundle;
use crate::resources::game_data::GameData;
use crate::resources::hero::hero_class::HeroClass;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player_component: PlayerComponent,
    pub attributes: AttributeBundle,
}

#[derive(Component, InspectorOptions)]
pub struct PlayerComponent {
    pub class: HeroClass,
    // pub current_health_points: f32,
    // pub max_health_points: f32,
    pub speed: f32,
    // pub strength: f32,
    // pub intelligence: f32,
    // pub Crit_chance: f32,
    // pub dodge_chance: f32,
    // pub restore_chance: f32,
    // pub damage_percent_bonus: f32,
    // pub power: Power,
    // pub base_stats: Stats,
}

impl PlayerComponent {
    pub fn new(hero_class: HeroClass, game_data: GameData) -> Self {
        let hero = game_data.get_hero(hero_class.clone());
        let base_stats = hero.stats;

        PlayerComponent {
            class: hero_class,
            // current_health_points: base_stats.health_points,
            // max_health_points: base_stats.health_points,
            speed: base_stats.speed,
            // strength: base_stats.strength,
            // intelligence: base_stats.intelligence,
            // Crit_chance: base_stats.Crit_chance,
            // dodge_chance: base_stats.dodge_chance,
            // restore_chance: base_stats.restore_chance,
            // power: hero.power,
            // damage_percent_bonus: 0.0,
            // base_stats: base_stats.clone(),
        }
    }
}
