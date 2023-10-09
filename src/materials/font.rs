use bevy::prelude::*;

use crate::resources::language::Language;

#[derive(Resource)]
pub struct FontMaterials {
    pub roboto_font: Handle<Font>,
    // more fonts here
}

impl FontMaterials {
    pub fn get_font(&self, language: Language) -> Handle<Font> {
        return match language {
            Language::EN => self.roboto_font.clone(),
            // more language here
        };
    }
}
