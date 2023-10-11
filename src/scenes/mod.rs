use bevy::prelude::States;

pub mod game_mode_select;
pub mod hero_select_scene;
pub mod loading_scene;
pub mod main_menu_scene;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
pub enum SceneState {
    #[default]
    LoadingScene,
    MainMenuScene,
    GameModeSelectScene,
    HeroSelectScene,
    PreClassicMode,
}
