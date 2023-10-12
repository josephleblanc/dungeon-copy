use bevy::prelude::*;

use crate::config::*;
use crate::materials::ingame::InGameMaterials;
use crate::plugins::classic_mode::ClassicModeData;
use crate::resources::dungeon::door::{Door, HorizontalDoor, VerticaltDoor};
use crate::resources::dungeon::doors::Doors;
use crate::resources::dungeon::Dungeon;
use crate::resources::player::player_dungeon_stats::PlayerDungeonStats;

const START_Y: f32 = 0.0 + WINDOW_HEIGHT / 2.0 - TILE_SIZE / 2.0;
const START_X: f32 = 0.0 - WINDOW_HEIGHT * RESOLUTION / 2.0 + TILE_SIZE / 2.0;

pub fn doors(
    mut commands: Commands,
    ingame_materials: Res<InGameMaterials>,
    mut data: ResMut<ClassicModeData>,
) {
    let doors = commands
        .spawn(SpriteBundle {
            ..Default::default()
        })
        .with_children(|parent| {
            for door in Door::iterator() {
                match door {
                    Door::Left | Door::Right => horizontal_door(parent, door, &ingame_materials),
                    Door::Bottom | Door::Top => vertical_door(parent, door, &ingame_materials),
                }
            }
        })
        .insert(Doors)
        .insert(Name::new("Doors"))
        .id();

    data.doors = Some(doors);
}

pub fn horizontal_door(parent: &mut ChildBuilder, door: &Door, ingame_materials: &InGameMaterials) {
    // TODO: change if/else to match
    let image = if *door == Door::Left {
        ingame_materials.dungeon_materials.wall_border_left.clone()
    } else {
        ingame_materials.dungeon_materials.wall_border_right.clone()
    };

    // TODO: change if/else to match
    let x = if *door == Door::Left {
        START_X
    } else {
        START_X + 15.0 * TILE_SIZE
    };

    let y = START_Y - 4.0 * TILE_SIZE;

    // TODO: change if/else to match
    let component_name = if *door == Door::Left {
        "Left Door"
    } else {
        "Right Door"
    };

    parent
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(x, y, 0.2),
                ..Default::default()
            },
            texture: image,
            ..Default::default()
        })
        .insert(Name::new(component_name))
        .insert(HorizontalDoor)
        .insert(door.clone());
}

pub fn vertical_door(
    grandparent: &mut ChildBuilder,
    door: &Door,
    ingame_materials: &InGameMaterials,
) {
    let left_part = ingame_materials.dungeon_materials.door_left_part.clone();
    let right_part = ingame_materials.dungeon_materials.door_right_part.clone();
    let door_closed = ingame_materials.dungeon_materials.door_closed.clone();

    let left_door_part_x = -96.0;
    let right_door_part_x = 96.0;

    let y = if *door == Door::Bottom { -224.0 } else { 224.0 };
    let z = if *door == Door::Bottom { 0.2 } else { 0.1 };

    let component_name = if *door == Door::Bottom {
        "Bottom Door"
    } else {
        "Top Door"
    };

    let vertical_door_type = if *door == Door::Bottom {
        VerticaltDoor::Bottom
    } else {
        VerticaltDoor::Top
    };

    grandparent
        .spawn(SpriteBundle {
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE * 2.0)),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(left_door_part_x, y, z),
                        ..Default::default()
                    },
                    texture: left_part,
                    ..Default::default()
                })
                .insert(Name::new("Left Verticalt Door Part"));

            parent
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(TILE_SIZE * 2.0, TILE_SIZE * 2.0)),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(0.0, y, z),
                        ..Default::default()
                    },
                    texture: door_closed,
                    ..Default::default()
                })
                .insert(Name::new("Main Verticalt Door Part"))
                .insert(door.clone());

            parent
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE * 2.0)),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(right_door_part_x, y, z),
                        ..Default::default()
                    },
                    texture: right_part,
                    ..Default::default()
                })
                .insert(Name::new("Left Verticalt Door Part"));
        })
        .insert(Name::new(component_name))
        .insert(vertical_door_type);
}

