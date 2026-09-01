mod queue;

pub use queue::{RectDraw, RenderQueue, TextDraw};

use crate::engine::assets::{Assets, ShaderId};
use macroquad::prelude::*;

pub struct Renderer;

impl Renderer {
    pub fn new() -> Self {
        Self
    }

    pub fn draw(&mut self, queue: &RenderQueue, assets: &Assets) {
        for r in &queue.rects {
            let use_shader = r.shader != ShaderId::None;
            if use_shader {
                if let Some(mat) = assets.material(r.shader) {
                    mat.set_uniform("time", get_time() as f32);
                    mat.set_uniform("thickness", r.thickness);
                    mat.set_uniform("pulse", r.pulse);
                    gl_use_material(mat);
                }
            }

            if use_shader || r.fill {
                draw_rectangle(r.x, r.y, r.w, r.h, r.color);
            } else {
                draw_rectangle_lines(r.x, r.y, r.w, r.h, r.thickness, r.color);
            }

            if use_shader {
                gl_use_default_material();
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
