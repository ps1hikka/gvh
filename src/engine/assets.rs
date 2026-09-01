use macroquad::audio::{Sound, load_sound};
use macroquad::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum FontId {
    #[default]
    Ui,
    Title,
    Dialogue,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum SfxId {
    UiHover,
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum ShaderId {
    #[default]
    None,
    Outline,
}

pub struct Assets {
    fonts: HashMap<FontId, Font>,
    sounds: HashMap<SfxId, Sound>,
    materials: HashMap<ShaderId, Material>,
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

        let outline = load_material(
            ShaderSource::Glsl {
                vertex: include_str!("../../assets/shaders/outline.vert"),
                fragment: include_str!("../../assets/shaders/outline.frag"),
            },
            MaterialParams {
                uniforms: vec![
                    UniformDesc::new("time", UniformType::Float1),
                    UniformDesc::new("thickness", UniformType::Float1),
                    UniformDesc::new("pulse", UniformType::Float1),
                ],
                ..Default::default()
            },
        )
        .expect("outline shader");

        let mut materials = HashMap::new();
        materials.insert(ShaderId::Outline, outline);

        Self {
            fonts,
            sounds,
            materials,
        }
    }

    pub fn font(&self, id: FontId) -> &Font {
        &self.fonts[&id]
    }

    pub fn sound(&self, id: SfxId) -> &Sound {
        &self.sounds[&id]
    }

    pub fn material(&self, id: ShaderId) -> Option<&Material> {
        self.materials.get(&id)
    }
}
