use macroquad::prelude::*;

use crate::objects::{Cloud, Obstacle, Player, Stage};
use crate::AssetManager;

#[derive(Clone)]
pub enum GameState {
    Start,
    InGame,
    GameOver,
}

#[derive(Clone)]
pub struct Game {
    pub delta: f32,
    pub asset_manager: AssetManager,
    pub state: GameState,
    pub score: u32,
    pub speed: f32,
    pub debug_draw: bool,
    pub stage: Stage,
    pub player: Player,
    pub obstacles: Vec<Obstacle>,
    pub clouds: Vec<Cloud>,
    prev: f64,
    next_spawn: f64,
    score_prev: u32,
}
impl Game {
    pub fn new() -> Game {
        Game {
            delta: 1.0,
            asset_manager: AssetManager::new(),
            state: GameState::Start,
            score: 0,
            speed: 8.0,
            debug_draw: false,
            stage: Stage::new(),
            player: Player::new(),
            obstacles: Vec::new(),
            clouds: Vec::new(),
            prev: get_time(),
            next_spawn: rand::gen_range(1.5, 10.0),
            score_prev: 0,
        }
    }

    pub async fn ready(&mut self) {
        self.asset_manager
            .add_texture("banner", "assets/banner.png")
            .await
            .unwrap()
            .add_texture("player_idle", "assets/idle.png")
            .await
            .unwrap()
            .add_texture("player_run_1", "assets/run_1.png")
            .await
            .unwrap()
            .add_texture("player_run_2", "assets/run_2.png")
            .await
            .unwrap()
            .add_texture("cloud_1", "assets/cloud_1.png")
            .await
            .unwrap()
            .add_texture("cloud_2", "assets/cloud_2.png")
            .await
            .unwrap()
            .add_texture("cloud_3", "assets/cloud_3.png")
            .await
            .unwrap()
            .add_texture("obstacle", "assets/obstacle.png")
            .await
            .unwrap()
            .add_font("default", "assets/Kenney Pixel Square.ttf")
            .await
            .unwrap();
    }

    pub fn draw(&self) {
        clear_background(Color::from_rgba(13, 43, 69, 255));

        self.stage.draw(&self);

        for obstacle in &self.obstacles {
            obstacle.draw(&self);
            if self.debug_draw {
                obstacle.debug_draw(&self);
            }
        }

        for cloud in &self.clouds {
            cloud.draw(&self);
        }

        self.player.draw(&self);
        if self.debug_draw {
            self.player.debug_draw();
        }

        match self.state {
            GameState::Start => {
                draw_texture(
                    *self.asset_manager.get_texture("banner").unwrap(),
                    screen_width() / 2.0 - 215.0,
                    screen_height() / 2.0 - 110.0,
                    WHITE,
                );

                draw_text_ex(
                    "PRESS SPACE TO START",
                    screen_width() / 2.0 - 175.0,
                    screen_height() / 2.0 + 50.0,
                    TextParams {
                        font: *self.asset_manager.get_font("default").unwrap(),
                        font_size: 25,
                        color: Color::from_rgba(13, 43, 69, 255),
                        ..Default::default()
                    },
                );

                draw_text_ex(
                    "press d to activate debug draw",
                    screen_width() / 2.0 - 155.0,
                    screen_height() / 2.0 + 80.0,
                    TextParams {
                        font: *self.asset_manager.get_font("default").unwrap(),
                        font_size: 15,
                        color: Color::from_rgba(13, 43, 69, 255),
                        ..Default::default()
                    },
                );
            }
            GameState::InGame => {}
            GameState::GameOver => {
                draw_text_ex(
                    "DEFEAT",
                    screen_width() / 2.0 - 145.0,
                    screen_height() / 2.0 - 30.0,
                    TextParams {
                        font: *self.asset_manager.get_font("default").unwrap(),
                        font_size: 70,
                        color: Color::from_rgba(13, 43, 69, 255),
                        ..Default::default()
                    },
                );

                draw_text_ex(
                    format!("YOUR SCORE WAS: {:06}", self.score).as_str(),
                    screen_width() / 2.0 - 175.0,
                    screen_height() / 2.0 + 50.0,
                    TextParams {
                        font: *self.asset_manager.get_font("default").unwrap(),
                        font_size: 25,
                        color: Color::from_rgba(13, 43, 69, 255),
                        ..Default::default()
                    },
                );

                draw_text_ex(
                    "PRESS SPACE TO RESTART",
                    screen_width() / 2.0 - 110.0,
                    screen_height() / 2.0 + 80.0,
                    TextParams {
                        font: *self.asset_manager.get_font("default").unwrap(),
                        font_size: 15,
                        color: Color::from_rgba(13, 43, 69, 255),
                        ..Default::default()
                    },
                );
            }
        }
    }

    pub fn update(&mut self) {
        self.delta = 60.0 / get_fps() as f32;

        if is_key_pressed(KeyCode::D) {
            self.debug_draw = !self.debug_draw;
        }

        if self.score - self.score_prev > 1000 {
            self.score_prev = self.score;
            self.speed += 1.0;
        }

        match self.state {
            GameState::Start => {
                if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                    self.state = GameState::InGame;
                    self.prev = get_time();
                    self.player.jump(self.delta);
                }
            }
            GameState::InGame => {
                self.score += 1;

                self.player.update(self.delta);

                self.spawn_obstacles();

                for (i, obstacle) in self.obstacles.clone().iter().enumerate() {
                    if self
                        .player
                        .collision_rect
                        .overlaps(&obstacle.collision_rect)
                    {
                        self.state = GameState::GameOver;
                        self.clouds.clear();
                    }

                    if obstacle.x_pos < -300.0 {
                        self.obstacles.remove(i);
                        break;
                    } else {
                        self.obstacles.get_mut(i).unwrap().update(self.delta);
                    }
                }

                for (i, cloud) in self.clouds.clone().iter().enumerate() {
                    if cloud.x_pos < -300.0 {
                        self.clouds.remove(i);
                        break;
                    } else {
                        self.clouds.get_mut(i).unwrap().update();
                    }
                }
            }
            GameState::GameOver => {
                if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                    self.state = GameState::InGame;
                    self.score = 0;
                    self.player.y_pos = 0.0;
                    self.obstacles.clear();
                }
            }
        }
    }

    fn spawn_obstacles(&mut self) {
        if get_time() - self.prev > self.next_spawn {
            self.obstacles.push(Obstacle::new(self.speed));
            self.prev = get_time();
            self.next_spawn = rand::gen_range(0.7, 2.);

            self.clouds.push(Cloud::new(
                screen_width() + 200.0 + rand::gen_range(0.0, 300.0),
                self.speed / 2.0,
            ));
            if rand::gen_range(0, 2) == 0 {
                self.clouds.push(Cloud::new(
                    screen_width() + 100.0 - rand::gen_range(0.0, 100.0),
                    self.speed / rand::gen_range(2.0, 2.6),
                ));
            }
        }
    }
}
