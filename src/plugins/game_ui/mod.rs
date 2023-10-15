use bevy::prelude::*;

use crate::scenes::SceneState;

pub mod turn_mode;

#[derive(Resource)]
pub struct IngameUiData {
    user_interface_root: Entity,
}

pub struct IngameUiPlugin;

impl Plugin for IngameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::InGameClassicMode), turn_mode::setup);
    }
}
