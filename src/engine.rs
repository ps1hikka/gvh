use macroquad::prelude::{clear_background, WHITE};
use renderer::{RenderQueue, Renderer, TextDraw};
mod renderer;

pub struct Engine {
    renderer: Renderer,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            renderer: Renderer::new(),
        }
    }

    pub fn draw(&mut self) {
        clear_background(WHITE);

        let mut queue = RenderQueue::default();

        queue.world_texts.push(TextDraw {
            content: "gvh".to_string(),
            x: 20.0,
            y: 40.0,
            size: 30.0,
            color: macroquad::prelude::GRAY,
        });

        self.renderer.draw(&mut queue);
    }
}
