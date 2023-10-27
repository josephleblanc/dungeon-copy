use bevy::prelude::*;

use crate::components::player::PlayerComponent;
use crate::plugins::game_ui::map::MapUiData;
use crate::plugins::input::movement::Movement;
use crate::plugins::player::PlayerEntity;
use crate::{config::TILE_SIZE, plugins::input::movement::click_move::MovementPath};

#[derive(Resource, Clone, Copy)]
pub struct MovePathFrameData {
    pub frame_root: Entity,
}

#[derive(Event, Resource, Clone)]
pub struct PathSpriteEvent {
    pub frame_root: Option<Entity>,
    pub move_path: Option<MovementPath>,
    pub action: SpriteAction,
}

impl PathSpriteEvent {
    pub fn spawn_move_path(move_path: MovementPath) -> Self {
        Self {
            frame_root: None,
            action: SpriteAction::Spawn,
            move_path: Some(move_path),
        }
    }
}

impl From<SpriteAction> for PathSpriteEvent {
    fn from(value: SpriteAction) -> Self {
        Self {
            frame_root: None,
            move_path: None,
            action: value,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum SpriteAction {
    Spawn,
    Despawn,
}

#[derive(Clone, Copy, Component, Debug)]
pub struct PathSprite {
    pub step: (Vec3, Vec3),
}

impl PathSprite {
    fn contains(self, other: Vec2) -> bool {
        let debug = false;
        if debug {
            println!("self.step.0.y: {:?}, other.y: {:?}", self.step.0.y, other.y);
            println!(
                "(other.x - self.step.0.x).is_sign_positive(): {}",
                (other.x - self.step.0.x).is_sign_positive()
            );
            println!(
                "(other.x - self.step.1.x).is_sign_positive(): {}",
                (other.x - self.step.1.x).is_sign_positive()
            );
            println!("self.step.0.x: {:?}, other.x: {:?}", self.step.0.x, other.x);
            println!(
                "(other.y - self.step.0.y).is_sign_positive(): {}",
                (other.y - self.step.0.y).is_sign_positive()
            );
            println!(
                "(other.y - self.step.1.y).is_sign_positive(): {}",
                (other.y - self.step.1.y).is_sign_positive()
            );
        }
        let x_min = self.step.0.x.min(self.step.1.x) + TILE_SIZE / 4.0;
        let x_max = self.step.0.x.max(self.step.1.x) - TILE_SIZE / 4.0;
        let y_min = self.step.0.y.min(self.step.1.y) + TILE_SIZE / 4.0;
        let y_max = self.step.0.y.max(self.step.1.y) - TILE_SIZE / 4.0;
        if self.is_horizontal() {
            other.x.clamp(x_min, x_max) == other.x
        } else if self.is_vertical() {
            other.y.clamp(y_min, y_max) == other.y
        } else {
            false
        }
    }

    pub fn is_vertical(self) -> bool {
        self.step.0.x == self.step.1.x
    }

    pub fn is_horizontal(self) -> bool {
        self.step.0.y == self.step.1.y
    }
}

/// Create a Node for the whole window which will be used as reference for
/// placement of the focus box sprite.
pub fn setup(map_ui_data: Res<MapUiData>, mut commands: Commands) {
    let move_path_frame = commands
        .spawn(SpriteBundle {
            ..Default::default()
        })
        .set_parent(map_ui_data.map_ui_sprites_root)
        .id();

    commands.insert_resource::<MovePathFrameData>(MovePathFrameData {
        frame_root: move_path_frame,
    });
}

/// Spawns the a solid line running through the center of the squares along a
/// MovePath. The spawned rectangular sprites are all set to be the children
/// of Entity stored in the MovePathFrameData resource.
pub fn spawn_move_path(
    mut commands: Commands,
    old_path_sprites: Query<(Entity, &PathSprite)>,
    move_path_frame: Res<MovePathFrameData>,
    mut event_reader: EventReader<PathSpriteEvent>,
) {
    let debug = false;
    // if debug {
    //     println!("debug | spawn_move_path | start");
    // }
    let Some(move_path) = event_reader
        .into_iter()
        .filter(|event| event.action == SpriteAction::Spawn)
        .find_map(|event| event.move_path.clone())
    else {
        return;
    };
    let offset: Vec3 = Vec3::new(TILE_SIZE / 2.0, TILE_SIZE / 2.0, 1.20);

    if debug && old_path_sprites.is_empty() {
        println!("old path sprites empty");
    }

    for (old_entity, old_step) in old_path_sprites
        .iter()
        .inspect(|(old_entity, old_step)| {
            if debug {
                println!("old step: {:?}", old_step)
            }
        })
        .filter(|(_, old)| !move_path.path.as_slice().contains(&old.step))
    {
        if debug {
            println!("despawning {:?}", old_step);
        }
        commands.entity(old_entity).despawn();
    }

    for step in move_path
        .path
        .iter()
        .inspect(|new_step| {
            if debug {
                println!("new_step: {:?}", new_step);
            }
        })
        .filter(|new_step| {
            !old_path_sprites
                .iter()
                .any(|(_, old_sprite)| (**new_step == old_sprite.step))
                || old_path_sprites.is_empty()
        })
    {
        let is_horizontal = step.0.x + TILE_SIZE == step.1.x || step.0.x - TILE_SIZE == step.1.x;
        let is_vertical = step.0.y + TILE_SIZE == step.1.y || step.0.y - TILE_SIZE == step.1.y;
        if is_horizontal {
            let signed_offset_x = if (step.1.x - step.0.x).is_sign_positive() {
                offset.x
            } else {
                -1.0 * offset.x
            };
            let transform_x = step.0.x + signed_offset_x;
            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        // TODO: Put this color in a const somewhere
                        color: Color::rgb(0.25, 0.25, 0.75),
                        custom_size: Some(Vec2::new(TILE_SIZE, 5.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(
                        transform_x,
                        step.0.y,
                        offset.z,
                    )),
                    ..default()
                })
                .insert(PathSprite { step: *step })
                .set_parent(move_path_frame.frame_root);
            if debug {
                println!("spawning step {:?}", step);
            }
        } else if is_vertical {
            let signed_offset_y = if (step.1.y - step.0.y).is_sign_positive() {
                offset.y
            } else {
                -1.0 * offset.y
            };
            let transform_y = step.0.y + signed_offset_y;
            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        // TODO: Put this color in a const somewhere
                        color: Color::rgb(0.25, 0.25, 0.0),
                        custom_size: Some(Vec2::new(5.0, TILE_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(
                        step.0.x,
                        transform_y,
                        offset.z,
                    )),
                    ..default()
                })
                .insert(PathSprite { step: *step })
                .set_parent(move_path_frame.frame_root);
            if debug {
                println!("spawning step {:?}", step);
            }
        }
    }
}

pub fn despawn_on_move(
    player_pos: Query<&Transform, With<PlayerComponent>>,
    path_sprites: Query<(Entity, &PathSprite)>,
    movement: Res<Movement>,
    mut commands: Commands,
) {
    let debug = false;
    if movement.moving {
        let player_pos = player_pos.get_single().unwrap().translation;
        for (entity, step) in path_sprites
            .iter()
            .filter(|(_, step)| step.contains(player_pos.truncate()))
            .inspect(|(_, step)| {
                if debug {
                    println!(
                        "spawn_on_move | step.contains: {}",
                        step.contains(player_pos.truncate())
                    )
                }
            })
        {
            commands.entity(entity).despawn();
            if debug {
                println!("spawn_on_move | spawning step {:?}", step);
            }
        }
    }
}

// Despawns a move path by recursively despawning the children of the Entity
// stored in the resource `Res<MovePathFrameData>`.
// The resource itself is despawned in the cleanup function of the containing
// map crate.
// pub fn despawn_move_path(
//     mut commands: Commands,
//     move_path_frame: Res<MovePathFrameData>,
//     mut event_reader: EventReader<PathSpriteEvent>,
// ) {
//     if event_reader
//         .into_iter()
//         .any(|event| event.action == SpriteAction::Despawn)
//     {
//         commands
//             .entity(move_path_frame.frame_root)
//             .despawn_descendants();
//     }
// }
