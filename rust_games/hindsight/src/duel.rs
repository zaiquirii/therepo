use crate::input::PlayerInput;

const H_VEL: i32 = 2;

#[derive(Default, Debug, Copy, Clone)]
pub struct Character {
    pub location: glm::IVec2,
}

#[derive(Default, Clone)]
pub struct DuelSimulation {
    characters: [Character; 2],
}

impl DuelSimulation {
    pub fn new() -> Self {
        Self {
            characters: [Character::default(); 2]
        }
    }

    pub fn simulate_frame(&mut self, inputs: &[PlayerInput]) {
        move_character(&mut self.characters[0], &inputs[0]);
        move_character(&mut self.characters[1], &inputs[1]);
    }

    pub fn characters(&self) -> &[Character; 2] {
        &self.characters
    }
}

fn move_character(character: &mut Character, input: &PlayerInput) {
    let h_vel = if input.left {
        -H_VEL
    } else if input.right {
        H_VEL
    } else {
        0
    };
    character.location.x += h_vel;
}