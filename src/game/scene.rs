use crate::engine::{Engine, RenderQueue};
use crate::game::scenes::DemoScene;

pub enum SceneCommand {
    None,
}

pub enum Scene {
    Demo(DemoScene),
}

impl Scene {
    pub fn update(&mut self, engine: &mut Engine, dt: f32) -> SceneCommand {
        match self {
            Scene::Demo(scene) => scene.update(engine, dt),
        }
    }

    pub fn draw(&self, engine: &Engine, queue: &mut RenderQueue) {
        match self {
            Scene::Demo(scene) => scene.draw(engine, queue),
        }
    }
}
