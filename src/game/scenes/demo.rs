use crate::engine::systems;
use crate::engine::{Engine, RenderQueue, Text, Transform, Typewriter};
use crate::game::scene::SceneCommand;
use macroquad::prelude::*;

pub struct DemoScene;

impl DemoScene {
    pub fn new(engine: &mut Engine) -> Self {
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
        Self
    }

    pub fn update(&mut self, engine: &mut Engine, dt: f32) -> SceneCommand {
        systems::typewriter_system(&mut engine.world, dt);
        SceneCommand::None
    }

    pub fn draw(&self, engine: &Engine, queue: &mut RenderQueue) {
        systems::collect_texts(&engine.world, queue);
    }
}