use std::fmt::{Display, Formatter};
use macroquad::color::BLACK;
use macroquad::prelude::*;
use crate::hindsight::{HindsightSession, PlayerCount, PlayerId, PlayerInfo};

mod hindsight;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("highsight had an issue")]
    Network(#[from] hindsight::Error)
}

type Result<T> = std::result::Result<T, Error>;

#[macroquad::main("hindsight")]
async fn main() -> Result<()> {
    let mut game = Game::new()?;
    loop {
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
}

struct Game {
    current_state: Box<dyn GameState>,
}

impl Game {
    fn new() -> Result<Self> {
        Ok(Game {
            current_state: Box::new(DuelState::new()?)
        })
    }

    fn update(&mut self) {
        let delta = get_frame_time();
        self.current_state.update(delta);
    }

    async fn render(&mut self) {
        self.current_state.render();
        next_frame().await;
    }
}

#[derive(Default)]
struct PlayerInput {
    left: bool,
    right: bool,
    jump: bool,
    shoot: bool,
}

pub trait GameState {
    fn update(&mut self, dt: f32);
    fn render(&mut self);
}

struct DuelState {
    game_time: FixedTimeLoop,
    input: DuelInputController,
}

impl DuelState {
    fn new() -> Result<Self> {
        let mut session = HindsightSession::new(PlayerCount::Two);
        session.register_player(PlayerId::One, PlayerInfo::Local)?;
        session.register_player(PlayerId::Two, PlayerInfo::Local)?;

        Ok(Self {
            game_time: FixedTimeLoop::new(1.0 / 60.0),
            input: DuelInputController {
                session
            },
        })
    }
}

impl GameState for DuelState {
    fn update(&mut self, dt: f32) {}

    fn render(&mut self) {
        clear_background(BLACK);
        draw_circle(screen_width() / 2.0, screen_height() / 2.0 - 30.0, 15.0, BLUE);
    }
}

struct DuelInputController {
    session: HindsightSession<PlayerInput>,
}

struct FixedTimeLoop {
    accumulated_time: f32,
    delta_time: f32,
}

impl FixedTimeLoop {
    pub fn new(delta_time: f32) -> Self {
        Self {
            accumulated_time: 0.0,
            delta_time,
        }
    }

    pub fn accumulate(&mut self, time: f32) {
        self.accumulated_time += time;
    }

    pub fn step(&mut self) -> bool {
        if self.accumulated_time >= self.delta_time {
            self.accumulated_time -= self.delta_time;
            true
        } else {
            false
        }
    }
}
