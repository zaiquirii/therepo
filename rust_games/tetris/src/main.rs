mod game;

use macroquad::prelude::*;

#[macroquad::main("tetris")]
async fn main() {
    let mut game = game::Game::new(UVec2::new(10, 20));
    loop {
        clear_background(BLACK);
        game.handle_input();
        game.update(get_frame_time());
        game.render();
        next_frame().await
    }
}
