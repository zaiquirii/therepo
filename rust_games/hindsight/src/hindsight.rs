use std::fmt::{Display, Formatter};

#[repr(u8)]
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


struct FrameId(u32);

struct PlayerHandle {}

enum SessionState {
    Initializing {
        players: [Option<PlayerInfo>; 2],
    },
    Running,
}

pub struct HindsightSession<PlayerInput: Default> {
    current_frame: FrameId,
    state: SessionState,
    input: PlayerInput,
}

impl<PlayerInput: Default> HindsightSession<PlayerInput> {
    pub fn new(player_count: PlayerCount) -> Self {
        Self {
            current_frame: FrameId(0),
            state: SessionState::Initializing {
                players: [None, None]
            },
            input: PlayerInput::default(),
        }
    }

    pub fn register_player(&mut self, player_id: PlayerId, info: PlayerInfo) -> Result<()> {
        match &mut self.state {
            SessionState::Running => {
                return Err(Error::InvalidOperation("Called register player while session is not in initializing state"));
            }
            SessionState::Initializing { players } => {
                let player = &mut players[player_id as usize];
                if player.is_some() {
                    return Err(Error::PlayerAlreadyRegistered(player_id));
                }
                let _ = player.insert(info);
                Ok(())
            }
        }
    }

    pub fn push_local_input(&mut self, player: PlayerId, input: PlayerInput) {}

    pub fn synchronize_input(&mut self, inputs: &mut [PlayerInput]) -> Result<()> {
        todo!()
    }

    pub fn frame_finished(&mut self) {}
}