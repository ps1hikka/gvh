use macroquad::prelude::*;

pub struct Game;

impl Game {
    pub fn new() -> Self {
        Self
    }

    pub async fn start(&self) {
        loop {
            clear_background(WHITE);
            draw_text("gvh", 20.0, 40.0, 30.0, GRAY);

            next_frame().await;
        }
    }
}
