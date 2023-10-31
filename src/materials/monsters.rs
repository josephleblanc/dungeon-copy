use bevy::prelude::*;

use crate::resources::monster::Monster;

#[derive(Clone)]
pub struct MonstersMaterials {
    pub training_dummy: Handle<Image>,
}

// TODO: Consider changing get_texture to take a `MonsterClass` instead of `Monster`
// if I plan on having multiple monsters that use the same sprite.
impl MonstersMaterials {
    pub fn get_texture(&self, monster: Monster) -> Handle<Image> {
        match monster {
            Monster::TrainingDummy => self.training_dummy.clone(),
        }
    }
}
