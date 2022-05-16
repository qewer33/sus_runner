use macroquad::prelude::*;

use crate::Game;

#[derive(Debug, Clone, Copy)]
pub struct Obstacle {
    pub x_pos: f32,
    pub speed: f32,
    pub width: u32,
    pub collision_rect: Rect,
}
impl Obstacle {
    pub fn new(speed: f32) -> Obstacle {
        Obstacle {
            x_pos: screen_width(),
            speed,
            width: rand::gen_range(1, 4),
            collision_rect: Rect::new(0.0, 0.0, 0.0, 0.0)
        }
    }

    pub fn draw(&self, parent: &Game) {
        for i in 0..self.width {
            draw_texture(
                *parent.asset_manager.get_texture("obstacle").unwrap(),
                self.x_pos + 55.0*i as f32,
                screen_height() / 2.0 + parent.stage.size / 2.0 - 115.0,
                WHITE,
            );
        }
    }

    pub fn debug_draw(&self, parent: &Game) {
        draw_rectangle(
            self.x_pos + 30.0,
            screen_height() / 2.0 + parent.stage.size / 2.0 - 70.0,
            5.0 + 55.0*self.width as f32,
            70.0,
            Color::from_rgba(237, 35, 61, 100),
        );
    }

    pub fn update(&mut self, delta: f32) {
        self.collision_rect = Rect::new(
            self.x_pos + 30.0,
            screen_height() / 2.0 + 380.0 / 2.0 - 70.0,
            5.0 + 55.0*self.width as f32,
            70.0
        );

        self.x_pos -= self.speed * delta;
    }
}
