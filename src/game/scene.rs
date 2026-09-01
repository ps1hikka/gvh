use crate::engine::{Engine, RenderQueue};
use crate::game::flow::FlowEvent;
use crate::game::scenes::{DemoScene, MenuScene};

pub enum SceneId {
    Menu,
    Demo,
}

pub enum Scene {
    Menu(MenuScene),
    Demo(DemoScene),
}

impl Scene {
    pub fn create(id: SceneId) -> Self {
        match id {
            SceneId::Menu => Scene::Menu(MenuScene::new()),
            SceneId::Demo => Scene::Demo(DemoScene::new()),
        }
    }

    pub fn enter(&mut self, engine: &mut Engine) {
        match self {
            Scene::Menu(s) => s.enter(engine),
            Scene::Demo(s) => s.enter(engine),
        }
    }

    pub fn exit(&mut self, engine: &mut Engine) {
        match self {
            Scene::Menu(s) => s.exit(engine),
            Scene::Demo(s) => s.exit(engine),
        }
    }

    pub fn update(&mut self, engine: &mut Engine, dt: f32) -> FlowEvent {
        match self {
            Scene::Menu(s) => s.update(engine, dt),
            Scene::Demo(s) => s.update(engine, dt),
        }
    }

    pub fn draw(&self, engine: &Engine, queue: &mut RenderQueue) {
        match self {
            Scene::Menu(s) => s.draw(engine, queue),
            Scene::Demo(s) => s.draw(engine, queue),
        }
    }
}
