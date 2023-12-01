use bevy::prelude::*;

use crate::config::{RESOLUTION, TILE_SIZE, WINDOW_HEIGHT};
use crate::materials::ingame::InGameMaterials;
use crate::plugins::interact::Interactable;
use crate::resources::dungeon::grid_square::GridSquare;

#[derive(Component)]
pub struct FocusBox {
    hide_pos: Vec3,
    display_z: f32,
}

/// Manages the position of the grid focus sprite.
/// If there is an entity with both GridSquare and Interactable, the focus box
/// will be moved to that position when the mouse hovers over it.
/// This is in part managed through the Interactable component, which keeps track
/// of whether the mouse is currently hovering over the area assigned to that
/// grid square, and in part by the FocusBox component, which holds the position
/// where the focus box sprite should be placed when no Interactable is being
/// hovered.
pub fn mouse_handle_system(
    query_grid: Query<
        (&Transform, &Interactable),
        (With<GridSquare>, Changed<Interactable>, Without<FocusBox>),
    >,
    mut query_focus_box: Query<(&mut Transform, &FocusBox)>,
) {
    if let Some((interact_transform, interactable)) = query_grid
        .iter()
        .find(|(_, interactable)| interactable.focused)
    {
        if interactable.active {
            let (mut focus_box_transform, focus_box) = query_focus_box.get_single_mut().unwrap();
            focus_box_transform.translation = (
                interact_transform.translation.x,
                interact_transform.translation.y,
                focus_box.display_z,
            )
                .into();
        }
    } else {
        let (mut focus_box_transform, focus_box) = query_focus_box.get_single_mut().unwrap();
        if focus_box_transform.translation != focus_box.hide_pos {
            focus_box_transform.translation = focus_box.hide_pos;
        }
    }
}

/// Initialize the grid focus box sprite.
/// It is important to note that the z position of the box must be placed so that
/// it appears above the grid square and below anything placed on the map sprite,
/// for example the player.
/// E.g. If the player is on z-position 1.5, and the grid square is on 1.0,
/// then the focus box should have a `z_layer` value so that
/// 1.0 < z_layer < 1.5
pub fn setup(builder: &mut ChildBuilder, ingame_materials: Res<InGameMaterials>) {
    let x_pos = 0.0;
    let y_pos = 0.0;
    // TODO: Make consts for the z_layers and keep them in an organized list
    // somewhere.
    // The z_layer should be between the player/monster sprites above and the
    // map tiles below.
    let z_layer = 0.125;

    let hidden_offscreen = Vec3::new(
        -1.0 * (WINDOW_HEIGHT * RESOLUTION / 2.0),
        -1.0 * (WINDOW_HEIGHT / 2.0),
        z_layer,
    );
    builder.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(x_pos, y_pos, z_layer),
                ..Default::default()
            },
            texture: ingame_materials.map_ui.grid_select_box.clone(),
            ..Default::default()
        },
        FocusBox {
            hide_pos: hidden_offscreen,
            display_z: z_layer,
        },
        Name::new("Grid Focus Box"),
    ));
}
