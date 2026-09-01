use crate::engine::systems;
use crate::engine::{Engine, RenderQueue, Text, Transform, Typewriter};
use crate::game::flow::FlowEvent;
use macroquad::prelude::*;

pub struct DemoScene {
    timer: f32,
}

impl DemoScene {
    pub fn new() -> Self {
        Self { timer: 0.0 }
    }

    pub fn enter(&mut self, engine: &mut Engine) {
        engine.world.spawn((
            Transform { x: 20.0, y: 40.0 },
            Text {
                content: String::new(),
                size: 30.0,
                color: GRAY,
            },
            Typewriter {
                full_text: "gvh".to_string(),
                chars_per_sec: 3.0,
                timer: 0.0,
                visible: 0,
                going_forward: true,
                looped: true,
            },
        ));
    }

    pub fn exit(&mut self, engine: &mut Engine) {
        engine.world.clear();
    }

    pub fn update(&mut self, engine: &mut Engine, dt: f32) -> FlowEvent {
        systems::typewriter_system(&mut engine.world, dt);

        self.timer += dt;
        if self.timer > 2.3 {
            FlowEvent::DemoFinished
        } else {
            FlowEvent::None
        }
    }

    pub fn draw(&self, engine: &Engine, queue: &mut RenderQueue) {
        systems::collect_texts(&engine.world, queue);
    }
}
