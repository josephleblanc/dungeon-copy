use bevy::prelude::*;

use crate::resources::hero::gender::Gender;
use crate::resources::hero::hero_class::HeroClass;

#[derive(Clone)]
pub struct HeroesMaterials {
    pub male_fighter: Handle<Image>,
    pub male_wizard: Handle<Image>,
    pub female_fighter: Handle<Image>,
    pub female_wizard: Handle<Image>,
}

impl HeroesMaterials {
    pub fn get_texture(&self, hero_class: HeroClass, gender: Gender) -> Handle<Image> {
        match hero_class {
            HeroClass::Fighter => match gender {
                Gender::Male => self.male_fighter.clone(),
                Gender::Female => self.female_fighter.clone(),
            },
            HeroClass::Wizard => match gender {
                Gender::Male => self.male_wizard.clone(),
                Gender::Female => self.female_wizard.clone(),
            },
        }
    }
}
