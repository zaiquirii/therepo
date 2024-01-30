use std::fmt::{Display, Formatter};
use macroquad::color::BLACK;
use macroquad::prelude::*;
use crate::hindsight::{Session, PlayerCount, PlayerId, PlayerInfo};
use crate::input::{LocalInput, PlayerInput};
use crate::time::FixedTimeLoop;

mod hindsight;
mod input;
mod time;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("highsight had an issue")]
    Network(#[from] hindsight::Error)
}

type Result<T> = std::result::Result<T, Error>;

#[macroquad::main("hindsight")]
async fn main() -> Result<()> {
    let mut game = Game::new()?;
    while game.running {
        // General gameplay loop
        // collect input
        // sync input
        // - On difference
        //      - Restore state from save point
        //      - Advance game state with new inputs
        // update state
        // render
        game.update();
        game.render().await;
    }
    Ok(())
}

enum UpdateResult {
    Continue,
    Quit,
}

struct Game {
    current_state: Box<dyn GameState>,
    running: bool,
}

impl Game {
    fn new() -> Result<Self> {
        Ok(Game {
            current_state: Box::new(DuelState::new()?),
            running: true,
        })
    }

    fn update(&mut self) {
        let delta = get_frame_time();
        match self.current_state.update(delta) {
            UpdateResult::Continue => {}
            UpdateResult::Quit => {
                self.running = false
            }
        }
    }

    async fn render(&mut self) {
        self.current_state.render();
        next_frame().await;
    }
}


pub trait GameState {
    fn update(&mut self, dt: f32) -> UpdateResult;
    fn render(&mut self);
}

struct DuelState {
    hindsight: Session<PlayerInput>,
    game_time: FixedTimeLoop,
    input: LocalInput,
}

impl DuelState {
    fn new() -> Result<Self> {
        let mut session = Session::new(PlayerCount::Two);
        session.register_player(PlayerId::One, PlayerInfo::Local)?;
        session.register_player(PlayerId::Two, PlayerInfo::Local)?;

        Ok(Self {
            hindsight: session,
            game_time: FixedTimeLoop::new(1.0 / 60.0),
            input: LocalInput::new(),
        })
    }

    fn simulate_frame(&mut self, inputs: &[PlayerInput]) {}
}

impl GameState for DuelState {
    fn update(&mut self, dt: f32) -> UpdateResult {
        self.game_time.accumulate(dt);
        let mut sync_inputs: [PlayerInput; 2] = [PlayerInput::default(); 2];
        while self.game_time.step() {
            self.input.poll_input();
            for player_id in 0..2 {
                let player_input = self.input.get_input(player_id);
                self.hindsight.push_local_input(PlayerId::from(player_id), player_input.clone());
            }
            self.hindsight.synchronize_input(&mut sync_inputs).unwrap();
            self.simulate_frame(&mut sync_inputs);
            self.hindsight.frame_finished();
        }

        if is_key_down(KeyCode::Escape) {
            UpdateResult::Quit
        } else {
            UpdateResult::Continue
        }
    }

    fn render(&mut self) {
        clear_background(BLACK);
        draw_circle(screen_width() / 2.0, screen_height() / 2.0 - 30.0, 15.0, BLUE);
    }
}

