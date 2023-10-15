use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
// use rand::Rng;
// use std::time::Duration;

use crate::components::player::PlayerComponent;
// use crate::components::player_animation::PlayerAnimation;
use crate::config::*;
use crate::plugins::player::{PLAYER_SIZE_HEIGHT, PLAYER_SIZE_WIDTH};
// use crate::resources::animation_state::AnimationState;
use crate::resources::dungeon::block_type::BlockType;
use crate::resources::player::player_available_movement::PlayerAvailableMovement;

/// Checks if there is a collision with a wall.
/// This function creates a PlayerAvailableMovement struct used in
/// player_movement_handle_system to restrict player movement if there is a
/// wall in that direction.
// TODO: This function currently only checks for walls, but should be altered
// to account for anything that should stop player movement, both in and out
// of combat.
pub fn wall_collision_check(
    player_position: Vec3,
    block_type_query: &Query<(&BlockType, &Transform), Without<PlayerComponent>>,
) -> PlayerAvailableMovement {
    let mut player_available_movement = PlayerAvailableMovement {
        can_move_up: true,
        can_move_down: true,
        can_move_left: true,
        can_move_right: true,
    };

    let player_size = Vec2::new(PLAYER_SIZE_WIDTH, PLAYER_SIZE_HEIGHT);

    for (block_type, block_transform) in block_type_query.iter() {
        // sets the block_position of the wall to the bottom of the wall for
        // WallTop walls. This lets the player move "behind" the wall,
        // making it seems as though the player is colliding with the wall
        // base.
        let block_position = match *block_type {
            BlockType::WallTop => block_transform.translation + Vec3::new(0.0, 64.0, 0.0),
            _ => block_transform.translation,
        };

        let block_size = match *block_type {
            BlockType::WallBottom => Vec2::new(TILE_SIZE, TILE_SIZE),
            BlockType::WallTop => Vec2::new(TILE_SIZE, TILE_SIZE),
            BlockType::WallLeft => Vec2::new(TILE_SIZE, TILE_SIZE),
            BlockType::WallRight => Vec2::new(TILE_SIZE, TILE_SIZE),
            BlockType::None => Vec2::new(0.0, 0.0),
        };

        if *block_type == BlockType::None {
            continue;
        }

        if collide(player_position, player_size, block_position, block_size).is_some() {
            match *block_type {
                BlockType::WallTop => player_available_movement.can_move_up = false,
                BlockType::WallBottom => player_available_movement.can_move_down = false,
                BlockType::WallLeft => player_available_movement.can_move_left = false,
                BlockType::WallRight => player_available_movement.can_move_right = false,
                BlockType::None => {}
            }
        }
    }
    player_available_movement
}
