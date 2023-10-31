use bevy::prelude::*;
use std::ops::Neg;

use crate::{config::TILE_SIZE, plugins::input::movement::click_move::MoveNode};

pub struct PlayerAvailableMovement {
    pub can_move_left: bool,
    pub can_move_right: bool,
    pub can_move_up: bool,
    pub can_move_down: bool,
    pub can_move_up_right: bool,
    pub can_move_up_left: bool,
    pub can_move_down_right: bool,
    pub can_move_down_left: bool,
}

impl PlayerAvailableMovement {
    pub fn open_nodes(self, current_node: &MoveNode, dest: Vec3) -> Vec<MoveNode> {
        let debug = false;
        if debug {
            println!("debug | PlayerAvailableMovement::open_nodes | start");
        }
        let mut open_paths: Vec<MoveNode> = Vec::with_capacity(4);
        if self.can_move_right {
            let new_move = Vec3::new(TILE_SIZE, 0.0, 0.0);
            open_paths.push(current_node.to_new_pos(new_move, dest));
        }
        if self.can_move_left {
            let new_move = Vec3::new(TILE_SIZE.neg(), 0.0, 0.0);
            open_paths.push(current_node.to_new_pos(new_move, dest));
        }
        if self.can_move_up {
            let new_move = Vec3::new(0.0, TILE_SIZE, 0.0);
            open_paths.push(current_node.to_new_pos(new_move, dest));
        }
        if self.can_move_up_right {
            let new_move = Vec3::new(TILE_SIZE, TILE_SIZE, 0.0);
            open_paths.push(current_node.to_new_pos(new_move, dest));
        }
        if self.can_move_up_left {
            let new_move = Vec3::new(TILE_SIZE.neg(), TILE_SIZE, 0.0);
            open_paths.push(current_node.to_new_pos(new_move, dest));
        }
        if self.can_move_down {
            let new_move = Vec3::new(0.0, TILE_SIZE.neg(), 0.0);
            open_paths.push(current_node.to_new_pos(new_move, dest));
        }
        if self.can_move_down_right {
            let new_move = Vec3::new(TILE_SIZE, TILE_SIZE.neg(), 0.0);
            open_paths.push(current_node.to_new_pos(new_move, dest));
        }
        if self.can_move_down_left {
            let new_move = Vec3::new(TILE_SIZE.neg(), TILE_SIZE.neg(), 0.0);
            open_paths.push(current_node.to_new_pos(new_move, dest));
        }
        if debug {
            println!(
                "debug | PlayerAvailableMovement::open_nodes | open_paths: {:?}",
                open_paths
            );
        }
        open_paths
    }

    pub fn new_all_true() -> Self {
        PlayerAvailableMovement {
            can_move_up: true,
            can_move_down: true,
            can_move_left: true,
            can_move_right: true,
            can_move_up_right: true,
            can_move_up_left: true,
            can_move_down_right: true,
            can_move_down_left: true,
        }
    }

    pub fn update_diagonals(&mut self) -> &mut Self {
        self.can_move_up_right = self.can_move_up && self.can_move_right;
        self.can_move_up_left = self.can_move_up && self.can_move_left;
        self.can_move_down_right = self.can_move_down && self.can_move_right;
        self.can_move_down_left = self.can_move_down && self.can_move_left;
        self
    }
}
