mod scene;
mod scenes;

use crate::engine::{Engine, RenderQueue};
use macroquad::prelude::*;
use scene::{Scene, SceneCommand};
use scenes::DemoScene;

pub struct Game {
    engine: Engine,
    scene: Scene,
}

impl Game {
    pub fn new() -> Self {
        let mut engine = Engine::new();
        let demo = DemoScene::new(&mut engine);

        Self {
            engine,
            scene: Scene::Demo(demo),
        }
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
        match self.scene.update(&mut self.engine, dt) {
            SceneCommand::None => {}
        }
    }

    fn draw(&mut self) {
        clear_background(WHITE);

        let mut queue = RenderQueue::default();
        self.scene.draw(&self.engine, &mut queue);
        self.engine.draw_queue(&queue);
    }
}
