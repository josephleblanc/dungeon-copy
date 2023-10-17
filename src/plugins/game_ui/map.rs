use bevy::prelude::*;

use crate::config::{RESOLUTION, TILE_SIZE, WINDOW_HEIGHT};
use crate::plugins::interact::Interactable;
use crate::resources::dungeon::grid_square::GridSquare;

use super::cartesian_to_ui;

#[derive(Resource)]
pub struct MapUiData {
    user_interface_root: Entity,
}

#[derive(Component)]
pub struct FocusBox;

pub fn mouse_handle_system(
    query_grid: Query<(&Transform, &Interactable), (With<GridSquare>, Changed<Interactable>)>,
    mut query_focus_box: Query<&mut Style, With<FocusBox>>,
    query_window: Query<&Window>,
) {
    for (transform, interactable) in query_grid.iter() {
        if interactable.focused && interactable.active {
            if let Some(cursor_position) = query_window.get_single().unwrap().cursor_position() {
                // if interactable.bounding_box.contains(cursor_position) {
                let mut focus_box_style = query_focus_box.get_single_mut().unwrap();
                let (left_pos, bottom) =
                    cartesian_to_ui(transform.translation.x, transform.translation.y);
                // let x_offset = focus_box_style.width / 2.0;
                // let y_offset = focus_box_style.height / 2.0;
                focus_box_style.left = Val::Px(left_pos);
                focus_box_style.bottom = Val::Px(bottom);
                println!(
                    "debug | focused transform.x: {}, transform.y: {}",
                    transform.translation.x, transform.translation.y
                );
                println!(
                    "debug | box left: {:?}, bottom: {:?}",
                    focus_box_style.left, focus_box_style.bottom
                );
                println!(
                    "debug | bounding_box lower: {:?}, bounding_box upper: {:?}",
                    interactable.bounding_box.lower, interactable.bounding_box.upper
                );
                // }
            }
        }
    }
}

pub fn setup(mut commands: Commands) {
    let user_interface_root = commands
        .spawn(NodeBundle {
            background_color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|builder| {
            build_focus_box(builder);
        })
        .id();

    commands.insert_resource(MapUiData {
        user_interface_root,
    });
}

pub fn build_focus_box(builder: &mut ChildBuilder) {
    let x_pos = 0.0;
    let y_pos = 0.0;
    builder.spawn((
        NodeBundle {
            style: Style {
                border: UiRect::all(Val::Px(5.0)),
                width: Val::Px(TILE_SIZE),
                height: Val::Px(TILE_SIZE),
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            border_color: Color::GOLD.into(),
            ..Default::default()
        },
        FocusBox,
    ));
}

pub fn cleanup(map_ui_data: Res<MapUiData>, mut commands: Commands) {
    commands
        .entity(map_ui_data.user_interface_root)
        .despawn_recursive();

    commands.remove_resource::<MapUiData>();
}
