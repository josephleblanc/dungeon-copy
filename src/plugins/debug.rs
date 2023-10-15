use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::plugins::game_ui::turn_mode::MovementModeRes;
use crate::scenes::SceneState;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugins(WorldInspectorPlugin::new());
            app.add_systems(
                Update,
                (
                    debug_scene.run_if(resource_changed::<State<SceneState>>()),
                    debug_movement_mode.run_if(
                        resource_exists::<MovementModeRes>()
                            .and_then(resource_changed::<MovementModeRes>()),
                    ),
                ),
            );
        }
    }
}

fn debug_scene(scene: Res<State<SceneState>>) {
    println!("debug plugin | scene: {:?}", scene);
}

fn debug_movement_mode(movement_mode: Res<MovementModeRes>) {
    println!("debug plugin | MovementMode {:?}", movement_mode);
}
