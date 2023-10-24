use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const PREFIX: &str = "./assets/new_rooms/";

#[derive(Clone, Serialize, Deserialize)]
pub struct Room {
    pub id: f32,
    pub tilemap: Vec<Vec<i32>>,
}

impl Room {
    /// Takes a source room file (.ron) and turns it into a Room so
    /// the raw tile numbers can be used later to assign tiles.
    /// The id is just a number (probably meaningless?).
    pub fn new(file_name: String) -> Self {
        let path = format!("{}{}", PREFIX, file_name);
        let file = match File::open(path) {
            Ok(file) => file,
            Err(err) => panic!("Can't open room file {}: {}", file_name, err.to_string()),
        };

        let reader = BufReader::new(file);

        ron::de::from_reader(reader).unwrap()
    }
}
