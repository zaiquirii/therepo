use std::collections::{HashMap, HashSet, VecDeque};
use std::io;
use crate::synacorvm::operations;
use crate::synacorvm::operations::{Operand, Operation};

struct State {
    instruction_counter: usize,
    running: bool,
    registers: [u16; 8],
    memory: Vec<u16>,
    stack: Vec<u16>,
    input: VecDeque<u16>,
}

pub struct VirtualMachine {
    instruction_counter: usize,
    running: bool,
    registers: [u16; 8],
    memory: Vec<u16>,
    stack: Vec<u16>,
    input: VecDeque<u16>,
    states: Vec<State>,
    commands: Vec<String>,
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self::new(2_usize.pow(15), 1000)
    }
}

impl VirtualMachine {
    pub fn new(memory: usize, starting_stack: usize) -> Self {
        Self {
            instruction_counter: 0,
            registers: [0; 8],
            running: false,
            memory: vec![0; memory],
            stack: Vec::new(),
            input: VecDeque::new(),
            states: Vec::new(),
            commands: Vec::new(),
        }
    }

    pub fn load_program_from_bytes(&mut self, data: &[u8]) {
        assert_eq!(data.len() % 2, 0);
        println!("Loading program bytes: {}", data.len());
        let mut i = 0;
        while i * 2 < data.len() {
            self.memory[i] = as_u16_le(&data[i * 2..i * 2 + 2]);
            i += 1;
        }
        println!("Success {}", self.memory[6147]);
        let x = 16357;
        let s = self.memory[x..(x+1000).min(self.memory.len())]
            .iter()
            .map(|x| char::from_u32(*x as u32).unwrap())
            .collect::<String>();
        println!("DATA AT: {} : {}", x, s);
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
        let mut jumped = false;
        let op = Operation::from(&self.memory[self.instruction_counter..])?;
        // Used to escape deadlock in challenge binary
        if self.instruction_counter == 6072 {
            self.registers[0] = 0;
            self.registers[1] = 5;
        }
        match &op {
            Operation::Halt => {
                if !self.pop_state() {
                    self.push_state();
                    println!("Shutting down");
                    self.running = false;
                }
                jumped = true;
            }
            Operation::Set { dst, src } => {
                self.set_register(dst, self.value_of(src))
            }
            Operation::Push { src } => {
                self.push_stack(self.value_of(src))
            }
            Operation::Pop { dst } => {
                let value = self.pop_stack().expect("Stack shouldn't be empty on pop");
                self.set_register(dst, value);
            }
            Operation::Eq { dst, lhs, rhs } => {
                let result = if self.value_of(lhs) == self.value_of(rhs) { 1 } else { 0 };
                self.set_register(dst, result);
            }
            Operation::Gt { dst, lhs, rhs } => {
                let result = if self.value_of(lhs) > self.value_of(rhs) { 1 } else { 0 };
                self.set_register(dst, result);
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
            Operation::Add { dst, lhs, rhs } => {
                let result = self.value_of(lhs) + self.value_of(rhs);
                self.set_register(dst, result % 32768);
            }
            Operation::Mult { dst, lhs, rhs } => {
                let result = self.value_of(lhs) as u32 * self.value_of(rhs) as u32;
                self.set_register(dst, (result % 32768) as u16);
            }
            Operation::Mod { dst, lhs, rhs } => {
                let result = self.value_of(lhs) % self.value_of(rhs);
                self.set_register(dst, result % 32768);
            }
            Operation::And { dst, lhs, rhs } => {
                let result = self.value_of(lhs) & self.value_of(rhs);
                self.set_register(dst, result);
            }
            Operation::Or { dst, lhs, rhs } => {
                let result = self.value_of(lhs) | self.value_of(rhs);
                self.set_register(dst, result);
            }
            Operation::Not { dst, src } => {
                let result = !self.value_of(src) & 0b0111111111111111;
                self.set_register(dst, result);
            }
            Operation::Rmem { dst, src } => {
                let addr = self.value_of(src);
                let val = self.memory[addr as usize];
                self.set_register(dst, val);
            }
            Operation::Wmem { dst, src } => {
                let val = self.value_of(src);
                let addr = self.value_of(dst);
                self.memory[addr as usize] = val;
            }
            Operation::Call { tgt } => {
                self.push_stack(self.instruction_counter as u16 + 2);
                self.jump_to(tgt);
                jumped = true;
            }
            Operation::Ret => {
                match self.pop_stack() {
                    None => {
                        println!("halted: stack is empty");
                        self.running = false;
                    }
                    Some(addr) => {
                        jumped = true;
                        self.instruction_counter = addr as usize;
                    }
                }
            }
            Operation::In { dst } => {
                let value = self.read_input();
                self.set_register(dst, value);
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

    fn set_register(&mut self, tgt: &Operand, value: u16) {
        match tgt {
            Operand::Literal { .. } => panic!("Invalid operation: Cannot set value of literal"),
            Operand::Reg { index } => self.registers[*index as usize] = value,
        }
    }

    fn value_of(&self, src: &Operand) -> u16 {
        match src {
            Operand::Literal { value } => { *value }
            Operand::Reg { index } => {
                self.registers[*index as usize]
            }
        }
    }

    fn push_stack(&mut self, value: u16) {
        self.stack.push(value);
    }

    fn pop_stack(&mut self) -> Option<u16> {
        self.stack.pop()
    }

    fn jump_to(&mut self, target: &Operand) {
        self.instruction_counter = self.value_of(target) as usize;
    }

    fn incr_instruction(&mut self) {
        self.instruction_counter += 1;
    }

    fn read_input(&mut self) -> u16 {
        if let Some(c) = self.input.pop_front() {
            return c;
        }

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Could not read stdin");
        if buffer != "commands\n" {
            self.commands.push(buffer.clone());
        }

        match buffer.as_str() {
            "save\n" => {
                self.push_state();
                buffer = "look\n".into();
            }
            "load\n" => {
                self.pop_state();
                buffer = "look\n".into();
            }
            "commands\n" => {
                println!("START COMMANDS");
                for s in &self.commands {
                    print!("{}", s);
                }
                println!("END COMMANDS");
            }
            "reg8\n" => {
                self.registers[7] = 25734;
                println!("Buffer set");
            }
            e => {}
        }
        buffer.bytes().for_each(|b| self.input.push_back(b as u16));
        self.input.pop_front().expect("Input should not be empty")
    }

    fn push_state(&mut self) {
        let state = State {
            instruction_counter: self.instruction_counter,
            running: self.running,
            registers: self.registers.clone(),
            memory: self.memory.clone(),
            stack: self.stack.clone(),
            input: self.input.clone(),
        };
        self.states.push(state);
        println!("State saved: {}", self.states.len());
    }

    fn pop_state(&mut self) -> bool {
        match self.states.pop() {
            None => {
                println!("No saved states to revert to");
                false
            }
            Some(s) => {
                self.instruction_counter = s.instruction_counter;
                self.running = s.running;
                self.registers = s.registers;
                self.memory = s.memory;
                self.stack = s.stack;
                self.input = s.input;
                println!("loaded states: states left : {}", self.states.len());
                true
            }
        }
    }

    pub fn dump_instructions(&self, start: usize) {
        let mut i = start;
        while i < self.memory.len() {
            if let Ok(up) = operations::print_op(i, &self.memory[i..]) {
                i += up
            } else {
                i += 1
            }
        }
    }
}

fn as_u16_le(data: &[u8]) -> u16 {
    data[0] as u16 | ((data[1] as u16) << 8)
}
