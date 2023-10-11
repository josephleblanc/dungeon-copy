use bevy::prelude::*;

use crate::materials::heroes::HeroesMaterials;

#[derive(Resource)]
pub struct InGameMaterials {
    pub heroes_materials: HeroesMaterials,
}
