use crate::hindsight::{Error, FrameId, PlayerCount, PlayerId, PlayerInfo};
use crate::hindsight::input_buffer::InputBuffer;

struct PlayerHandle {}

const MAX_ROLLBACK_FRAMES: usize = 8;

enum SessionState {
    Initializing {
        players: [Option<PlayerInfo>; 2],
    },
    Running,
}

pub struct Session<PlayerInput: Default + Clone> {
    current_frame: FrameId,
    state: SessionState,
    input_buffers: Vec<InputBuffer<PlayerInput>>,
    player_count: PlayerCount,
    max_rollback: usize
}

impl<PlayerInput: Default + Clone> Session<PlayerInput> {
    pub fn new(player_count: PlayerCount, max_rollback: usize) -> Self {
        let mut input_buffers = Vec::new();
        for _ in 0..player_count as u8 {
            input_buffers.push(InputBuffer::new(max_rollback))
        }
        Self {
            current_frame: FrameId(0),
            state: SessionState::Initializing {
                players: [None, None]
            },
            input_buffers,
            player_count,
            max_rollback,
        }
    }

    pub fn register_player(&mut self, player_id: PlayerId, info: PlayerInfo) -> crate::hindsight::Result<()> {
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

    pub fn push_local_input(&mut self, player: PlayerId, input: PlayerInput) {
        self.input_buffers[player as usize].set_frame_input(self.current_frame, input)
    }

    pub fn synchronize_input(&mut self, out_inputs: &mut [PlayerInput]) -> Result<(), SyncInputError> {
        // println!("synchronizing input for frame: {:?}", self.current_frame);
        for i in 0..self.player_count as usize {
            match self.input_buffers[i].get_frame_input(self.current_frame) {
                None => return Err(SyncInputError::LocalInputMissing(PlayerId::from(i as u8))),
                Some(input) => {
                    out_inputs[i] = input.clone()
                }
            }
        }
        Ok(())
    }

    pub fn synchronize(&mut self) -> Option<Rollback> {
        let frames = self.max_rollback as u32;
        if self.current_frame.0 < frames{
            return None;
        }

        self.current_frame.0 -= frames;
        Some(Rollback {
            target_frame: self.current_frame,
            sim_frames: frames as u8,
        })
    }

    pub fn frame_finished(&mut self) {
        self.current_frame.0 += 1
    }
    pub fn current_frame(&self) -> FrameId {
        self.current_frame
    }
}

pub struct Rollback {
    pub target_frame: FrameId,
    pub sim_frames: u8,
}

#[derive(thiserror::Error, Debug)]
pub enum SyncInputError {
    #[error("Missing input for local player {0}")]
    LocalInputMissing(PlayerId),
}
