use macroquad::audio::{Sound, load_sound};
use macroquad::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontId {
    Ui,
    Title,
    Dialogue,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum SfxId {
    UiHover,
}

pub struct Assets {
    fonts: HashMap<FontId, Font>,
    sounds: HashMap<SfxId, Sound>,
}

impl Assets {
    pub async fn load() -> Self {
        let mut fonts = HashMap::new();
        fonts.insert(
            FontId::Ui,
            load_ttf_font("assets/fonts/Inter-Regular.ttf")
                .await
                .unwrap(),
        );
        fonts.insert(
            FontId::Title,
            load_ttf_font("assets/fonts/Inter-Bold.ttf").await.unwrap(),
        );
        fonts.insert(
            FontId::Dialogue,
            load_ttf_font("assets/fonts/Inter-Regular.ttf")
                .await
                .unwrap(),
        );

        let mut sounds = HashMap::new();
        sounds.insert(
            SfxId::UiHover,
            load_sound("assets/sfx/ui_hover.ogg").await.unwrap(),
        );
        Self { fonts, sounds }
    }

    pub fn font(&self, id: FontId) -> &Font {
        &self.fonts[&id]
    }

    pub fn sound(&self, id: SfxId) -> &Sound {
        &self.sounds[&id]
    }
}
