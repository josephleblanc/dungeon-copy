use bevy::prelude::*;
use bevy::utils::Duration;

use crate::components::player::PlayerComponent;
use crate::components::player_animation::PlayerAnimation;
use crate::plugins::game_ui::turn_mode::{MovementMode, MovementModeRes};
use crate::plugins::input::movement::turn_based::to_nearest_square;
use crate::plugins::input::movement::turn_based::turn_based_movement;
use crate::plugins::input::movement::wander::wander_movement_system;
use crate::plugins::interact::Interactable;
use crate::resources::dungeon::block_type::BlockType;
use crate::resources::dungeon::grid_square::GridSquare;

pub mod click_move;
pub mod turn_based;
pub mod wander;

/// Hands off player movement control to the correct system, either
/// wander or turn-based.
pub fn player_movement_system(
    movement_mode: Res<MovementModeRes>,
    player_query: Query<(&PlayerComponent, &mut PlayerAnimation, &mut Transform)>,
    block_type_query: Query<(&BlockType, &Transform), Without<PlayerComponent>>,
    ground_query: Query<(&Transform, &Interactable), (Without<PlayerComponent>, With<GridSquare>)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    movement: ResMut<Movement>,
) {
    if movement_mode.is_changed() && **movement_mode == MovementMode::TurnBasedMovement {
        to_nearest_square(player_query, ground_query, movement);
    } else {
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
}

#[derive(Resource, Debug)]
pub struct Movement {
    timer: Timer,
    moving: bool,
    target: Option<Vec2>,
    start: Option<Vec2>,
    delta: Option<Vec2>,
    delta_length: Option<f32>,
    pos: Option<Vec2>,
    moved_length: Option<f32>,
    speed: Option<f32>,
}

impl Movement {
    // TODO: Change to set_delta() and make another function called set_target
    // that takes `target` instead of `delta` as argument and adjusts `self`
    // similarly to the current `set_target()`
    pub fn set_target(&mut self, start: Vec2, delta: Vec2, speed: f32) {
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

    pub fn delta_over_time(&mut self, delta_time: Duration) -> Result<Vec2, &'static str> {
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
            let two_d = self.target.unwrap();
            *translation = Vec3::new(two_d.x, two_d.y, translation.z);
            self.pos = self.target;
        } else {
            *translation += Vec3::new(move_delta.x, move_delta.y, 0.0);
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
