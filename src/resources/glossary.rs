use crate::config::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

// use crate::config::*;
use crate::resources::language::Language;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Glossary {
    pub main_menu_scene_text: MainMenuSceneText,
    pub loading_scene_text: LoadingSceneText,
    pub shared_text: SharedText,
    pub movement_mode: MovementModeText,
    pub combat_mode: CombatModeText,
    pub turn_action_display: TurnActionDisplay,
    pub action_bar: ActionBar,
    pub attack_submenu: AttackSubMenu,
    pub move_submenu: MoveSubMenu,
}

/// This trait is for enums which have a corresponding translation, and is
/// mostly used for buttons.
pub trait Translation {
    fn to_string_glossary(self, glossary: &Glossary) -> String;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MoveSubMenu {
    pub move_action: String,
    pub standard_action: String,
    pub full_move: String,
    pub five_foot_step: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AttackSubMenu {
    pub single_attack: String,
    pub full_attack: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActionBar {
    pub attack: String,
    pub move_action: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TurnActionDisplay {
    pub move_action: String,
    pub standard_action: String,
    pub immediate_action: String,
    pub five_foot_step: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CombatModeText {
    pub in_combat: String,
    pub out_of_combat: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MainMenuSceneText {
    pub play: String,
    pub options: String,
    pub quit: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoadingSceneText {
    pub loading: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SharedText {
    pub classic_mode: String,
    pub survival_mode: String,
    pub select_game_mode: String,
    pub select_hero: String,
    pub continue_: String,
    pub quit: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MovementModeText {
    pub wander_movement: String,
    pub turn_based_movement: String,
}

impl Glossary {
    pub fn new(language: Language) -> Self {
        let file_name = match language {
            Language::EN => ENGLISH_LANGUAGE_FILE,
            // more languages here
        };

        match File::open(file_name) {
            Ok(mut file) => {
                // I prefer the default error message over the one constructed here.
                // let error_message = format!(
                //     "{}: JSON was not well-formatted",
                //     if language == Language::EN {
                //         "Language::EN"
                //     } else {
                //         // other languages here
                //         "No Other Languages"
                //     }
                // );

                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                let Glossary = serde_json::from_str(&contents).unwrap();
                // I prefer the default error message over the one constructed here.
                // .unwrap_or_else(|_| panic!("{}", error_message));
                Glossary
            }
            Err(err) => panic!("Can't find language file: {}", err),
        }
    }
}
