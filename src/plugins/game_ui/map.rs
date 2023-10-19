use bevy::prelude::*;

use crate::config::TILE_SIZE;
use crate::materials::ingame::InGameMaterials;
use crate::plugins::interact::Interactable;
use crate::resources::dungeon::grid_square::GridSquare;

#[derive(Resource)]
pub struct MapUiData {
    user_interface_root: Entity,
    pub map_ui_sprites_root: Entity,
}

#[derive(Component)]
pub struct FocusBox;

pub fn mouse_handle_system(
    query_grid: Query<
        (&Transform, &Interactable),
        (With<GridSquare>, Changed<Interactable>, Without<FocusBox>),
    >,
    mut query_focus_box: Query<&mut Transform, With<FocusBox>>,
) {
    for (interact_transform, interactable) in query_grid.iter() {
        if interactable.focused && interactable.active {
            let mut focus_box_transform = query_focus_box.get_single_mut().unwrap();
            focus_box_transform.translation.x = interact_transform.translation.x;
            focus_box_transform.translation.y = interact_transform.translation.y;
        }
    }
}

pub fn setup(mut commands: Commands, ingame_materials: Res<InGameMaterials>) {
    let user_interface_root = commands
        .spawn(NodeBundle {
            background_color: Color::NONE.into(),
            ..Default::default()
        })
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
        FocusBox,
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
