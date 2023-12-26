use crate::synacorvm::operations::{Operand, Operation, UnknownOpcode};
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

    pub fn run(&mut self) -> Result<(), UnknownOpcode> {
        self.instruction_counter = 0;
        self.running = true;
        while self.running {
            self.step()?;
        }
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), UnknownOpcode> {
        // Load operation
        // Execute operation
        let op_start = self.memory.start_at(self.instruction_counter);
        let op = Operation::from(op_start)?;
        match op {
            Operation::Halt => {
                println!("Shutting down");
                self.running = false
            }
            Operation::Out { ref src } => {
                let c = match src {
                    Operand::Literal { value } => {
                        char::from_u32(*value as u32).expect("This should be a valid ascii value")
                    }
                };
                print!("{}", c);
            }
            Operation::Noop => {}
        }
        self.instruction_counter += op.instr_len();
        Ok(())
    }

    fn incr_instruction(&mut self) {
        self.instruction_counter += 1;
    }
}