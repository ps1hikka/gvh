use macroquad::prelude::Color;

pub struct Transform {
    pub x: f32,
    pub y: f32,
}

pub struct Text {
    pub content: String,
    pub size: f32,
    pub color: Color,
}

pub struct Typewriter {
    pub full_text: String,
    pub chars_per_sec: f32,
    pub timer: f32,
    pub visible: usize,
    pub going_forward: bool,
    pub looped: bool,
}