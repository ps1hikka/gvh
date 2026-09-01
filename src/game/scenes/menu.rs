use crate::engine::ui::{Align, FlexDirection, FlexPanel, FlexStyle, Rect};
use crate::engine::{Engine, RenderQueue};
use crate::game::flow::FlowEvent;
use crate::game::scene::Scene;
use crate::game::ui::Button;
use macroquad::prelude::{screen_height, screen_width};

pub struct MenuScene {
    panel: FlexPanel,
    start: Button,
    quit: Button,
}

impl MenuScene {
    pub fn new() -> Self {
        let style = FlexStyle {
            direction: FlexDirection::Column,
            gap: 16.0,
            padding: 40.0,
            align: Align::Start,
        };

        Self {
            panel: FlexPanel::new(style),
            start: Button::new("Начать", 220.0, 48.0),
            quit: Button::new("Выход", 220.0, 48.0),
        }
    }

    fn layout(&mut self) {
        self.panel
            .set_rect(Rect::new(0.0, 0.0, screen_width(), screen_height()));

        let mut items = [self.start.item, self.quit.item];
        self.panel.layout(&mut items);
        self.start.item = items[0];
        self.quit.item = items[1];
    }
}

impl Scene for MenuScene {
    fn update(&mut self, _engine: &mut Engine, _dt: f32) -> FlowEvent {
        self.layout();

        if self.start.update() {
            return FlowEvent::StartGame;
        }
        if self.quit.update() {
            std::process::exit(0);
        }
        FlowEvent::None
    }

    fn draw(&self, _engine: &Engine, queue: &mut RenderQueue) {
        self.start.draw(queue);
        self.quit.draw(queue);
    }
}
