use macroquad::prelude::*;

use crate::Game;

#[derive(Clone)]
pub struct Stage {
    pub size: f32,
}
impl Stage {
    pub fn new() -> Stage {
        Stage { size: 380.0 }
    }

    pub fn draw(&self, parent: &Game) {
        draw_rectangle(
            0.0,
            screen_height() / 2.0 - self.size / 2.0,
            screen_width(),
            self.size,
            Color::from_rgba(208, 129, 89, 255),
        );

        draw_text_ex(
            format!("{:06}", parent.score).as_str(),
            screen_width() - 175.0,
            screen_height() / 2.0 - self.size / 2.0 + 50.0,
            TextParams {
                font: *parent.asset_manager.get_font("default").unwrap(),
                font_size: 35,
                color: Color::from_rgba(13, 43, 69, 255),
                ..Default::default()
            },
        );
    }
}
