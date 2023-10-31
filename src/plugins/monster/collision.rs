use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::components::player::PlayerComponent;
use crate::config::TILE_SIZE;
use crate::plugins::player::{PLAYER_SIZE_HEIGHT, PLAYER_SIZE_WIDTH};
use crate::resources::player::player_available_movement::PlayerAvailableMovement;

#[derive(Component, Clone, Debug)]
pub struct MonsterBox {
    pub width: f32,
    pub height: f32,
}

pub fn monster_collision_check(
    player_position: Vec3,
    monster_query: &Query<(&MonsterBox, &Transform), Without<PlayerComponent>>,
) -> PlayerAvailableMovement {
    let mut player_available_movement = PlayerAvailableMovement::new_all_true();
    let player_size = Vec2::new(PLAYER_SIZE_WIDTH, PLAYER_SIZE_HEIGHT);

    // TODO: Add support for more sizes
    // I'll need to add more sizes here to match medium, large, etc. sized creatures.
    let monster_size = Vec2::new(TILE_SIZE, TILE_SIZE);

    let right = player_position + Vec3::new(TILE_SIZE, 0.0, 0.0);
    let left = player_position + Vec3::new(-TILE_SIZE, 0.0, 0.0);
    let up = player_position + Vec3::new(0.0, TILE_SIZE, 0.0);
    let down = player_position + Vec3::new(0.0, -TILE_SIZE, 0.0);
    for (_monster_box, monster_position) in monster_query.iter() {
        if collide(up, player_size, monster_position.translation, monster_size).is_some() {
            player_available_movement.can_move_up = false;
        }
        if collide(
            right,
            player_size,
            monster_position.translation,
            monster_size,
        )
        .is_some()
        {
            player_available_movement.can_move_up = false;
        }
        if collide(
            down,
            player_size,
            monster_position.translation,
            monster_size,
        )
        .is_some()
        {
            player_available_movement.can_move_up = false;
        }
        if collide(
            left,
            player_size,
            monster_position.translation,
            monster_size,
        )
        .is_some()
        {
            player_available_movement.can_move_up = false;
        }
    }
    todo!()
}
