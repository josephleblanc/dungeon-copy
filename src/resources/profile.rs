use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::resources::game_mode::GameMode;

#[derive(Resource, Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    experience: usize,
    game_mode: GameMode,
}

impl Profile {
    pub fn new() -> Self {
        Profile {
            experience: 0,
            game_mode: GameMode::ClassicMode,
        }
    }

    pub fn set_game_mode(&mut self, game_mode: GameMode) {
        self.game_mode = game_mode;
    }
}
