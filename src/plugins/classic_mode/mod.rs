use bevy::prelude::*;

use crate::scenes::SceneState;

pub struct ClassicModePlugin;

#[derive(Resource)]
pub struct ClassicModeData {
    pub doors: Option<Entity>,
    pub ground: Option<Entity>,
    pub walls: Option<Entity>,
    pub end_point: Option<Entity>,
}

impl Plugin for ClassicModePlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(
        //     OnEnter(SceneState::PreClassicMode),
        //     dungeon::initiate::initiate_classic_mode,
        // );
    }
}
