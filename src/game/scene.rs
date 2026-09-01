use crate::engine::{Engine, RenderQueue};
use crate::game::flow::FlowEvent;

pub trait Scene {
    fn enter(&mut self, _engine: &mut Engine) {}
    fn exit(&mut self, _engine: &mut Engine) {}
    fn update(&mut self, engine: &mut Engine, dt: f32) -> FlowEvent;
    fn draw(&self, engine: &Engine, queue: &mut RenderQueue);
}

pub enum SceneId {
    Menu,
    Demo,
}

pub fn create(id: SceneId) -> Box<dyn Scene> {
    match id {
        SceneId::Menu => Box::new(super::scenes::MenuScene::new()),
        SceneId::Demo => Box::new(super::scenes::DemoScene::new()),
    }
}
