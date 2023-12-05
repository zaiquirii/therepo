mod game;

use macroquad::prelude::*;

#[macroquad::main("tetris")]
async fn main() {
    let mut game = game::Game::new(UVec2::new(10, 20));
    let mut last_time = get_time();
    loop {
        let delta = get_frame_time();
        let now = get_time();
        // println!("time: {} {} {}", delta, now, now - last_time);
        last_time = now;
        clear_background(BLACK);
        game.handle_input();
        game.update(get_frame_time());
        game.render();
        next_frame().await
    }
}