pub fn horizontal_doors_system(
    mut horizontal_door_query: Query<(&HorizontalDoor, &Door, &mut Visibility)>,
    player_dungeon_stats: Res<PlayerDungeonStats>,
    dungeon: Res<Dungeon>,
) {
    if player_dungeon_stats.is_changed() {
        for (_horizontal_door, door, mut visibility) in horizontal_door_query.iter_mut() {
            if !player_dungeon_stats.is_room_cleared {
                *visibility = Visibility::Visible;
            } else {
                let current_floor = dungeon.current_floor.clone();
                let current_position = current_floor.current_position;

                let total_columns = current_floor.total_columns;

                let has_right_room = if current_position.column_index < total_columns - 1 {
                    let right_room_column_index = current_position.column_index + 1;
                    current_floor.map[current_position.row_index][right_room_column_index] != 0.0
                } else {
                    false
                };

                let has_left_room = if current_position.column_index > 0 {
                    let left_room_column_index = current_position.column_index - 1;
                    current_floor.map[current_position.row_index][left_room_column_index] != 0.0
                } else {
                    false
                };

                *visibility = if *door == Door::Right {
                    if has_right_room {
                        Visibility::Hidden
                    } else {
                        Visibility::Inherited
                    }
                } else {
                    if has_left_room {
                        Visibility::Hidden
                    } else {
                        Visibility::Inherited
                    }
                }
            }
        }
    }
}

/// Manages the open/close state of doors and whether or not they are visible.
/// Doors are opened when a room is cleared, closed otherwise.
/// Doors are visible if there is a next room in that direction (up/down,left/right),
/// hidden otherwise.
pub fn vertical_doors_system(
    mut vertical_door_query: Query<(&VerticaltDoor, &Children, &mut Visibility)>,
    mut visibility_query: Query<&mut Visibility, Without<VerticaltDoor>>,
    mut image_query: Query<(&Door, &mut Handle<Image>)>,
    player_dungeon_stats: Res<PlayerDungeonStats>,
    ingame_materials: Res<InGameMaterials>,
    dungeon: Res<Dungeon>,
) {
    if player_dungeon_stats.is_changed() {
        for (vertical_door, children, mut visibility) in vertical_door_query.iter_mut() {
            let current_floor = dungeon.current_floor.clone();
            let current_position = current_floor.current_position;
            let total_rows = current_floor.total_rows;

            // Checks whether there are rooms above or below the current room.
            let has_next_room = if *vertical_door == VerticaltDoor::Top {
                if current_position.row_index > 0 {
                    let above_room_row_index = current_position.row_index - 1;
                    current_floor.map[above_room_row_index][current_position.column_index] != 0.0
                } else {
                    false
                }
            } else if current_position.row_index < total_rows - 1 {
                let below_room_row_index = current_position.row_index + 1;
                current_floor.map[below_room_row_index][current_position.column_index] != 0.0
            } else {
                false
            };

            // hides the doors if there is no next room.
            *visibility = if has_next_room {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };

            // the children of VerticalDoor are their images
            for child in children.iter() {
                let mut child_visibility = visibility_query.get_mut(*child).unwrap();
                *child_visibility = if has_next_room {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };
            }

            if has_next_room {
                let is_room_cleared = player_dungeon_stats.is_room_cleared;
                for child in children.iter() {
                    let result = image_query.get_mut(*child);
                    // TODO: change `if let Ok(result)`
                    if result.is_ok() {
                        let (_door, mut texture) = result.unwrap();
                        *texture = if is_room_cleared {
                            ingame_materials.dungeon_materials.door_opened.clone()
                        } else {
                            ingame_materials.dungeon_materials.door_closed.clone()
                        }
                    }
                }
            }
        }
    }
}
