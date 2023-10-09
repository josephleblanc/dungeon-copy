use crate::config::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

// use crate::config::*;
use crate::resources::language::Language;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Glossery {
    pub main_menu_scene_text: MainMenuSceneText,
    pub loading_scene_text: LoadingSceneText,
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

impl Glossery {
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
                let glossery = serde_json::from_str(&contents).unwrap();
                // I prefer the default error message over the one constructed here.
                // .unwrap_or_else(|_| panic!("{}", error_message));
                glossery
            }
            Err(err) => panic!("Can't find language file: {}", err),
        }
    }
}
