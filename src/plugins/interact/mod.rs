// Inspired by https://github.com/Anshorei/bevy_rei/tree/master

#![allow(dead_code)]

use crate::config::TILE_SIZE;
use bevy::prelude::*;
use std::ops::Deref;
use std::ops::DerefMut;
use std::slice::Iter;

use crate::scenes::SceneState;

use crate::plugins::game_ui::translate::trans_to_window;
use crate::plugins::game_ui::translate::window_to_trans;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InteractingPos>();
        app.add_systems(
            Update,
            interact_system.run_if(in_state(SceneState::InGameClassicMode)),
        );
        app.add_systems(OnExit(SceneState::InGameClassicMode), cleanup);
    }
}

#[derive(Component, Reflect, Debug, Copy, Clone)]
pub enum ReferenceFrame {
    WindowFrame,
    Translation,
}

#[derive(Resource, Default, Reflect)]
pub struct InteractingPos {
    pub pos: Vec2,
    pub active: bool,
    pub interacting_type: InteractingType,
    pub entity: Option<Entity>,
}

#[derive(Component, Reflect, Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum InteractingType {
    #[default]
    None,
    MapGrid,
    Enemy,
    PlayerCharacter,
}

impl InteractingType {
    pub fn iterator() -> Iter<'static, Self> {
        use InteractingType::*;
        [Enemy, MapGrid, PlayerCharacter].iter()
    }
}

// impl PartialOrd for InteractingType {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         let mut self_number = 0;
//         let mut other_number = 0;
//         for (order_number, i_type) in InteractingType::iterator().enumerate() {
//             if i_type == self {
//                 self_number = order_number;
//             }
//             if i_type == other {
//                 other_number = order_number;
//             }
//         }
//         Some(self_number.cmp(&other_number))
//     }
// }

impl Deref for InteractingPos {
    type Target = Vec2;
    fn deref(&self) -> &Self::Target {
        &self.pos
    }
}

impl DerefMut for InteractingPos {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pos
    }
}

impl<T> AsRef<T> for InteractingPos
where
    T: ?Sized,
    <InteractingPos as Deref>::Target: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.deref().as_ref()
    }
}

// impl From<Vec2> for InteractingPos {
//     fn from(value: Vec2) -> Self {
//         Self {
//             pos: value,
//             active: false,
//         }
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
    /// The center position of the entity being interacted with.
    /// The z-position is not included.
    pub center: Vec2,
    /// Whether or not the entity is currently interactable.
    pub active: bool,
    /// Whether or not the entity is currently being focused by the
    /// cursor.
    pub focused: bool,
    /// The type of thing to be interacted with, e.g. Enemy, Gridsquare, etc.
    pub interacting_type: InteractingType,
}

impl Interactable {
    /// constructs a new Interactable struct from a lower and upper bound.
    /// This `from_trans` version has bounding box points in the lower left
    /// and upper right corner of the box.
    pub fn new_from_trans(
        lower: Vec2,
        upper: Vec2,
        interacting_type: InteractingType,
    ) -> Interactable {
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
            // TODO: Consider putting this into the arguments instead.
            center: lower + (upper - lower) / 2.0,
            interacting_type,
        }
    }

    /// constructs a new Interactable struct from a lower and upper bound.
    /// This `from_trans` version has bounding box points in the upper left
    /// and lower right corner of the box.
    pub fn new_from_window(
        lower: Vec2,
        upper: Vec2,
        interacting_type: InteractingType,
    ) -> Interactable {
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
            center: lower + (upper - lower) / 2.0,
            interacting_type,
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

#[derive(Event, Clone, Copy, Debug)]
pub struct InteractingPosEvent {
    pub pos: Vec2,
    pub interacting_type: InteractingType,
    pub entity: Option<Entity>,
}

impl InteractingPosEvent {
    pub fn new(pos: Vec2, interacting_type: InteractingType, entity: Option<Entity>) -> Self {
        Self {
            pos,
            interacting_type,
            entity,
        }
    }
}

// impl From<InteractingPos> for InteractingPosEvent {
//     fn from(value: InteractingPos) -> Self {
//         Self {
//             pos: value.pos,
//             interacting_type: value.interacting_type,
//         }
//     }
// }

impl From<Vec2> for InteractingPosEvent {
    fn from(value: Vec2) -> Self {
        Self {
            pos: value,
            interacting_type: InteractingType::None,
            entity: None,
        }
    }
}

pub fn interact_system(
    window_query: Query<&Window>,
    mut interactable_query: Query<(Entity, &mut Interactable)>,
    mut interacting_pos: ResMut<InteractingPos>,
    mut event_writer: EventWriter<InteractingPosEvent>,
) {
    let debug = false;
    for window in window_query.iter() {
        if let Some(cursor_pos) = window.cursor_position() {
            let mut highest_priority_event: Option<InteractingPosEvent> = None;
            for (entity, mut interactable) in interactable_query.iter_mut() {
                let is_interacting = interactable.bound_wf.contains(cursor_pos);
                interactable.focused = is_interacting;

                if is_interacting
                    && (interacting_pos.entity.is_none()
                        || interacting_pos.entity.unwrap() != entity)
                {
                    // interacting_pos.pos = interactable.center;
                    // TODO: make sure this is actually doing something.
                    interacting_pos.active = true;
                    ////////////////////////////////////////////////////
                    if highest_priority_event.is_some() {
                        if highest_priority_event.unwrap().interacting_type
                            < interactable.interacting_type
                        {
                            highest_priority_event = Some(InteractingPosEvent::new(
                                interactable.center,
                                // interacting_pos.pos,
                                interactable.interacting_type,
                                Some(entity),
                            ));
                        }
                    } else {
                        highest_priority_event = Some(InteractingPosEvent::new(
                            interactable.center,
                            // interacting_pos.pos,
                            interactable.interacting_type,
                            Some(entity),
                        ));
                    }
                }
            }
            if let Some(event) = highest_priority_event {
                if event.pos != interacting_pos.pos {
                    if debug {
                        println!(
                        "interact | interact_system | sending event with interacting_type: \n{:#?}",
                        event.interacting_type
                    );
                    }
                    // println!(
                    //     "event.pos: {}, interacting_pos.pos: {}",
                    //     event.pos, interacting_pos.pos
                    // );
                    interacting_pos.pos = event.pos;
                    interacting_pos.interacting_type = event.interacting_type;
                    interacting_pos.entity = event.entity;
                    event_writer.send(event);
                }
            }
        }
    }
}

// pub fn interacting_entity(
//     mut enemy_query: Query<(Entity, &Interactable)>,
//     mut interacting_pos: ResMut<InteractingPos>,
//     mut event_writer: EventWriter<InteractingPosEvent>,
// ) {
//     for event in event_writer.iter() {
//         if event.interacting_type == InteractingType::Enemy {
//             if let Some(entity, interactable) = enemy_query.get(event.)
//         }
//     }
// }

fn cleanup(mut commands: Commands) {
    commands.remove_resource::<InteractingPos>();
}
