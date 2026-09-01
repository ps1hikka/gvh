use hecs::World;
pub(crate) use renderer::{RenderQueue, Renderer};

pub(crate) mod assets;
pub mod components;
pub(crate) mod renderer;
pub mod systems;
pub mod ui;
pub(crate) mod audio;

use crate::engine::assets::Assets;
pub use components::{Text, Transform, Typewriter};

pub struct Engine {
    pub world: World,
    renderer: Renderer,
    pub(crate) assets: Assets,
}

impl Engine {
    pub async fn new() -> Self {
        let assets = Assets::load().await;

        Self {
            world: World::new(),
            assets,
            renderer: Renderer::new(),
        }
    }

    pub fn draw_queue(&mut self, queue: &RenderQueue) {
        self.renderer.draw(queue, &self.assets);
    }
}
