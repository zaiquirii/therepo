use macroquad::miniquad::CursorIcon::Default;
use macroquad::prelude::*;
use crate::game::field::{Cell, FieldState, PlayingField, Rotation, Shape, Tetromino, TickResult};
use crate::game::input::UserAction;

pub struct GameConfig {
    pub min_user_input_time: f64,
    pub block_drop_time: f32,
    pub cell_size: f32,
}

pub const CONFIG: GameConfig = GameConfig {
    min_user_input_time: 0.1,
    block_drop_time: 0.5,
    cell_size: 20.0,
};

pub struct Game {
    field: PlayingField,
    next_block: Tetromino,
    last_block_drop: f32,
    last_user_input_time: f64,
}

impl Game {
    pub fn new(field_size: UVec2) -> Self {
        let mut g = Game {
            field: PlayingField::new(field_size),
            next_block: Tetromino {
                pos: IVec2::ZERO,
                shape: Shape::O,
                rot: 0,
                color: WHITE,
            },
            last_block_drop: 0.0,
            last_user_input_time: 0.0,
        };
        g.reset();
        g
    }

    fn reset(&mut self) {
        let s = self.field.size;
        self.field = PlayingField::new(s);
        self.update_next_block();
        self.field.active_block = Some(self.next_block);
        self.update_next_block();

    }

    pub fn handle_input(&mut self) {
        if let Some(a) = self.get_action() {
            match a {
                UserAction::Left => {
                    self.field.move_active_block(IVec2::NEG_X);
                }
                UserAction::Right => {
                    self.field.move_active_block(IVec2::X);
                }
                UserAction::Down => {
                    self.field.move_active_block(IVec2::Y);
                }
                UserAction::RotateLeft => {
                    self.field.rotate_active_block(Rotation::Left);
                }
                UserAction::RotateRight => {
                    self.field.rotate_active_block(Rotation::Right);
                }
                UserAction::DropBlock => {
                    let r = self.field.drop_active_block();
                    self.handle_tick_result(r);
                }
            }
        }
    }

    fn get_action(&mut self) -> Option<UserAction> {
        let current_time = get_time();
        let delta = current_time - self.last_user_input_time;
        if delta > CONFIG.min_user_input_time {
            if is_key_down(KeyCode::Right) {
                self.last_user_input_time = current_time;
                return Some(UserAction::Right);
            }
            if is_key_down(KeyCode::Left) {
                self.last_user_input_time = current_time;
                return Some(UserAction::Left);
            }
            if is_key_down(KeyCode::Down) {
                self.last_user_input_time = current_time;
                return Some(UserAction::Down);
            }
            if is_key_down(KeyCode::Space) {
                self.last_user_input_time = current_time;
                return Some(UserAction::DropBlock);
            }
            if is_key_down(KeyCode::Q) {
                self.last_user_input_time = current_time;
                return Some(UserAction::RotateLeft);
            }
            if is_key_down(KeyCode::W) {
                self.last_user_input_time = current_time;
                return Some(UserAction::RotateRight);
            }
        }
        None
    }

    pub fn update(&mut self, delta: f32) {
        self.last_block_drop += delta;
        match self.field.state {
            FieldState::Falling => {
                if self.last_block_drop >= CONFIG.block_drop_time {
                    let r = self.field.tick(delta);
                    self.last_block_drop = 0.0;
                    self.handle_tick_result(r);
                }
            }
            FieldState::ClearingLines { .. } => {
                let r = self.field.tick(delta);
                self.handle_tick_result(r);
            }
        }
    }
    fn handle_tick_result(&mut self, result: TickResult) {
        match result {
            TickResult::BlockLocked => {
                self.field.set_active_block(Some(self.next_block));
                self.update_next_block();
            }
            TickResult::LinesCleared(_) => {
                self.field.set_active_block(Some(self.next_block));
                self.update_next_block();
            }
            TickResult::GameOver => {
                self.reset()
            }
            TickResult::Updated => {// DO NOTHING }
            }
            TickResult::ClearingLines => {
                self.field.set_active_block(None)
            }
        }
    }

    fn update_next_block(&mut self) {
        let s = Shape::rand();
        // let s = Shape::O;
        self.next_block = Tetromino {
            pos: IVec2::new((self.field.size.x / 2 - 1) as i32, 1),
            shape: s,
            rot: 0,
            color: s.color(),
        }
    }

    pub fn render(&self) {
        self.field.render()
    }
}

