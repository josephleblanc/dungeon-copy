use bevy::prelude::*;

use crate::resources::animation_state::AnimationState;

/// Used to keep track of an animation state as it goes through its different frames.
#[derive(Component)]
pub struct PlayerAnimation {
    /// The amount of time between frames.
    pub animation_timer: Timer,
    /// The type of animation being shown, e.g. Walking, Idle, Attacking, etc.
    pub animation_state: AnimationState,
}

impl PlayerAnimation {
    pub fn new() -> Self {
        PlayerAnimation {
            animation_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            animation_state: AnimationState::Idle,
        }
    }
}
