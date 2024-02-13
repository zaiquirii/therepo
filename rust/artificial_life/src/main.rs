use macroquad::color::{BLACK};
use macroquad::input::{is_key_pressed, KeyCode, mouse_position};
use macroquad::math::{Rect, Vec2};
use macroquad::prelude::{next_frame, request_new_screen_size, screen_height, screen_width};
use macroquad::time::get_frame_time;
use macroquad::window::clear_background;
use crate::simulation::Simulation;
use crate::time::FixedTimeLoop;

mod simulation;
mod time;
mod grid;

#[macroquad::main("artificial_life")]
async fn main() {
    request_new_screen_size(480.0 * 2.0, 360.0 * 2.0);
    next_frame().await;

    let mut simulation = Simulation::new(Rect::new(0.0, 0.0, screen_width(), screen_height()));
    let mut show_ui = false;
    let mut paused = false;
    let mut show_grid = false;

    let mut timer = FixedTimeLoop::new();

    simulation.tick();
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::M) {
            show_ui = !show_ui;
        }
        if is_key_pressed(KeyCode::G) {
            show_grid = !show_grid;
        }
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
            timer.reset();
        }

        let step = is_key_pressed(KeyCode::N);

        timer.accumulate(get_frame_time());
        if (!paused && timer.tick(simulation.fps().recip())) || step {
            simulation.tick();
        }

        clear_background(BLACK);
        simulation.render();
        if show_grid {
            let mouse_pos = mouse_position();
            simulation.render_grid(Some(Vec2::new(
                mouse_pos.0,
                mouse_pos.1,
            )));
        }
        if show_ui {
            simulation.render_ui();
        }
        next_frame().await;
    }
}
