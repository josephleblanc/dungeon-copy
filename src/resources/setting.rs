use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::resources::language::Language;

#[derive(Resource, Serialize, Deserialize, Debug)]
pub struct Setting {
    language: Language,
    // more settings here, e.g. enable_sound and enable_music
}

impl Setting {
    pub fn new(/* settings options here, probably as bools */) -> Self {
        Setting {
            language: Language::EN,
            // more settings here
        }
    }

    pub fn get_language(&self) -> Language {
        self.language
    }

    pub fn set_language(&mut self, language: Language) {
        self.language = language;
    }
}

impl FromWorld for Setting {
    fn from_world(_world: &mut World) -> Self {
        let setting = Setting::new();
        setting
    }
}
