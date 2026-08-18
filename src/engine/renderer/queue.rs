use macroquad::prelude::Color;

pub struct TextDraw {
    pub content: String,
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub color: Color,
}

#[derive(Default)]
pub struct RenderQueue {
    pub world_texts: Vec<TextDraw>,
}
