use bevy::prelude::*;

use crate::materials::menu_box::MenuBoxMaterials;

#[derive(Resource)]
pub struct ScenesMaterials {
    pub main_background_image: Handle<Image>,
    pub sub_background_image: Handle<Image>,
    pub menu_box_materials: MenuBoxMaterials,
}
