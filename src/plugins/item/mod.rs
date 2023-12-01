#![allow(dead_code, unused_variables)]

use bevy::prelude::*;

pub mod equipment;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {}
}
