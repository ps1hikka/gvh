use macroquad::prelude::Color;
use crate::engine::assets::FontId;

pub struct TextDraw {
    pub content: String,
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub color: Color,
    pub font: FontId,
}

pub struct RectDraw {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: Color,
    pub fill: bool,
    pub thickness: f32,
}

#[derive(Default)]
pub struct RenderQueue {
    pub world_texts: Vec<TextDraw>,
    pub rects: Vec<RectDraw>,
}
