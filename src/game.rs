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
            let mut dt = get_frame_time();

            if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl) {
                dt *= 4.0;
            }

            self.engine.update(dt);
            self.engine.draw();

            next_frame().await;
        }
    }
}
