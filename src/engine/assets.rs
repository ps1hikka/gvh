use macroquad::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontId {
    Ui,
    Title,
    Dialogue,
}

pub struct Assets {
    fonts: HashMap<FontId, Font>,
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
        Self { fonts }
    }

    pub fn font(&self, id: FontId) -> &Font {
        &self.fonts[&id]
    }
}
