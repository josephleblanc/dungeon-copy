// Inspired by https://github.com/Anshorei/bevy_rei/tree/master

use crate::config::{RESOLUTION, TILE_SIZE, WINDOW_HEIGHT};
use bevy::prelude::*;
use std::ops::{Deref, DerefMut};

use crate::scenes::SceneState;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            interact_system.run_if(in_state(SceneState::InGameClassicMode)),
        );
    }
}

// #[derive(Resource, Default)]
// pub struct InteractingEntity {
//     pub entity: Entity,
// }

// impl Deref for InteractingEntity {
//     type Target = Entity;
//     fn deref(&self) -> &Self::Target {
//         &self.entity
//     }
// }
//
// impl DerefMut for InteractingEntity {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.entity
//     }
// }
//
// impl<T> AsRef<T> for InteractingEntity
// where
//     T: ?Sized,
//     <InteractingEntity as Deref>::Target: AsRef<T>,
// {
//     fn as_ref(&self) -> &T {
//         self.deref().as_ref()
//     }
// }

#[derive(Component, Reflect)]
pub struct Interactable {
    pub bounding_box: BoundingBox,
    pub active: bool,
    pub focused: bool,
}

impl Interactable {
    pub fn new(lower: Vec2, upper: Vec2) -> Interactable {
        Interactable {
            // The values defining the lower x,y values and upper x,y values
            // of the rectangle containing the interactable entity.
            bounding_box: BoundingBox { upper, lower },
            /// Whether or not the entity is currently interactable.
            active: true,
            /// Whether or not the entity is currently being focused by the
            /// cursor.
            focused: false,
        }
    }
}

#[derive(Component, Reflect, Debug)]
pub struct BoundingBox {
    pub upper: Vec2,
    pub lower: Vec2,
}

// TODO: Expand functionality of BoundingBox and interact_system() to include
// None, Hovered, and Pressed
impl BoundingBox {
    pub fn contains(&self, cursor_pos: Vec2) -> bool {
        self.lower.min(cursor_pos) == self.lower && self.upper.max(cursor_pos) == self.upper
    }
}

pub fn interact_system(
    window_query: Query<&Window>,
    mut interactable_query: Query<(&mut Interactable, Entity)>,
    // mut interacting_entity: ResMut<InteractingEntity>,
) {
    for window in window_query.iter() {
        if let Some(cursor_pos) = window.cursor_position() {
            for (mut interactable, entity) in interactable_query.iter_mut() {
                interactable.focused = interactable.bounding_box.contains(cursor_pos);
                // **interacting_entity = entity;
            }
        }
    }
}
