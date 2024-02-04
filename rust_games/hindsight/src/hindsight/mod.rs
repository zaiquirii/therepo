mod input_buffer;
mod frame_cache;
mod session;

use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use macroquad::telemetry::Frame;

pub use frame_cache::FrameCache;
pub use session::Session;
use crate::input::PlayerInput;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum PlayerCount {
    One = 1,
    Two = 2,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum PlayerId {
    One = 0,
    Two = 1,
}

impl From<u8> for PlayerId {
    fn from(value: u8) -> Self {
        match value {
            0 => PlayerId::One,
            1 => PlayerId::Two,
            _ => panic!("invalid value: {value}")
        }
    }
}

impl fmt::Display for PlayerId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        std::fmt::Display::fmt(&(*self as u8), f)
    }
}

pub enum PlayerInfo {
    Local
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid operation: {0}")]
    InvalidOperation(&'static str),
    #[error("player already registered: {0:?}")]
    PlayerAlreadyRegistered(PlayerId),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct FrameId(u32);

impl FrameId {
    pub fn next(&self) -> FrameId {
        FrameId(self.0 + 1)
    }
}