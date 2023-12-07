mod game;

use std::time::{SystemTime};
use macroquad::prelude::*;
use macroquad::rand::srand;

#[macroquad::main("tetris")]
async fn main() {
    let d = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    srand(d.as_secs());
    let mut game = game::Game::new(UVec2::new(10, 20));
    loop {
        clear_background(BLACK);
        game.handle_input();
        game.update(get_frame_time());
        game.render();
        next_frame().await
    }
}
