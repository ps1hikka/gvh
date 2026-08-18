mod queue;

pub use queue::{RenderQueue, TextDraw};

use macroquad::prelude::draw_text;

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&mut self, queue: &RenderQueue) {
        for t in &queue.world_texts {
            draw_text(&t.content, t.x, t.y, t.size, t.color);
        }
    }
}