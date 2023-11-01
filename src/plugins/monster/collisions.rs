use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::components::player::PlayerComponent;
use crate::config::TILE_SIZE;
use crate::plugins::player::{PLAYER_SIZE_HEIGHT, PLAYER_SIZE_WIDTH};
use crate::resources::monster::Monster;
use crate::resources::player::player_available_movement::PlayerAvailableMovement;

#[derive(Component, Clone, Debug)]
pub struct MonsterBox {
    pub width: f32,
    pub height: f32,
}

pub fn monster_collision_check(
    player_position: Vec3,
    monster_query: &Query<(&Monster, &Transform), Without<PlayerComponent>>,
) -> PlayerAvailableMovement {
    let debug = true;
    let mut player_available_movement = PlayerAvailableMovement::new_all_true();
    let player_size = Vec2::new(PLAYER_SIZE_WIDTH, PLAYER_SIZE_HEIGHT);

    // TODO: Add support for more sizes
    // I'll need to add more sizes here to match medium, large, etc. sized creatures.
    let monster_size = Vec2::new(TILE_SIZE, TILE_SIZE - 40.0);

    for (_monster_box, monster_position) in monster_query.iter() {
        for (move_dir, can_move) in player_available_movement.iter_mut() {
            let new_player_pos = player_position + move_dir.to_offset();
            *can_move = collide(
                new_player_pos,
                player_size,
                monster_position.translation,
                monster_size,
            )
            .is_none();
            if debug {
                println!(
                    "collisions | monster_collision_check | \n\t\
                player_pos, new_player_pos, monster_position.translation: \
                \n\t{:?}: {}, {}, {}, \tcan_move: {}",
                    move_dir,
                    player_position,
                    new_player_pos,
                    monster_position.translation,
                    can_move
                );
                // println!(
                //     "collisions | monster_collision_check | new_player_pos: \n\t{}",
                //     new_player_pos
                // );
                // println!(
                //     "collisions | monster_collision_check | monster_position.translation: \n\t{}",
                //     monster_position.translation
                // );
                // println!(
                //     "collisions | monster_collision_check | can_move: \n\t{}",
                //     can_move
                // );
                // println!("collisions | monster_collision_check | player_pos, monster_pos collision is_some: {}\n",
                // collide(player_position, player_size, monster_position.translation, monster_size).is_some());
            }
        }
    }
    player_available_movement
}
