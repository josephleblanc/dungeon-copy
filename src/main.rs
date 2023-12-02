#![allow(
    clippy::collapsible_else_if,
    clippy::type_complexity,
    clippy::too_many_arguments
)]

use bevy::window::{WindowResizeConstraints, WindowResolution};
use bevy::{prelude::*, window::WindowMode};

use config::*;
use plugins::debug::DebugPlugin;

mod components;
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
        .add_plugins(scenes::game_mode_select::GameModeSelectScenePlugin)
        .add_plugins(scenes::hero_select_scene::HeroSelectScenePlugin)
        .add_plugins(plugins::classic_mode::ClassicModePlugin)
        .add_plugins(plugins::player::PlayerPlugin)
        .add_plugins(plugins::input::InputHandlePlugin)
        .add_plugins(plugins::game_ui::IngameUiPlugin)
        .add_plugins(plugins::interact::InteractionPlugin)
        .add_plugins(plugins::monster::MonsterPlugin)
        .add_plugins(plugins::combat::CombatPlugin)
        .add_plugins(plugins::combat_mode::CombatModePlugin)
        .add_plugins(plugins::actions::ActionPlugin)
        .add_plugins(DebugPlugin)
        .run();
}
