// Inspired by https://github.com/Anshorei/bevy_rei/tree/master

use crate::config::TILE_SIZE;
use bevy::prelude::*;
// use std::ops::{Deref, DerefMut};

use crate::scenes::SceneState;

use crate::plugins::game_ui::translate::trans_to_window;
use crate::plugins::game_ui::translate::window_to_trans;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            interact_system.run_if(in_state(SceneState::InGameClassicMode)),
        );
    }
}

#[derive(Component, Reflect, Debug, Copy, Clone)]
pub enum ReferenceFrame {
    WindowFrame,
    Translation,
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

#[derive(Component, Reflect, Copy, Clone)]
pub struct Interactable {
    /// The values defining the lower x,y values and upper x,y values
    /// of the rectangle containing the interactable entity.
    /// These values are translation values, by default with an origin
    /// at the center of the screen in a cartesian fashion.
    pub bound_tr: BoundingBox,
    // These values are window frame values, in reference to the top
    // left pixel, with positive y going down the screen, and positive
    // x going to the right.
    pub bound_wf: BoundingBox,
    /// Whether or not the entity is currently interactable.
    pub active: bool,
    /// Whether or not the entity is currently being focused by the
    /// cursor.
    pub focused: bool,
}

impl Interactable {
    /// constructs a new Interactable struct from a lower and upper bound.
    /// This `from_trans` version has bounding box points in the lower left
    /// and upper right corner of the box.
    pub fn new_from_trans(lower: Vec2, upper: Vec2) -> Interactable {
        let bound_tr = BoundingBox {
            upper,
            lower,
            reference_frame: ReferenceFrame::Translation,
            height: TILE_SIZE,
        };
        Interactable {
            bound_tr,
            bound_wf: bound_tr.to_wf(),
            active: true,
            focused: false,
        }
    }

    /// constructs a new Interactable struct from a lower and upper bound.
    /// This `from_trans` version has bounding box points in the upper left
    /// and lower right corner of the box.
    pub fn new_from_window(lower: Vec2, upper: Vec2) -> Interactable {
        let bound_wf = BoundingBox {
            upper,
            lower,
            reference_frame: ReferenceFrame::WindowFrame,
            height: TILE_SIZE,
        };
        Interactable {
            bound_tr: bound_wf.to_trans(),
            bound_wf,
            active: true,
            focused: false,
        }
    }
}

#[derive(Component, Reflect, Debug, Copy, Clone)]
pub struct BoundingBox {
    pub upper: Vec2,
    pub lower: Vec2,
    pub reference_frame: ReferenceFrame,
    pub height: f32,
}

// TODO: Expand functionality of BoundingBox and interact_system() to include
// None, Hovered, and Pressed
impl BoundingBox {
    pub fn contains(&self, cursor_pos: Vec2) -> bool {
        self.lower.min(cursor_pos) == self.lower && self.upper.max(cursor_pos) == self.upper
    }

    /// Creates a new BoundingBox with `translation` reference frame instead of
    /// a `window frame` reference frame. This requires changing the points from
    /// the upper left and lower right to the lower left and upper right.
    pub fn to_trans(self) -> Self {
        let upper: Vec2 = window_to_trans(self.upper.x, self.upper.y - self.height).into();
        let lower: Vec2 = window_to_trans(self.lower.x, self.lower.y + self.height).into();
        BoundingBox {
            upper,
            lower,
            reference_frame: ReferenceFrame::Translation,
            height: TILE_SIZE,
        }
    }

    /// Creates a new BoundingBox with `translation` reference frame instead of
    /// a `window frame` reference frame. This requires changing the points from
    /// the lower left and upper right to the upper left and lower right.
    pub fn to_wf(self) -> Self {
        let upper: Vec2 = trans_to_window(self.upper.x, self.upper.y - self.height).into();
        let lower: Vec2 = trans_to_window(self.lower.x, self.lower.y + self.height).into();
        BoundingBox {
            upper,
            lower,
            reference_frame: ReferenceFrame::WindowFrame,
            height: TILE_SIZE,
        }
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
                interactable.focused = interactable.bound_wf.contains(cursor_pos);
                // **interacting_entity = entity;
            }
        }
    }
}
