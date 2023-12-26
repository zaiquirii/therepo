use crate::synacorvm::operations::Operand::Literal;
use std::result;
use crate::synacorvm::operations::Error::OperandValueToHigh;

#[derive(Debug)]
pub enum Error {
    UnknownOpcode {
        code: u16,
    },
    OperandValueToHigh {
        value: u16,
    },
}

pub type Result<T> = result::Result<T, Error>;


#[derive(Debug)]
pub enum Operation {
    Halt,
    Jmp { tgt: Operand },
    Jt { src: Operand, tgt: Operand },
    Jf { src: Operand, tgt: Operand },
    Out { src: Operand },
    Noop,
}

#[derive(Debug)]
pub enum Operand {
    Literal { value: u16 },
}

impl Operand {
    fn from_raw(raw: u16) -> Result<Self> {
        if raw >= 32768 {
            return Err(OperandValueToHigh { value: raw });
        }

        Ok(Literal {
            value: raw,
        })
    }
}

impl Operation {
    pub fn instr_len(&self) -> usize {
        match self {
            Operation::Halt => 0,
            Operation::Noop => 1,
            Operation::Out { .. } | Operation::Jmp { .. } => 2,
            Operation::Jt { .. } | Operation::Jf { .. } => 3,
        }
    }
}

impl Operation {
    /// Attempt to parse operation starting at beginning of `raw`.
    pub fn from(raw: &[u16]) -> Result<Self> {
        let op = match raw[0] {
            0 => Operation::Halt,
            6 => Operation::Jmp {
                tgt: Operand::from_raw(raw[1])?,
            },
            7 => Operation::Jt {
                src: Operand::from_raw(raw[1])?,
                tgt: Operand::from_raw(raw[2])?,
            },
            8 => Operation::Jf {
                src: Operand::from_raw(raw[1])?,
                tgt: Operand::from_raw(raw[2])?,
            },
            19 => Operation::Out {
                src: Operand::from_raw(raw[1])?
            },
            21 => Operation::Noop,
            code => return Err(Error::UnknownOpcode { code }),
        };
        Ok(op)
    }
}