// Inspired by https://github.com/Anshorei/bevy_rei/tree/master

use bevy::prelude::*;

use crate::scenes::SceneState;

pub mod grid_map;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            interact_system.run_if(in_state(SceneState::InGameClassicMode)),
        );
    }
}

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

#[derive(Component, Reflect)]
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
    mut interactable_query: Query<&mut Interactable>,
) {
    for window in window_query.iter() {
        if let Some(cursor_pos) = window.cursor_position() {
            println!("debug | cursor_pos: {}", cursor_pos);
            for mut interactable in interactable_query.iter_mut() {
                interactable.focused = interactable.bounding_box.contains(cursor_pos);
            }
        }
    }
}
