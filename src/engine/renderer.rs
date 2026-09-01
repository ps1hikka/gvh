mod queue;

pub use queue::{RenderQueue, TextDraw, RectDraw};

use crate::engine::assets::Assets;
use macroquad::prelude::{TextParams, draw_rectangle, draw_rectangle_lines, draw_text_ex};

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&mut self, queue: &RenderQueue, assets: &Assets) {
        for r in &queue.rects {
            if r.fill {
                draw_rectangle(r.x, r.y, r.w, r.h, r.color);
            } else {
                draw_rectangle_lines(r.x, r.y, r.w, r.h, r.thickness, r.color);
            }
        }

        for t in &queue.world_texts {
            draw_text_ex(
                &t.content,
                t.x,
                t.y,
                TextParams {
                    font: Some(assets.font(t.font)),
                    font_size: t.size as u16,
                    color: t.color,
                    ..Default::default()
                },
            );
        }
    }
}
