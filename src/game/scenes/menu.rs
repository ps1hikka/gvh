use crate::engine::renderer::TextDraw;
use crate::engine::{Engine, RenderQueue};
use crate::game::flow::FlowEvent;
use crate::game::scene::Scene;
use macroquad::prelude::*;

pub struct MenuScene;

impl MenuScene {
    pub fn new() -> Self {
        Self
    }
}

impl Scene for MenuScene {
    fn update(&mut self, _engine: &mut Engine, _dt: f32) -> FlowEvent {
        if is_key_pressed(KeyCode::Enter) {
            FlowEvent::StartGame
        } else {
            FlowEvent::None
        }
    }

    fn draw(&self, _engine: &Engine, queue: &mut RenderQueue) {
        queue.world_texts.push(TextDraw {
            content: "MENU".into(),
            x: 40.0,
            y: 80.0,
            size: 40.0,
            color: GRAY,
        });
        queue.world_texts.push(TextDraw {
            content: "Enter: go to demo".into(),
            x: 40.0,
            y: 140.0,
            size: 24.0,
            color: GRAY,
        });
    }
}
