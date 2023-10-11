use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::resources::game_mode::GameMode;
use crate::resources::hero::gender::Gender;
use crate::resources::hero::hero_class::HeroClass;
use crate::scenes::hero_select_scene::ButtonComponent;

#[derive(Resource, Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub experience: usize,
    pub game_mode: GameMode,
    pub hero_class: HeroClass,
    pub gender: Gender,
}

impl Profile {
    pub fn new() -> Self {
        Profile {
            experience: 0,
            game_mode: GameMode::ClassicMode,
            hero_class: HeroClass::Fighter,
            gender: Gender::Male,
        }
    }

    pub fn set_game_mode(&mut self, game_mode: GameMode) {
        self.game_mode = game_mode;
    }

    pub fn set_hero(&mut self, button: ButtonComponent) {
        match button {
            ButtonComponent::MaleFighter => {
                self.hero_class = HeroClass::Fighter;
                self.gender = Gender::Male;
            }
            ButtonComponent::FemaleFighter => {
                self.hero_class = HeroClass::Fighter;
                self.gender = Gender::Female;
            }
            ButtonComponent::MaleWizard => {
                self.hero_class = HeroClass::Wizard;
                self.gender = Gender::Male;
            }
            ButtonComponent::FemaleWizard => {
                self.hero_class = HeroClass::Wizard;
                self.gender = Gender::Female;
            }
        }
    }
}
