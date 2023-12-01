#![allow(dead_code)]

/// The state of the animation. Currently has only `Idle, Moving, Hit`,
/// but more should be added.
// TODO: Add more animation states when animations are added.
#[derive(PartialEq, Eq)]
pub enum AnimationState {
    Idle,
    Moving,
    Hit,
}
