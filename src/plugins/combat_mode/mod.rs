use bevy::prelude::*;

use self::initiative::Initiative;

pub mod initiative;
pub mod state;

pub struct CombatModePlugin;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
pub struct CombatMode;

impl Plugin for CombatModePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TurnOrder>();
    }
}

#[derive(Clone, Deref, DerefMut, Resource, Default)]
pub struct TurnOrder(Vec<(Entity, Initiative)>);
