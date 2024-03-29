pub struct Memory {
    data: Vec<u16>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size]
        }
    }

    pub fn start_at(&self, index: usize) -> &[u16] {
        &self.data[index..]
    }

    pub(crate) fn push_program_bytes(&mut self, program: &[u8]) {
        let mut i = 0;
        while i * 2 < program.len() {
            self.data[i] = as_u16_le(&program[i * 2..i * 2 + 2]);
            i += 1;
        }
    }

    pub fn get(&self, addr: u16) -> u16 {
        self.data[addr as usize]
    }

    pub fn set(&mut self, addr: u16, value: u16) {
        self.data[addr as usize] = value
    }
}

pub struct Stack {
    data: Vec<u16>,
}

impl Stack {
    pub fn new(capacity: usize) -> Self {
        Stack {
            data: Vec::with_capacity(capacity),
        }
    }
}

fn as_u16_le(data: &[u8]) -> u16 {
    data[0] as u16 | ((data[1] as u16) << 8)
}
