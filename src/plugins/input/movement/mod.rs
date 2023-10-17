use bevy::prelude::*;
use bevy::utils::Duration;

use crate::components::player::PlayerComponent;
use crate::components::player_animation::PlayerAnimation;
use crate::config::*;
use crate::plugins::game_ui::turn_mode::{MovementMode, MovementModeRes};
use crate::plugins::player::collisions::wall_collision_check;
use crate::resources::animation_state::AnimationState;
use crate::resources::dungeon::block_type::BlockType;

#[derive(Resource, Debug)]
pub struct Movement {
    timer: Timer,
    moving: bool,
    target: Option<Vec3>,
    start: Option<Vec3>,
    delta: Option<Vec3>,
    delta_length: Option<f32>,
    pos: Option<Vec3>,
    moved_length: Option<f32>,
    speed: Option<f32>,
}

impl Movement {
    pub fn set_target(&mut self, start: Vec3, delta: Vec3, speed: f32) {
        self.moving = true;
        self.target = Some(start + delta);
        self.start = Some(start);
        self.pos = Some(start);
        self.moved_length = Some(0.0);
        self.delta = Some(delta);
        self.delta_length = Some(delta.length());
        self.speed = Some(speed);
        println!("target set: {:#?}", self);
    }

    pub fn delta_over_time(&mut self, delta_time: Duration) -> Result<Vec3, &'static str> {
        if self.speed.is_some() && self.start.is_some() && self.target.is_some() {
            // let mut move_delta = self.speed.unwrap() * TILE_SIZE * delta_time.as_secs_f32();
            let mut move_delta =
                self.speed.unwrap() * self.delta.unwrap() * delta_time.as_secs_f32();
            let move_delta_length = move_delta.length();

            // make sure the movement doesn't overshoot the target.
            if move_delta_length + self.moved_length.unwrap() > self.delta_length.unwrap() {
                move_delta = move_delta.clamp_length_max(
                    (self.delta_length.unwrap() - self.moved_length.unwrap()).abs(),
                );
            }
            self.moved_length = Some(self.moved_length.unwrap() + move_delta.length());
            Ok(move_delta)
        } else {
            Err("Attempted to use delta_over_time method on empty movement.")
        }
    }

    pub fn update(&mut self, translation: &mut Vec3, tick: Duration) -> Result<(), &'static str> {
        let delta_time = self.smart_tick(tick);
        let move_delta = self.delta_over_time(delta_time)?;

        let near_end = (self.delta_length.unwrap() - self.moved_length.unwrap())
            .abs()
            .floor()
            == 0.0;
        // make sure the movement doesn't overshoot the target.
        if near_end || self.timer.finished() {
            *translation = self.target.unwrap();
            self.pos = self.target;
        } else {
            *translation += move_delta;
            self.pos = Some(self.pos.unwrap() + move_delta);
        };
        println!("{:?}", self);
        Ok(())
    }

    pub fn is_finished(&self) -> bool {
        self.pos == self.target
    }

    pub fn reset(&mut self) {
        self.timer.reset();
        self.moving = false;
        self.target = None;
        self.start = None;
        self.delta = None;
        self.pos = None;
        self.speed = None;
    }

    /// Calculates the amount of time passed since the last tick,
    /// unless the amount of time passed exceeds the remaining time left on the
    /// timer, in which case it returns the remaining time on the timer.
    /// Updates the timer on the Movement struct.
    fn smart_tick(&mut self, time_delta: Duration) -> Duration {
        let elapsed = self.timer.elapsed();
        self.timer.tick(time_delta);
        if self.timer.finished() {
            self.timer.duration() - elapsed
        } else {
            time_delta
        }
    }
}

impl Default for Movement {
    fn default() -> Self {
        Movement {
            timer: Timer::from_seconds(0.5, TimerMode::Once),
            moving: false,
            target: None,
            start: None,
            delta: None,
            pos: None,
            speed: None,
            delta_length: None,
            moved_length: None,
        }
    }
}

