use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

use crate::config::DATA_FILE;
use crate::resources::hero::hero_class::HeroClass;
use crate::resources::hero::Hero;

#[derive(Resource)]
pub struct PauseSceneData {
    pub(crate) user_interface_root: Entity,
}

#[derive(Resource, Serialize, Deserialize, Debug, Clone)]
pub struct GameData {
    heroes: [Hero; 2],
    // weapons: [Weapon; 11],
    // skills: [Skill; 4],
    // player_list_effects_information: [Effect; 8],
    // monsters: [Monster; 10],
}

impl GameData {
    pub fn new() -> Self {
        let data = match File::open(DATA_FILE) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                serde_json::from_str(&contents).expect("JSON was not well-formatted")
            }
            Err(err) => panic!("Can't find language file: {}", err),
        };
        data
    }

    /// Searcher GameData for the initial/base Hero info.
    pub fn get_hero(&self, hero_class: HeroClass) -> Hero {
        self.heroes
            .iter()
            .find(|hero| hero.hero_class == hero_class)
            .unwrap()
            .clone()
    }
}
