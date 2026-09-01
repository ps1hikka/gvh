use hecs::World;
use crate::engine::assets::FontId;
use crate::engine::components::{Text, Transform};
use crate::engine::renderer::{RenderQueue, TextDraw};

pub fn collect_texts(world: &World, queue: &mut RenderQueue) {
    for (tf, text) in world.query::<(&Transform, &Text)>().iter() {
        queue.world_texts.push(TextDraw {
            content: text.content.clone(),
            x: tf.x,
            y: tf.y,
            size: text.size,
            color: text.color,
            font: FontId::Ui,
        });
    }
}