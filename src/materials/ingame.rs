use bevy::prelude::*;

use crate::materials::dungeon::DungeonMaterials;
use crate::materials::heroes::HeroesMaterials;
use crate::materials::map_ui::MapUiMaterials;
use crate::materials::monsters::MonstersMaterials;

#[derive(Resource)]
pub struct InGameMaterials {
    pub heroes_materials: HeroesMaterials,
    pub dungeon_materials: DungeonMaterials,
    pub map_ui: MapUiMaterials,
    pub monsters_materials: MonstersMaterials,
}