/// Hands off player movement control to the correct system, either
/// wander or turn-based.
pub fn player_movement_system(
    movement_mode: Res<MovementModeRes>,
    player_query: Query<(&PlayerComponent, &mut PlayerAnimation, &mut Transform)>,
    block_type_query: Query<(&BlockType, &Transform), Without<PlayerComponent>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    movement: ResMut<Movement>,
) {
    match **movement_mode {
        MovementMode::WanderMovement => {
            wander_movement_system(player_query, block_type_query, keyboard_input, time)
        }
        MovementMode::TurnBasedMovement => {
            turn_based_movement(
                player_query,
                block_type_query,
                keyboard_input,
                time,
                movement,
            );
        }
    };
}

pub fn wander_movement_system(
    mut player_query: Query<(&PlayerComponent, &mut PlayerAnimation, &mut Transform)>,
    block_type_query: Query<(&BlockType, &Transform), Without<PlayerComponent>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player_stats, mut player_animation, mut transform) = player_query.single_mut();

    let mut delta = Vec3::new(0.0, 0.0, 0.0);

    let player_position = transform.translation;
    player_animation.animation_state = AnimationState::Idle;

    let player_availalbe_movement = wall_collision_check(player_position, &block_type_query);

    if keyboard_input.pressed(KeyCode::W) && player_availalbe_movement.can_move_up {
        delta.y += player_stats.speed * TILE_SIZE * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::S) && player_availalbe_movement.can_move_down {
        delta.y -= player_stats.speed * TILE_SIZE * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::A) && player_availalbe_movement.can_move_left {
        delta.x -= player_stats.speed * TILE_SIZE * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::D) && player_availalbe_movement.can_move_right {
        delta.x += player_stats.speed * TILE_SIZE * time.delta_seconds();
    }

    transform.translation += delta;

    if delta.x < 0.0 {
        transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
    } else if delta.x > 0.0 {
        transform.rotation = Quat::default();
    }

    if delta != Vec3::ZERO {
        player_animation.animation_state = AnimationState::Moving;
    }
}

pub fn turn_based_movement(
    mut player_query: Query<(&PlayerComponent, &mut PlayerAnimation, &mut Transform)>,
    block_type_query: Query<(&BlockType, &Transform), Without<PlayerComponent>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut movement: ResMut<Movement>,
) {
    let (player_stats, mut player_animation, mut transform) = player_query.single_mut();

    let mut delta = Vec3::new(0.0, 0.0, 0.0);

    let player_position = transform.translation;
    player_animation.animation_state = AnimationState::Idle;

    let player_availalbe_movement = wall_collision_check(player_position, &block_type_query);

    if !movement.moving && movement.target.is_none() {
        if keyboard_input.pressed(KeyCode::W) && player_availalbe_movement.can_move_up {
            delta.y += TILE_SIZE;
        }

        if keyboard_input.pressed(KeyCode::S) && player_availalbe_movement.can_move_down {
            delta.y -= TILE_SIZE;
        }

        if keyboard_input.pressed(KeyCode::A) && player_availalbe_movement.can_move_left {
            delta.x -= TILE_SIZE;
        }

        if keyboard_input.pressed(KeyCode::D) && player_availalbe_movement.can_move_right {
            delta.x += TILE_SIZE;
        }
        if delta != Vec3::ZERO {
            if delta.x < 0.0 {
                transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
            } else if delta.x > 0.0 {
                transform.rotation = Quat::default();
            }
            movement.set_target(transform.translation, delta, player_stats.speed);
        }
    }

    if !movement.is_finished() && movement.moving {
        let time_delta = time.delta();
        println!(
            "debug | update movement for time.delta(): {}",
            time_delta.as_secs()
        );
        println!(
            "      | self.pos - self.target: {}",
            movement.pos.unwrap() - movement.target.unwrap()
        );
        movement
            .update(&mut transform.translation, time_delta)
            .unwrap();
        if movement.is_finished() {
            movement.reset();
        }
    }
}
