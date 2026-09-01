#![allow(dead_code)]

use super::Rect;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FlexDirection {
    Row,
    Column,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Align {
    Start,
    Center,
    Stretch,
}

#[derive(Clone, Copy, Debug)]
pub struct FlexStyle {
    pub direction: FlexDirection,
    pub gap: f32,
    pub padding: f32,
    pub align: Align,
}

impl Default for FlexStyle {
    fn default() -> Self {
        Self {
            direction: FlexDirection::Column,
            gap: 0.0,
            padding: 0.0,
            align: Align::Stretch,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FlexItem {
    pub grow: f32,
    pub shrink: f32,
    pub basis: (f32, f32),
    pub min: (f32, f32),
    pub max: (f32, f32),
    pub computed: Rect,
}

impl FlexItem {
    pub fn new(basis_w: f32, basis_h: f32) -> Self {
        Self {
            grow: 0.0,
            shrink: 1.0,
            basis: (basis_w, basis_h),
            min: (0.0, 0.0),
            max: (f32::MAX, f32::MAX),
            computed: Rect::default(),
        }
    }
}

pub struct FlexPanel {
    pub style: FlexStyle,
    pub rect: Rect,
    dirty: bool,
}

impl FlexPanel {
    pub fn new(style: FlexStyle) -> Self {
        Self {
            style,
            rect: Rect::default(),
            dirty: true,
        }
    }

    pub fn set_rect(&mut self, rect: Rect) {
        if self.rect != rect {
            self.rect = rect;
            self.dirty = true;
        }
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn layout(&mut self, items: &mut [FlexItem]) {
        if !self.dirty || items.is_empty() {
            self.dirty = false;
            return;
        }

        let pad = self.style.padding;
        let gap = self.style.gap;
        let inner_x = self.rect.x + pad;
        let inner_y = self.rect.y + pad;
        let inner_w = (self.rect.w - pad * 2.0).max(0.0);
        let inner_h = (self.rect.h - pad * 2.0).max(0.0);
        let gaps = gap * (items.len().saturating_sub(1) as f32);

        match self.style.direction {
            FlexDirection::Column => {
                let mut used = 0.0;
                let mut grow_sum = 0.0;
                for item in items.iter() {
                    used += item.basis.1.clamp(item.min.1, item.max.1);
                    grow_sum += item.grow.max(0.0);
                }
                let extra = (inner_h - gaps - used).max(0.0);

                let mut y = inner_y;
                for item in items.iter_mut() {
                    let mut h = item.basis.1.clamp(item.min.1, item.max.1);
                    if grow_sum > 0.0 && item.grow > 0.0 {
                        h += extra * (item.grow / grow_sum);
                    }
                    h = h.clamp(item.min.1, item.max.1);

                    let (x, w) = match self.style.align {
                        Align::Start => (
                            inner_x,
                            item.basis.0.clamp(item.min.0, item.max.0).min(inner_w),
                        ),
                        Align::Center => {
                            let w = item.basis.0.clamp(item.min.0, item.max.0).min(inner_w);
                            (inner_x + (inner_w - w) * 0.5, w)
                        }
                        Align::Stretch => (inner_x, inner_w),
                    };

                    item.computed = Rect::new(x, y, w, h);
                    y += h + gap;
                }
            }
            FlexDirection::Row => {
                let mut used = 0.0;
                let mut grow_sum = 0.0;
                for item in items.iter() {
                    used += item.basis.0.clamp(item.min.0, item.max.0);
                    grow_sum += item.grow.max(0.0);
                }
                let extra = (inner_w - gaps - used).max(0.0);

                let mut x = inner_x;
                for item in items.iter_mut() {
                    let mut w = item.basis.0.clamp(item.min.0, item.max.0);
                    if grow_sum > 0.0 && item.grow > 0.0 {
                        w += extra * (item.grow / grow_sum);
                    }
                    w = w.clamp(item.min.0, item.max.0);

                    let (y, h) = match self.style.align {
                        Align::Start => (
                            inner_y,
                            item.basis.1.clamp(item.min.1, item.max.1).min(inner_h),
                        ),
                        Align::Center => {
                            let h = item.basis.1.clamp(item.min.1, item.max.1).min(inner_h);
                            (inner_y + (inner_h - h) * 0.5, h)
                        }
                        Align::Stretch => (inner_y, inner_h),
                    };

                    item.computed = Rect::new(x, y, w, h);
                    x += w + gap;
                }
            }
        }

        self.dirty = false;
    }
}
