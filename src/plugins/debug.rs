use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::scenes::SceneState;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugins(WorldInspectorPlugin::new());
            app.add_systems(
                Update,
                print_scene.run_if(resource_changed::<State<SceneState>>()),
            );
        }
    }
}

fn print_scene(scene: Res<State<SceneState>>) {
    println!("debug plugin | scene: {:?}", scene);
}
