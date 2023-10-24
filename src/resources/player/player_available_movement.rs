use bevy::prelude::*;
use std::ops::Neg;

use crate::{config::TILE_SIZE, plugins::input::movement::click_move::MoveNode};

pub struct PlayerAvailableMovement {
    pub can_move_left: bool,
    pub can_move_right: bool,
    pub can_move_up: bool,
    pub can_move_down: bool,
}

impl PlayerAvailableMovement {
    pub fn open_nodes(self, current_node: &MoveNode, dest: Vec3) -> Vec<MoveNode> {
        println!("debug | PlayerAvailableMovement::open_nodes | start");
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
        if self.can_move_down {
            let new_move = Vec3::new(0.0, TILE_SIZE.neg(), 0.0);
            open_paths.push(current_node.to_new_pos(new_move, dest));
        }
        println!(
            "debug | PlayerAvailableMovement::open_nodes | open_paths: {:?}",
            open_paths
        );
        open_paths
    }
}
