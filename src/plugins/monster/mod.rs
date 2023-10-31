use crate::scenes::SceneState;
use bevy::prelude::*;

pub mod animation;
pub mod collision;
pub mod spawn;

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(SceneState::InGameClassicMode),
            spawn::spawn_training_dummy,
        );
    }
}
