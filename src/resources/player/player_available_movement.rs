use bevy::prelude::*;
use std::ops::Neg;
use std::slice::Iter;

use crate::{config::TILE_SIZE, plugins::input::movement::click_move::MoveNode};

#[derive(Debug, Clone, Copy, Deref, DerefMut)]
pub struct PlayerAvailableMovement([(MoveDirection, bool); 8]);

impl PlayerAvailableMovement {
    pub fn can_move_up(&self) -> bool {
        self.iter()
            .any(|(dir, val)| *dir == MoveDirection::Up && *val)
    }

    pub fn can_move_right(&self) -> bool {
        self.iter()
            .any(|(dir, val)| *dir == MoveDirection::Right && *val)
    }

    pub fn can_move_down(&self) -> bool {
        self.iter()
            .any(|(dir, val)| *dir == MoveDirection::Down && *val)
    }

    pub fn can_move_left(&self) -> bool {
        self.iter()
            .any(|(dir, val)| *dir == MoveDirection::Left && *val)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MoveDirection {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl MoveDirection {
    pub fn iterator() -> Iter<'static, MoveDirection> {
        [
            Self::Up,
            Self::UpRight,
            Self::Right,
            Self::DownRight,
            Self::Down,
            Self::DownLeft,
            Self::Left,
            Self::UpLeft,
        ]
        .iter()
    }

    pub fn to_offset(self) -> Vec3 {
        match self {
            Self::Up => Vec3::new(0.0, TILE_SIZE, 0.0),
            Self::UpRight => Vec3::new(TILE_SIZE, TILE_SIZE, 0.0),
            Self::Right => Vec3::new(TILE_SIZE, 0.0, 0.0),
            Self::DownRight => Vec3::new(TILE_SIZE, -TILE_SIZE, 0.0),
            Self::Down => Vec3::new(0.0, -TILE_SIZE, 0.0),
            Self::DownLeft => Vec3::new(-TILE_SIZE, -TILE_SIZE, 0.0),
            Self::Left => Vec3::new(-TILE_SIZE, 0.0, 0.0),
            Self::UpLeft => Vec3::new(-TILE_SIZE, TILE_SIZE, 0.0),
        }
    }
}

impl PlayerAvailableMovement {
    pub fn check_direction(&self, direction: MoveDirection) -> bool {
        let mut true_self = self.into_iter().filter(|(_, val)| *val).map(|(dir, _)| dir);
        true_self.any(|dir| dir == direction)
    }
    pub fn open_nodes(self, current_node: &MoveNode, dest: Vec3) -> Vec<MoveNode> {
        let debug = false;
        if debug {
            println!("debug | PlayerAvailableMovement::open_nodes | start");
        }
        let mut open_paths: Vec<MoveNode> = Vec::with_capacity(8);
        for (move_direction, _) in self.iter().filter(|(_move_dir, can_move)| *can_move) {
            let new_move = match *move_direction {
                MoveDirection::Up => Vec3::new(0.0, TILE_SIZE, 0.0),
                MoveDirection::UpRight => Vec3::new(TILE_SIZE, TILE_SIZE, 0.0),
                MoveDirection::Right => Vec3::new(TILE_SIZE, 0.0, 0.0),
                MoveDirection::DownRight => Vec3::new(TILE_SIZE, TILE_SIZE.neg(), 0.0),
                MoveDirection::Down => Vec3::new(0.0, TILE_SIZE.neg(), 0.0),
                MoveDirection::DownLeft => Vec3::new(TILE_SIZE.neg(), TILE_SIZE.neg(), 0.0),
                MoveDirection::Left => Vec3::new(TILE_SIZE.neg(), 0.0, 0.0),
                MoveDirection::UpLeft => Vec3::new(TILE_SIZE.neg(), TILE_SIZE, 0.0),
            };
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
        PlayerAvailableMovement([
            (MoveDirection::Up, true),
            (MoveDirection::UpRight, true),
            (MoveDirection::Right, true),
            (MoveDirection::DownRight, true),
            (MoveDirection::Down, true),
            (MoveDirection::DownLeft, true),
            (MoveDirection::Left, true),
            (MoveDirection::UpLeft, true),
        ])
    }

    pub fn merge(&mut self, other: Self) -> &mut Self {
        for ((_dir, val), (_other_dir, other_val)) in self.iter_mut().zip(other.iter()) {
            *val = *val && *other_val;
        }
        self
    }

    // TODO: This is currently buggy - e.g. it will return can_move_up_right
    // true when the up and right are clear, but there still may be something
    // occupying that square.
    // Fix when I have placed objects to move around.
    /// Updates the values of the diagonal directions of self, given that the
    /// cardinal directions have been changed.
    pub fn update_diagonals(&mut self) -> &mut Self {
        use MoveDirection::*;
        let mut true_self = self.into_iter().filter(|(_, val)| *val).map(|(dir, _)| dir);
        for (move_dir, val) in self.iter_mut() {
            *val = match *move_dir {
                UpRight => true_self.any(|dir| dir == Up) && true_self.any(|dir| dir == Right),
                DownRight => true_self.any(|dir| dir == Down) && true_self.any(|dir| dir == Right),
                DownLeft => true_self.any(|dir| dir == Down) && true_self.any(|dir| dir == Left),
                UpLeft => true_self.any(|dir| dir == Up) && true_self.any(|dir| dir == Left),
                _ => *val,
            };
        }
        self
    }
}

#[derive(DerefMut, Deref)]
pub struct Something([String; 2]);
