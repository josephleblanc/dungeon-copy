#![allow(dead_code)]

use bevy::prelude::*;

use crate::plugins::input::path_move::MovementPath;

#[derive(Event, Debug, Clone)]
pub struct PathListEvent {
    pub path_list: Option<Vec<MovementPath>>,
    pub action: Option<PathListAction>,
}

impl PathListEvent {
    pub fn new(path_list: Option<Vec<MovementPath>>, action: Option<PathListAction>) -> Self {
        Self { path_list, action }
    }
}

impl From<PathListAction> for PathListEvent {
    fn from(value: PathListAction) -> Self {
        Self {
            path_list: None,
            action: Some(value),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PathListAction {
    Remove,
    AddPath,
    StartPath,
    StartMove,
}
