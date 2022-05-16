use macroquad::prelude::*;
// use macroquad::experimental::animation::{AnimatedSprite, Animation};

use crate::Game;

#[derive(Clone)]
pub struct Player {
    pub y_pos: f32,
    pub collision_rect: Rect,
    gravity: f32,
    jump_pow: f32,
    y_vel: f32,
}
impl Player {
    pub fn new() -> Player {
        Player {
            y_pos: 0.0,
            collision_rect: Rect::new(0.0, 0.0, 0.0, 0.0),
            gravity: 1.2,
            jump_pow: 25.0,
            y_vel: 0.0,
        }
    }

    pub fn draw(&self, parent: &Game) {
        let texture = *parent.asset_manager.get_texture("player_idle").unwrap();

        draw_texture(
            texture,
            30.0,
            screen_height() / 2.0 + parent.stage.size / 2.0 - self.y_pos - 120.0,
            WHITE,
        );
    }

    pub fn debug_draw(&self, parent: &Game) {
        draw_rectangle(
            60.0,
            screen_height() / 2.0 + parent.stage.size / 2.0 - self.y_pos - 90.0,
            60.0,
            90.0,
            Color::from_rgba(24, 143, 247, 100),
        );
    }

    pub fn update(&mut self, parent: &Game) {
        self.collision_rect = Rect::new(
            60.0,
            screen_height() / 2.0 + parent.stage.size / 2.0 - self.y_pos - 90.0,
            60.0,
            90.0,
        );

        self.jump();
    }

    pub fn jump(&mut self) {
        if is_key_pressed(KeyCode::Space) && self.y_pos == 0.0 {
            self.y_vel = self.jump_pow;
        }

        self.y_pos += self.y_vel;

        if self.y_pos > 0.0 {
            self.y_vel -= self.gravity;
        } else {
            self.y_pos = 0.0;
        }
    }
}
