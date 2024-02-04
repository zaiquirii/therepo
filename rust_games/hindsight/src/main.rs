use std::fmt::{Display, Formatter};
use macroquad::color::BLACK;
use macroquad::prelude::*;
use crate::duel::DuelSimulation;
use crate::hindsight::{Session, PlayerCount, PlayerId, PlayerInfo, FrameCache, FrameId};
use crate::input::{LocalInput, PlayerInput};
use crate::time::FixedTimeLoop;

mod hindsight;
mod input;
mod time;
mod duel;

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
    simulation: DuelSimulation,
    game_time: FixedTimeLoop,
    input: LocalInput,
    state_buffer: FrameCache<DuelSimulation>,
}

impl DuelState {
    fn new() -> Result<Self> {
        let rollback_buffer = 120;
        let mut session = Session::new(PlayerCount::Two, rollback_buffer);
        session.register_player(PlayerId::One, PlayerInfo::Local)?;
        session.register_player(PlayerId::Two, PlayerInfo::Local)?;

        Ok(Self {
            simulation: DuelSimulation::new(),
            hindsight: session,
            game_time: FixedTimeLoop::new(1.0 / 60.0),
            input: LocalInput::new(),
            state_buffer: FrameCache::new(rollback_buffer)
        })
    }

    fn simulate_frame(&mut self, inputs: &[PlayerInput]) {
        self.simulation.simulate_frame(inputs)
    }

    fn save_state(&mut self, frame: FrameId) {
        let state = self.simulation.clone();
        self.state_buffer.set(frame, state);
    }
}

impl GameState for DuelState {
    fn update(&mut self, dt: f32) -> UpdateResult {
        self.game_time.accumulate(dt);
        let mut sync_inputs: [PlayerInput; 2] = [PlayerInput::default(); 2];
        while self.game_time.step() {
            let sync_result = self.hindsight.synchronize();
            if let Some(rollback) = sync_result {
                let prev_state = self.state_buffer.get_mut(rollback.target_frame);
                self.simulation = prev_state.clone();
                let mut target_frame = rollback.target_frame.next();
                for _ in 0..rollback.sim_frames {
                    self.hindsight.synchronize_input(&mut sync_inputs).unwrap();
                    self.simulate_frame(&mut sync_inputs);
                    self.save_state(target_frame);
                    target_frame = target_frame.next();
                    self.hindsight.frame_finished();
                }
            }

            self.input.poll_input();
            for player_id in 0..2 {
                let player_input = self.input.get_input(player_id);
                self.hindsight.push_local_input(PlayerId::from(player_id), player_input.clone());
            }
            self.hindsight.synchronize_input(&mut sync_inputs).unwrap();
            self.simulate_frame(&mut sync_inputs);
            self.save_state(self.hindsight.current_frame());
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

        for c in self.simulation.characters() {
            draw_circle(screen_width() / 2.0 + c.location.x as f32, screen_height() / 2.0 - 30.0, 15.0, BLUE);
        }
    }
}

