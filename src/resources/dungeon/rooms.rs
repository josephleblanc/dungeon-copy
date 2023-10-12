use bevy::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};

use crate::config::LIST_ROOM_FILE;
use crate::resources::dungeon::room::Room;

#[derive(Resource, Clone)]
pub struct Rooms {
    rooms: Vec<Room>,
}

impl Rooms {
    /// Build a vec of rooms, in the order listed in their source file.
    pub fn new() -> Self {
        let file = match File::open(LIST_ROOM_FILE) {
            Ok(file) => file,
            Err(err) => panic!("Can't open list room file: {}", err.to_string()),
        };

        let buffered = io::BufReader::new(file);
        let lines: Vec<_> = buffered.lines().collect();

        let mut rooms: Vec<Room> = Vec::new();

        for line in lines {
            let file_name = line.unwrap();
            rooms.push(Room::new(file_name));
        }

        Rooms { rooms }
    }

    // search for the room by its id in the vec of rooms.
    pub fn get_room(&self, room_id: f32) -> Room {
        let result = self
            .rooms
            .iter()
            .find(|room| room.id == room_id.floor())
            .expect(format!("Can't find room: {}", room_id).as_str());

        result.clone()
    }
}
