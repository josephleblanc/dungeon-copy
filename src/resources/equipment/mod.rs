use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

use self::weapon::{Weapon, WeaponName};

pub mod weapon;

pub const WEAPON_DATA: &str = "assets/equipment/weapons/weapon_data.ron";

#[derive(Deref, DerefMut, Resource, Serialize, Deserialize)]
pub struct Armory(HashMap<WeaponName, Weapon>);

impl Armory {
    pub fn new() -> Self {
        match File::open(WEAPON_DATA) {
            Ok(file) => {
                let reader = BufReader::new(file);
                ron::de::from_reader(reader).unwrap()
            }
            Err(err) => panic!("Can't find language file: {}", err),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Enhancement {
    Plus1,
    Plus2,
    Plus3,
    Plus4,
    Plus5,
    Plus6,
    Plus7,
    Plus8,
    Plus9,
    Plus10,
}
