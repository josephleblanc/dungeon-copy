use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::resources::glossery::Glossery;
use crate::resources::language::Language;
use crate::resources::setting::Setting;

#[derive(Resource, Serialize, Deserialize, Debug, Clone)]
pub struct Dictionary {
    en_glossery: Glossery,
    // more languages here
    current_language: Language,
}

impl Dictionary {
    pub fn new(current_language: Language) -> Self {
        Dictionary {
            en_glossery: Glossery::new(Language::EN),
            current_language,
        }
    }
    pub fn get_glossary(&self) -> Glossery {
        match self.current_language {
            Language::EN => self.en_glossery.clone(),
            // more languages here
        }
    }

    pub fn get_current_language(&self) -> Language {
        self.current_language
    }

    pub fn set_current_language(&mut self, language: Language) {
        self.current_language = language;
    }
}

impl FromWorld for Dictionary {
    fn from_world(world: &mut World) -> Self {
        let setting = world.get_resource_mut::<Setting>().unwrap();
        Dictionary::new(setting.get_language())
    }
}
