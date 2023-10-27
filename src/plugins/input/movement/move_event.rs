use bevy::prelude::*;

use crate::plugins::input::click_move::MovementPathList;
use crate::plugins::input::movement::path_move::MovementPath;
use crate::plugins::input::movement::MovementMode;
use crate::plugins::input::movement::MovementModeRes;

#[derive(Event, Debug)]
pub struct MovementPathEvent {
    move_path: Option<MovementPath>,
    action: Option<MovePathAction>,
}

impl MovementPathEvent {
    pub fn new() -> Self {
        MovementPathEvent {
            move_path: None,
            action: None,
        }
    }
    pub fn set_move_path(&mut self, value: MovementPath) -> &mut Self {
        self.move_path = Some(value);
        self
    }

    pub fn set_action(&mut self, value: MovePathAction) -> &mut Self {
        self.action = Some(value);
        self
    }
}

impl From<MovePathAction> for MovementPathEvent {
    fn from(value: MovePathAction) -> Self {
        let mut move_path = Self::new();
        move_path.set_action(value);
        move_path
    }
}

#[derive(Clone, Debug, Copy)]
pub enum MovePathAction {
    InsertOrActivate,
    Remove,
    // TakePath,
}

pub fn move_event_system(
    mut commands: Commands,
    move_path: Option<ResMut<MovementPath>>,
    mut event_reader: EventReader<MovementPathEvent>,
    movement_mode: Res<MovementModeRes>,
) {
    let debug = false;
    if debug {
        println!("debug | move_event_system | start move_event_system");
    }
    if **movement_mode == MovementMode::TurnBasedMovement {
        if let Some(event) = event_reader.into_iter().next() {
            if let Some(action) = event.action {
                match action {
                    MovePathAction::InsertOrActivate => {
                        println!(
                            "debug | move_event_system | MovePathEvent::InsertOrActivate received"
                        );
                        if let Some(mut movement_path) = move_path {
                            if !movement_path
                                .path
                                .iter()
                                .zip(event.move_path.clone().unwrap().path.iter())
                                .inspect(|item| println!("{:?}", item))
                                .any(|(step_old, step_new)| step_old != step_new)
                            {
                                println!(
                                    "debug | move_event_system | movement_path.is_active() = {}",
                                    movement_path.is_active()
                                );
                                if !movement_path.is_active() {
                                    movement_path.set_active();
                                }
                            }
                        } else {
                            commands.insert_resource::<MovementPath>(
                                event.move_path.to_owned().unwrap(),
                            );
                        }
                    }
                    MovePathAction::Remove => {
                        if move_path.is_some() {
                            commands.remove_resource::<MovementPath>();
                            commands.remove_resource::<MovementPathList>();
                        }
                    } // MovePathAction::TakePath => todo!(),
                }
            }
        }
    }
}
