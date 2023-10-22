use bevy::prelude::*;

use crate::config::{RESOLUTION, TILE_SIZE, WINDOW_HEIGHT};
use crate::materials::ingame::InGameMaterials;
use crate::plugins::interact::Interactable;
use crate::resources::dungeon::grid_square::GridSquare;

#[derive(Resource)]
pub struct MapUiData {
    user_interface_root: Entity,
    pub map_ui_sprites_root: Entity,
}

#[derive(Component)]
pub struct FocusBox {
    hide_pos: Vec3,
    display_z: f32,
}

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

pub fn setup(mut commands: Commands, ingame_materials: Res<InGameMaterials>) {
    let user_interface_root = commands
        .spawn((
            NodeBundle {
                background_color: Color::NONE.into(),
                ..Default::default()
            },
            Name::new("Map Focus Box Root Node"),
        ))
        .id();
    // .with_children(|builder| {
    //     build_focus_box(builder);
    // })

    let map_ui_sprites_root = commands
        .spawn(SpriteBundle {
            ..Default::default()
        })
        .with_children(|builder| {
            build_focus_box(builder, ingame_materials);
        })
        .id();

    commands.insert_resource(MapUiData {
        user_interface_root,
        map_ui_sprites_root,
    });
}

pub fn build_focus_box(builder: &mut ChildBuilder, ingame_materials: Res<InGameMaterials>) {
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

pub fn cleanup(mut commands: Commands, map_ui_data: Res<MapUiData>) {
    commands
        .entity(map_ui_data.user_interface_root)
        .despawn_recursive();
    commands
        .entity(map_ui_data.map_ui_sprites_root)
        .despawn_recursive();

    commands.remove_resource::<MapUiData>();
}
