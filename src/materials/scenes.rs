use bevy::prelude::*;

use crate::materials::heroes::HeroesMaterials;
use crate::materials::icon::IconMaterials;
use crate::materials::menu_box::MenuBoxMaterials;

#[derive(Resource)]
pub struct ScenesMaterials {
    pub main_background_image: Handle<Image>,
    pub sub_background_image: Handle<Image>,
    pub menu_box_materials: MenuBoxMaterials,
    pub icon_materials: IconMaterials,
    pub heroes_materials: HeroesMaterials,
}
