use bevy::prelude::*;

use crate::scenes::SceneState;

use self::attack::AttackPlugin;

pub mod attack;
pub mod bonus;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AttackPlugin);
    }
}
