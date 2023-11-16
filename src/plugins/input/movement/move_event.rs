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
        if let Some(event) = event_reader.read().next() {
            if let Some(action) = event.action {
                match action {
                    MovePathAction::InsertOrActivate => {
                        let mut move_path = event.move_path.clone().unwrap();
                        move_path.set_active();
                        move_path.set_traversing();
                        commands.insert_resource::<MovementPath>(move_path);
                        // }
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
