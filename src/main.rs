use macroquad::prelude::*;

use sus_runner::Game;

#[macroquad::main("sus_runner")]
async fn main() {
    let mut game = Game::new();

    game.ready().await;

    loop {
        game.update();
        game.draw();

        next_frame().await
    }
}
