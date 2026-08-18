use hecs::World;
use macroquad::prelude::{clear_background, GRAY, WHITE};
use renderer::{RenderQueue, Renderer};
use crate::engine::components::{Text, Transform};
use crate::engine::systems::collect_texts;

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
                content: "gvh".to_string(),
                size: 30.0,
                color: GRAY,
            },
        ));

        Self {
            world,
            renderer: Renderer::new(),
        }
    }

    pub fn draw(&mut self) {
        clear_background(WHITE);

        let mut queue = RenderQueue::default();
        collect_texts(&self.world, &mut queue);

        self.renderer.draw(&mut queue);
    }
}
