#![allow(dead_code)]

use bevy::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum Moral {
    Good,
    Evil,
    Neutral,
}

#[derive(Debug, Copy, Clone)]
pub enum Order {
    Lawful,
    Neutral,
    Chaotic,
}

#[derive(Debug, Component, Copy, Clone)]
pub struct Alignment {
    pub moral: Moral,
    pub order: Order,
}
