use crate::engine::RenderQueue;
use crate::engine::assets::{Assets, FontId, SfxId};
use crate::engine::renderer::{RectDraw, TextDraw};
use crate::engine::ui::{FlexItem, Rect};
use macroquad::prelude::*;

pub struct Button {
    pub item: FlexItem,
    pub label: String,
    pub hovered: bool,
    pub font: FontId,
}

impl Button {
    pub fn new(label: &str, w: f32, h: f32) -> Self {
        Self {
            item: FlexItem::new(w, h),
            label: label.to_string(),
            hovered: false,
            font: FontId::Ui,
        }
    }

    pub fn rect(&self) -> Rect {
        self.item.computed
    }

    pub fn update(&mut self, assets: &Assets) -> bool {
        let (mx, my) = mouse_position();
        let hovered = self.rect().contains(mx, my);

        if hovered && !self.hovered {
            crate::engine::audio::play(assets, SfxId::UiHover);
        }

        self.hovered = hovered;
        self.hovered && is_mouse_button_pressed(MouseButton::Left)
    }

    pub fn draw(&self, queue: &mut RenderQueue) {
        let r = self.rect();
        let color = if self.hovered { DARKGRAY } else { GRAY };

        queue.rects.push(RectDraw {
            x: r.x,
            y: r.y,
            w: r.w,
            h: r.h,
            color,
            fill: false,
            thickness: 2.0,
        });

        queue.world_texts.push(TextDraw {
            content: self.label.clone(),
            x: r.x + 12.0,
            y: r.y + r.h * 0.5 + 8.0,
            size: 24.0,
            color,
            font: self.font,
        });
    }
}
