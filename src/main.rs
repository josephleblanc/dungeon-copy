use bevy::window::{WindowResizeConstraints, WindowResolution};
use bevy::{prelude::*, window::WindowMode};

use config::*;
use plugins::debug::DebugPlugin;
use scenes::SceneState;

mod config;
mod materials;
mod plugins;
mod resources;
mod scenes;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(
                            WINDOW_HEIGHT * RESOLUTION,
                            WINDOW_HEIGHT,
                        ),
                        title: TITLE.to_string(),
                        position: WindowPosition::At(IVec2::new(
                            MONITOR_WIDTH / 4,
                            MONITOR_HEIGHT / 4,
                        )),
                        resizable: false,
                        resize_constraints: WindowResizeConstraints {
                            min_width: WINDOW_HEIGHT * RESOLUTION,
                            max_width: WINDOW_HEIGHT * RESOLUTION,
                            min_height: WINDOW_HEIGHT,
                            max_height: WINDOW_HEIGHT,
                        },
                        mode: WindowMode::Windowed,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .init_resource::<resources::setting::Setting>()
        .init_resource::<resources::dictionary::Dictionary>()
        .add_state::<scenes::SceneState>()
        .add_plugins(plugins::camera::CameraPlugin)
        .add_plugins(scenes::loading_scene::LoadingScenePlugin)
        .add_plugins(scenes::main_menu_scene::MainMenuScenePlugin)
        .add_systems(
            Update,
            debug_scene_state.run_if(state_changed::<SceneState>()),
        )
        .add_plugins(DebugPlugin)
        .run();
}

fn debug_scene_state(scene_state: Res<State<SceneState>>) {
    println!("scene:state: {:?}", scene_state.get());
}
