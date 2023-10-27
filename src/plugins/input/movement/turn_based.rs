use bevy::prelude::*;
use bevy::utils::Duration;

use crate::components::player::PlayerComponent;
use crate::components::player_animation::PlayerAnimation;
use crate::config::*;
use crate::plugins::game_ui::translate::trans_to_window;
use crate::plugins::interact::Interactable;
use crate::plugins::player::collisions::wall_collision_check;
use crate::resources::animation_state::AnimationState;
use crate::resources::dungeon::block_type::BlockType;
use crate::resources::dungeon::grid_square::GridSquare;

use super::Movement;

pub fn turn_based_movement(
    mut player_query: Query<(&PlayerComponent, &mut PlayerAnimation, &mut Transform)>,
    block_type_query: Query<(&BlockType, &Transform), Without<PlayerComponent>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut movement: ResMut<Movement>,
) {
    let debug = false;
    let (player_stats, mut player_animation, mut transform) = player_query.single_mut();

    let mut delta = Vec3::new(0.0, 0.0, 0.0);

    let player_position = transform.translation;
    player_animation.animation_state = AnimationState::Idle;

    let player_available_movement = wall_collision_check(player_position, &block_type_query);

    if !movement.moving && movement.target.is_none() {
        if keyboard_input.pressed(KeyCode::W) && player_available_movement.can_move_up {
            delta.y += TILE_SIZE;
        }

        if keyboard_input.pressed(KeyCode::S) && player_available_movement.can_move_down {
            delta.y -= TILE_SIZE;
        }

        if keyboard_input.pressed(KeyCode::A) && player_available_movement.can_move_left {
            delta.x -= TILE_SIZE;
        }

        if keyboard_input.pressed(KeyCode::D) && player_available_movement.can_move_right {
            delta.x += TILE_SIZE;
        }
        if delta != Vec3::ZERO {
            if delta.x < 0.0 {
                transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
            } else if delta.x > 0.0 {
                transform.rotation = Quat::default();
            }
            movement.set_target(
                transform.translation.truncate(),
                delta.truncate(),
                player_stats.speed,
            );
        }
    }

    if !movement.is_finished() && movement.moving {
        let time_delta = time.delta();
        if debug {
            println!(
                "debug | update movement for time.delta(): {}",
                time_delta.as_secs()
            );
            println!(
                "      | self.pos - self.target: {}",
                movement.pos.unwrap() - movement.target.unwrap()
            );
        }
        movement
            .update(&mut transform.translation, time_delta)
            .unwrap();
        if movement.is_finished() {
            movement.reset();
        }
    }
}

pub fn to_nearest_square(
    mut player_query: Query<(&PlayerComponent, &mut PlayerAnimation, &mut Transform)>,
    ground_query: Query<(&Transform, &Interactable), (Without<PlayerComponent>, With<GridSquare>)>,
    mut movement: ResMut<Movement>,
) {
    let debug = false;
    let (player_stats, mut player_animation, mut transform) = player_query.single_mut();

    let player_position = transform.translation;
    player_animation.animation_state = AnimationState::Moving;

    let nearest_square_center = ground_query
        .iter()
        .filter(|(_, interactable)| interactable.bound_tr.contains(player_position.truncate()))
        .map(|(transform, _)| transform.translation)
        .next()
        .unwrap();

    if debug {
        println!("debug | nearest_square_center: {:?}", nearest_square_center);
    }
    let delta = nearest_square_center - player_position;

    if debug {
        println!("debug | delta: {:?}", delta);
    }
    movement.set_target(
        player_position.truncate(),
        delta.truncate(),
        player_stats.speed,
    );
}
