use crate::synacorvm::operations::Operand::Literal;

#[derive(Debug)]
pub struct UnknownOpcode {
    code: u16,
}

#[derive(Debug)]
pub enum Operation {
    Halt,
    Out { src: Operand },
    Noop,
}

#[derive(Debug)]
pub enum Operand {
    Literal { value: u16 },
}

impl Operand {
    fn from_raw(raw: u16) -> Self {
        if raw >= 32768 {
            panic!("operand value too high");
        }

        Literal {
            value: raw,
        }
    }
}

impl Operation {
    pub fn instr_len(&self) -> usize {
        match self {
            Operation::Halt => 0,
            Operation::Noop => 1,
            Operation::Out { .. } => 2,
        }
    }
}

impl Operation {
    /// Attempt to parse operation starting at beginning of `raw`.
    pub fn from(raw: &[u16]) -> Result<Self, UnknownOpcode> {
        let op = match raw[0] {
            0 => Operation::Halt,
            19 => Operation::Out {
                src: Operand::from_raw(raw[1])
            },
            21 => Operation::Noop,
            code => return Err(UnknownOpcode { code }),
        };
        Ok(op)
    }
}