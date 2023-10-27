use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct MapGrid {
    pub positions: Vec<Vec2>,
}
