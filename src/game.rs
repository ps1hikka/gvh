mod flow;
mod scene;
mod scenes;

use crate::engine::{Engine, RenderQueue};
use macroquad::prelude::*;
use scene::{Scene, SceneId, create};

pub struct Game {
    engine: Engine,
    scene: Box<dyn Scene>,
}

impl Game {
    pub fn new() -> Self {
        let mut engine = Engine::new();
        let mut scene = create(SceneId::Demo);
        scene.enter(&mut engine);

        Self { engine, scene }
    }

    pub async fn start(&mut self) {
        loop {
            let mut dt = get_frame_time();
            if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl) {
                dt *= 4.0;
            }

            self.update(dt);
            self.draw();
            next_frame().await;
        }
    }

    fn update(&mut self, dt: f32) {
        let event = self.scene.update(&mut self.engine, dt);

        if let Some(id) = flow::handle(event) {
            self.switch(id);
        }
    }

    fn switch(&mut self, id: SceneId) {
        self.scene.exit(&mut self.engine);
        let mut next = create(id);
        next.enter(&mut self.engine);
        self.scene = next;
    }

    fn draw(&mut self) {
        clear_background(WHITE);

        let mut queue = RenderQueue::default();
        self.scene.draw(&self.engine, &mut queue);
        self.engine.draw_queue(&queue);
    }
}
