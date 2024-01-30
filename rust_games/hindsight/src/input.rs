use macroquad::input::{is_key_down, KeyCode};

#[derive(Default, Copy, Clone)]
pub struct PlayerInput {
    left: bool,
    right: bool,
    jump: bool,
    shoot: bool,
}

impl PlayerInput {
    fn from_keycodes(codes: &[KeyCode; 4]) -> Self {
        Self {
            left: is_key_down(codes[0]),
            right: is_key_down(codes[1]),
            jump: is_key_down(codes[2]),
            shoot: is_key_down(codes[3]),
        }
    }
}

pub struct LocalInput {
    local_inputs: [PlayerInput; 2],
}

impl LocalInput {
    pub fn new() -> Self {
        Self {
            local_inputs: Default::default()
        }
    }
    pub fn poll_input(&mut self) {
        let p1_controls = [
            KeyCode::Left,
            KeyCode::Right,
            KeyCode::Up,
            KeyCode::Space
        ];

        let p2_controls = [
            KeyCode::A,
            KeyCode::D,
            KeyCode::W,
            KeyCode::F,
        ];

        self.local_inputs = [
            PlayerInput::from_keycodes(&p1_controls),
            PlayerInput::from_keycodes(&p2_controls),
        ]
    }

    pub fn get_input(&self, player: u8) -> &PlayerInput {
        &self.local_inputs[player as usize]
    }
}
