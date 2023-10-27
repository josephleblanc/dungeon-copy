use bevy::prelude::*;

use crate::materials::ingame::InGameMaterials;

pub mod focus_box;
pub mod pathing;

#[derive(Resource)]
pub struct MapUiData {
    user_interface_root: Entity,
    pub map_ui_sprites_root: Entity,
}

/// Create a Node for the whole window which will be used as reference for
/// placement of the focus box sprite.
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
            focus_box::setup(builder, ingame_materials);
        })
        .id();

    commands.insert_resource(MapUiData {
        user_interface_root,
        map_ui_sprites_root,
    });
}

/// Cleans up both the Node frame (w/ children) for the map sprite as well
/// as any other UI elements attached to the user_interface_root, which
/// are UI, rather than sprite, components.
pub fn cleanup(mut commands: Commands, map_ui_data: Res<MapUiData>) {
    commands
        .entity(map_ui_data.user_interface_root)
        .despawn_recursive();
    commands
        .entity(map_ui_data.map_ui_sprites_root)
        .despawn_recursive();

    commands.remove_resource::<MapUiData>();
    commands.remove_resource::<pathing::MovePathFrameData>();
}
