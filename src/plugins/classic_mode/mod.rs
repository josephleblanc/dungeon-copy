use bevy::prelude::*;

use crate::resources::game_data::PauseSceneData;
use crate::scenes::SceneState;

use super::input::movement::map::MapGrid;

pub mod dungeon;

pub struct ClassicModePlugin;

#[derive(Resource)]
/// Contains the entities which are used as the roots for spawning everything
/// in classic mode. These can be referenced elsewhere - for example, to
/// despawn all of their children recursively in cleanup.
pub struct ClassicModeData {
    pub doors: Option<Entity>,
    pub ground: Option<Entity>,
    pub walls: Option<Entity>,
    pub end_point: Option<Entity>,
}

impl Plugin for ClassicModePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(SceneState::PreClassicMode),
            dungeon::initiate::initiate_classic_mode,
        );

        app.add_systems(
            OnEnter(SceneState::InGameClassicMode),
            (
                dungeon::ground::ground,
                dungeon::doors::doors,
                dungeon::walls::walls,
                dungeon::end_point::end_point,
            ),
        );

        app.add_systems(
            Update,
            (
                dungeon::doors::horizontal_doors_system,
                dungeon::doors::vertical_doors_system,
                dungeon::walls::temporary_walls_system,
                dungeon::end_point::end_point_handle_system,
            )
                .run_if(
                    in_state(SceneState::InGameClassicMode)
                        .and_then(not(resource_exists::<PauseSceneData>())),
                ),
        );

        app.add_systems(OnExit(SceneState::InGameClassicMode), clean_up_classic_mode);
    }
}

fn clean_up_classic_mode(mut commands: Commands, classic_mode_data: Res<ClassicModeData>) {
    commands
        .entity(classic_mode_data.doors.unwrap())
        .despawn_recursive();

    commands
        .entity(classic_mode_data.walls.unwrap())
        .despawn_recursive();

    commands
        .entity(classic_mode_data.end_point.unwrap())
        .despawn_recursive();

    commands
        .entity(classic_mode_data.ground.unwrap())
        .despawn_recursive();

    commands.remove_resource::<MapGrid>();
}
