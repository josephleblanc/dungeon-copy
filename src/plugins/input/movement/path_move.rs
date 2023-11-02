use bevy::prelude::*;

use crate::plugins::input::movement::move_event::MovePathAction;
use crate::plugins::input::movement::move_event::MovementPathEvent;
use crate::plugins::input::movement::Movement;
use crate::plugins::input::movement::PlayerAnimation;
use crate::plugins::input::movement::PlayerComponent;

#[derive(Resource, Clone, Debug, PartialEq)]
pub struct MovementPath {
    pub path: Vec<(Vec3, Vec3)>,
    active: bool,
    traversing: bool,
}

impl MovementPath {
    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn set_active(&mut self) {
        self.active = true;
    }

    pub fn set_inactive(&mut self) {
        self.active = false;
    }

    pub fn new_inactive(path: Vec<(Vec3, Vec3)>) -> Self {
        Self {
            path,
            active: false,
            traversing: false,
        }
    }

    pub fn new_active(path: Vec<(Vec3, Vec3)>) -> Self {
        Self {
            path,
            active: true,
            traversing: false,
        }
    }

    pub fn is_traversing(&self) -> bool {
        self.traversing
    }

    pub fn set_traversing(&mut self) {
        self.traversing = true;
    }

    pub fn to_event(self, action: MovePathAction) -> MovementPathEvent {
        let mut event = MovementPathEvent::new();
        event.set_move_path(self).set_action(action);
        event
    }

    pub fn end(&self) -> Vec3 {
        self.path.first().unwrap().1
    }

    pub fn start(&self) -> Vec3 {
        self.path.last().unwrap().0
    }

    pub fn join(&mut self, other: Self) {
        self.path.extend_from_slice(other.path.as_slice())
    }
}

pub fn path_move_system(
    mut player_query: Query<(&PlayerComponent, &mut PlayerAnimation, &mut Transform)>,
    move_path: Option<ResMut<MovementPath>>,
    mut movement: ResMut<Movement>,
    time: Res<Time>,
    mut event_writer: EventWriter<MovementPathEvent>,
) {
    let debug = false;
    if debug {
        println!("debug | path_move_system | start path_move_system");
    }
    let (player_stats, _player_animation, mut transform) = player_query.single_mut();
    if let Some(mut move_path) = move_path {
        move_path.set_traversing();
        if debug {
            println!(
                "debug | path_move_system | move_path.is_active(): {}",
                move_path.is_active()
            );
        }
        if move_path.is_active() {
            if !movement.moving {
                if let Some((start, end)) = move_path.path.pop() {
                    let delta = end - start;
                    movement.set_target(
                        transform.translation.truncate(),
                        delta.truncate(),
                        player_stats.speed,
                    );
                } else {
                    event_writer.send(MovePathAction::Remove.into());
                }
            } else if !movement.is_finished() {
                let time_delta = time.delta();
                movement
                    .update(&mut transform.translation, time_delta)
                    .unwrap();
                if movement.is_finished() {
                    movement.reset();
                }
            }
        }
    }
}
