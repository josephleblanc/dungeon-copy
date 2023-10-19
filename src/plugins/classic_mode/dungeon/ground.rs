use bevy::prelude::*;

use crate::config::*;
use crate::materials::ingame::InGameMaterials;
use crate::plugins::classic_mode::dungeon::{TOTAL_TILE_HEIGHT, TOTAL_TILE_WIDTH};
use crate::plugins::classic_mode::ClassicModeData;
use crate::plugins::interact::Interactable;
use crate::resources::dungeon::grid_square::GridSquare;
use crate::resources::dungeon::ground::Ground;
use crate::resources::dungeon::layer::Layer;

pub fn ground(
    mut commands: Commands,
    ingame_materials: Res<InGameMaterials>,
    mut data: ResMut<ClassicModeData>,
) {
    let tile_offset = TILE_SIZE / 2.0;
    let start_y: f32 = 0.0 + WINDOW_HEIGHT / 2.0 - tile_offset;
    let start_x: f32 = 0.0 - WINDOW_HEIGHT * RESOLUTION / 2.0 + tile_offset;

    let ground = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(WINDOW_HEIGHT * RESOLUTION, WINDOW_HEIGHT)),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            for row_index in 0..TOTAL_TILE_HEIGHT {
                for column_index in 0..TOTAL_TILE_WIDTH {
                    if row_index >= 1 && column_index > 0 && column_index < 15 {
                        // let offset_x = column_index as f32 * TILE_SIZE;
                        // let offset_y = row_index as f32 * TILE_SIZE;
                        let x = start_x + column_index as f32 * TILE_SIZE;
                        let y = start_y - row_index as f32 * TILE_SIZE;

                        let box_lower_tr: Vec2 = Vec2::new(x - tile_offset, y - tile_offset);
                        let box_upper_tr: Vec2 = Vec2::new(x + tile_offset, y + tile_offset);

                        parent
                            .spawn(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                                    ..Default::default()
                                },
                                transform: Transform {
                                    translation: Vec3::new(x, y, 0.0),
                                    ..Default::default()
                                },
                                texture: ingame_materials.dungeon_materials.floor.clone(),
                                ..Default::default()
                            })
                            // adds a blue dot at tile position for debugging
                            .with_children(|builder| {
                                builder.spawn(SpriteBundle {
                                    sprite: Sprite {
                                        color: Color::rgb(1.0, 0.25, 0.75),
                                        custom_size: Some(Vec2::new(10.0, 10.0)),
                                        ..default()
                                    },
                                    transform: Transform::from_translation(Vec3::new(0., 0., 10.)),
                                    ..default()
                                });
                            })
                            .insert(Layer)
                            .insert(GridSquare)
                            .insert(Interactable::new_from_trans(box_lower_tr, box_upper_tr))
                            .insert(Name::new(format!("Layer ({}, {})", x, y)));
                    }
                }
            }
        })
        .insert(Name::new("Ground"))
        .insert(Ground)
        .id();

    data.ground = Some(ground);
}
