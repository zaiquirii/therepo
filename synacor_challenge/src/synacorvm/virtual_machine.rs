use crate::synacorvm::operations;
use crate::synacorvm::operations::{Operand, Operation};
use crate::synacorvm::storage::{Memory, Stack};

pub struct VirtualMachine {
    instruction_counter: usize,
    running: bool,
    memory: Memory,
    stack: Stack,
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self::new(2_usize.pow(15) as usize, 1000)
    }
}

impl VirtualMachine {
    pub fn new(memory: usize, starting_stack: usize) -> Self {
        Self {
            instruction_counter: 0,
            running: false,
            memory: Memory::new(memory),
            stack: Stack::new(starting_stack),
        }
    }

    pub fn load_program_from_bytes(&mut self, data: &[u8]) {
        assert_eq!(data.len() % 2, 0);
        println!("Loading program bytes: {}", data.len());
        self.memory.push_program_bytes(data);
        println!("Success");
    }

    pub fn run(&mut self) -> operations::Result<()> {
        self.instruction_counter = 0;
        self.running = true;
        while self.running {
            self.step()?;
        }
        Ok(())
    }

    pub fn step(&mut self) -> operations::Result<()> {
        // Load operation
        // Execute operation
        let mut jumped = false;
        let op_start = self.memory.start_at(self.instruction_counter);
        let op = Operation::from(op_start)?;
        match &op {
            Operation::Halt => {
                println!("Shutting down");
                self.running = false
            }
            Operation::Jmp { tgt } => {
                jumped = true;
                self.jump_to(tgt)
            }
            Operation::Jt { src, tgt } => {
                if self.value_of(src) != 0 {
                    jumped = true;
                    self.jump_to(tgt);
                }
            }
            Operation::Jf { src, tgt } => {
                if self.value_of(src) == 0 {
                    jumped = true;
                    self.jump_to(tgt);
                }
            }
            Operation::Out { src } => {
                let c = self.value_of(src);
                let o = char::from_u32(c as u32)
                    .expect("This should be a valid ascii value");
                print!("{}", o);
            }
            Operation::Noop => { /* Do Nothing */ }
        }
        if !jumped {
            self.instruction_counter += op.instr_len();
        }
        Ok(())
    }

    fn value_of(&self, src: &Operand) -> u16 {
        match src {
            Operand::Literal { value } => { *value }
        }
    }

    fn jump_to(&mut self, target: &Operand) {
        self.instruction_counter = self.value_of(target) as usize;
    }

    fn incr_instruction(&mut self) {
        self.instruction_counter += 1;
    }
}