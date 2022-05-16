use macroquad::prelude::*;

use crate::Game;

#[derive(Clone, Copy)]
pub struct Cloud {
    pub x_pos: f32,
    pub y_pos: f32,
    pub variant: i32,
    pub speed: f32,
}
impl Cloud {
    pub fn new(x_pos: f32, speed: f32) -> Cloud {
        Cloud {
            x_pos,
            y_pos: rand::gen_range(15.0, 75.0),
            variant: rand::gen_range(1, 4),
            speed,
        }
    }

    pub fn draw(&self, parent: &Game) {
        draw_texture(
            *parent
                .asset_manager
                .get_texture(format!("cloud_{}", self.variant).as_str())
                .unwrap(),
            self.x_pos,
            screen_height() / 2.0 - parent.stage.size / 2.0 + self.y_pos,
            WHITE,
        );
    }

    pub fn update(&mut self) {
        self.x_pos -= self.speed;
    }
}
