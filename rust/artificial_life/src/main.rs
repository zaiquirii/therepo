use macroquad::color::{BLACK};
use macroquad::input::{is_key_pressed, KeyCode, mouse_position};
use macroquad::math::{Rect, Vec2};
use macroquad::prelude::{next_frame, request_new_screen_size, screen_height, screen_width};
use macroquad::time::get_frame_time;
use macroquad::window::{clear_background, Conf};
use crate::simulation::Simulation;
use crate::time::FixedTimeLoop;

mod simulation;
mod time;
mod grid;

const UI_WIDTH: f32 = 400.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Artificial Life".to_owned(),
        window_width: 480 * 2 + UI_WIDTH as i32,
        window_height: 360 * 2,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut simulation = Simulation::new(Rect::new(0.0, 0.0, screen_width() - UI_WIDTH, screen_height()));
    let mut paused = false;
    let mut show_grid = false;

    let mut timer = FixedTimeLoop::new();

    simulation.tick();
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
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
        simulation.render_ui();
        next_frame().await;
    }
}
