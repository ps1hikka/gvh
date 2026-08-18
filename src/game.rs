use macroquad::prelude::*;
use crate::engine::Engine;

pub struct Game {
    engine: Engine,
}

impl Game {
    pub fn new() -> Self {
        Self { engine: Engine::new()}
    }

    pub async fn start(&mut self) {
        loop {
            self.engine.draw();
            next_frame().await;
        }
    }
}
