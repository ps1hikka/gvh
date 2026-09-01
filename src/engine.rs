use hecs::World;
pub(crate) use renderer::{RenderQueue, Renderer};

pub(crate) mod renderer;
pub mod systems;
pub mod components;

pub use components::{Text, Transform, Typewriter};

pub struct Engine {
    pub world: World,
    renderer: Renderer,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            renderer: Renderer::new(),
        }
    }

    pub fn draw_queue(&mut self, queue: &RenderQueue) {
        self.renderer.draw(queue);
    }
}