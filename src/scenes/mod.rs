use bevy::prelude::States;

pub mod loading_scene;
pub mod main_menu_scene;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
pub enum SceneState {
    #[default]
    LoadingScene,
    MainMenuScene,
}
