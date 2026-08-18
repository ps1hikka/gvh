use hecs::World;
use macroquad::prelude::{clear_background, GRAY, WHITE};
use renderer::{RenderQueue, Renderer};
use crate::engine::components::{Text, Transform, Typewriter};

pub(crate) mod renderer;
pub mod systems;
mod components;

pub struct Engine {
    world: World,
    renderer: Renderer
}

impl Engine {
    pub fn new() -> Self {
        let mut world = World::new();

        world.spawn((
            Transform { x: 20.0, y: 40.0 },
            Text {
                content: String::new(), // начнём с пустого
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

        Self {
            world,
            renderer: Renderer::new(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        systems::typewriter_system(&mut self.world, dt);
    }

    pub fn draw(&mut self) {
        clear_background(WHITE);

        let mut queue = RenderQueue::default();
        systems::collect_texts(&self.world, &mut queue);

        self.renderer.draw(&mut queue);
    }
}
