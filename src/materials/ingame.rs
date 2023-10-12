use bevy::prelude::*;

use crate::materials::dungeon::DungeonMaterials;
use crate::materials::heroes::HeroesMaterials;

#[derive(Resource)]
pub struct InGameMaterials {
    pub heroes_materials: HeroesMaterials,
    pub dungeon_materials: DungeonMaterials,
}
