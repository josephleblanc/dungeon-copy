use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

use crate::components::armor_class::ArmorClass;
use crate::components::attributes::AttributeBundle;
use crate::resources::monster::monster_stats::MonsterStats;

pub mod monster_spawn_controller;
pub mod monster_stats;

#[derive(Resource, Deref, DerefMut)]
pub struct MonsterLibrary(HashMap<Monster, MonsterStats>);

#[derive(Component, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Monster {
    TrainingDummy,
}

pub const MONSTER_DIR: &str = "assets/monsters/";
pub const MONSTER_STATS: [&str; 1] = ["training_dummy.ron"];

impl MonsterLibrary {
    pub fn new() -> Self {
        let mut monster_library: HashMap<Monster, MonsterStats> = HashMap::new();
        for file_name in MONSTER_STATS {
            let file_path = format!("{}{}", MONSTER_DIR, file_name);
            let monster_stats: MonsterStats = match File::open(file_path) {
                Ok(file) => {
                    let reader = BufReader::new(file);
                    ron::de::from_reader(reader).unwrap()
                }
                Err(err) => panic!("Can't find language file: {}", err),
            };
            monster_library.insert(monster_stats.monster, monster_stats);
        }

        MonsterLibrary(monster_library)
    }
}
