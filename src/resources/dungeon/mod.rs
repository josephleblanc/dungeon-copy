use bevy::prelude::*;

use rand::distributions::{Distribution, Uniform};
use std::fs::File;
use std::io::{self, BufRead};

// TODO: A number of these mod files are just a single Component.
// Think about moving them into this file.
pub mod block_type;
pub mod door;
pub mod doors;
pub mod end_point;
pub mod floor;
pub mod grid_square;
pub mod ground;
pub mod layer;
pub mod position;
pub mod room;
pub mod rooms;
pub mod wall;
pub mod wall_type;
pub mod walls;

use crate::config::LIST_FLOOR_FILE;
use floor::Floor;

const TOTAL_DUNGEON_FLOORS: usize = 5;

#[derive(Resource)]
pub struct Dungeon {
    pub floors: Vec<Floor>,
    pub current_floor: Floor,
}

impl Dungeon {
    /// Creates a new, random ordering of the floors in LIST_FLOOR_FILE, stores
    /// them in Dungeon.floors, and initializes Dungeon.current_floor to the
    /// first in the random sequence.
    pub fn new() -> Self {
        let file = match File::open(LIST_FLOOR_FILE) {
            Ok(file) => file,
            Err(err) => panic!("Can't open list floor file: {}", err.to_string()),
        };

        let buffered = io::BufReader::new(file);
        let lines: Vec<_> = buffered.lines().collect();
        let total_available_floors: usize = lines.len();

        let mut floors: Vec<Floor> = Vec::new();

        let mut rng = rand::thread_rng();
        let dice = Uniform::from(0..total_available_floors);

        let mut floors_indexes: Vec<usize> = Vec::new();
        loop {
            let throw = dice.sample(&mut rng);

            if !floors_indexes.contains(&throw) {
                floors_indexes.push(throw);
            }

            if floors_indexes.len() == TOTAL_DUNGEON_FLOORS {
                break;
            }
        }

        let mut floor_index = 0;

        for (index, line) in lines.into_iter().enumerate() {
            if floors_indexes.contains(&index) {
                let map_name = line.unwrap();
                if floor_index == TOTAL_DUNGEON_FLOORS - 1 {
                    floors.push(Floor::new(map_name, true));
                } else {
                    floors.push(Floor::new(map_name, false));
                }
                floor_index += 1;
            }
        }

        let current_floor = floors[0].clone();

        Dungeon {
            floors,
            current_floor,
        }
    }
}
