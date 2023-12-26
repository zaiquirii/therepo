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
    Set { dst: Operand, src: Operand },
    Push { src: Operand },
    Pop { dst: Operand },
    Eq { dst: Operand, lhs: Operand, rhs: Operand },
    Gt { dst: Operand, lhs: Operand, rhs: Operand },
    And { dst: Operand, lhs: Operand, rhs: Operand },
    Or { dst: Operand, lhs: Operand, rhs: Operand },
    Not { dst: Operand, src: Operand },
    Rmem { dst: Operand, src: Operand },
    Wmem { dst: Operand, src: Operand },
    Jmp { tgt: Operand },
    Jt { src: Operand, tgt: Operand },
    Jf { src: Operand, tgt: Operand },
    Add { dst: Operand, lhs: Operand, rhs: Operand },
    Mult { dst: Operand, lhs: Operand, rhs: Operand },
    Mod { dst: Operand, lhs: Operand, rhs: Operand },
    Call { tgt: Operand },
    Ret,
    Out { src: Operand },
    In { dst: Operand },
    Noop,
}

#[derive(Debug)]
pub enum Operand {
    Literal { value: u16 },
    Reg { index: u16 },
}

impl Operand {
    fn from_raw(raw: u16) -> Result<Self> {
        if raw <= 32767 {
            Ok(Operand::Literal { value: raw })
        } else if raw <= 32775 {
            Ok(Operand::Reg { index: raw - 32768 })
        } else {
            Err(OperandValueToHigh { value: raw })
        }
    }
}

impl Operation {
    pub fn instr_len(&self) -> usize {
        match self {
            Operation::Halt |
            Operation::Ret |
            Operation::Noop => {
                1
            }
            Operation::Out { .. } |
            Operation::In { .. } |
            Operation::Jmp { .. } |
            Operation::Push { .. } |
            Operation::Pop { .. } |
            Operation::Call { .. } => {
                2
            }
            Operation::Set { .. } |
            Operation::Jt { .. } |
            Operation::Jf { .. } |
            Operation::Not { .. } |
            Operation::Rmem { .. } |
            Operation::Wmem { .. } => {
                3
            }
            Operation::Add { .. } |
            Operation::Mult { .. } |
            Operation::Mod { .. } |
            Operation::Eq { .. } |
            Operation::Gt { .. } |
            Operation::And { .. } |
            Operation::Or { .. } => {
                4
            }
        }
    }
}

impl Operation {
    /// Attempt to parse operation starting at beginning of `raw`.
    pub fn from(raw: &[u16]) -> Result<Self> {
        let op = match raw[0] {
            0 => Operation::Halt,
            1 => Operation::Set {
                dst: Operand::from_raw(raw[1])?,
                src: Operand::from_raw(raw[2])?,
            },
            2 => Operation::Push {
                src: Operand::from_raw(raw[1])?,
            },
            3 => Operation::Pop {
                dst: Operand::from_raw(raw[1])?,
            },
            4 => Operation::Eq {
                dst: Operand::from_raw(raw[1])?,
                lhs: Operand::from_raw(raw[2])?,
                rhs: Operand::from_raw(raw[3])?,
            },
            5 => Operation::Gt {
                dst: Operand::from_raw(raw[1])?,
                lhs: Operand::from_raw(raw[2])?,
                rhs: Operand::from_raw(raw[3])?,
            },
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
            9 => Operation::Add {
                dst: Operand::from_raw(raw[1])?,
                lhs: Operand::from_raw(raw[2])?,
                rhs: Operand::from_raw(raw[3])?,
            },
            10 => Operation::Mult {
                dst: Operand::from_raw(raw[1])?,
                lhs: Operand::from_raw(raw[2])?,
                rhs: Operand::from_raw(raw[3])?,
            },
            11 => Operation::Mod {
                dst: Operand::from_raw(raw[1])?,
                lhs: Operand::from_raw(raw[2])?,
                rhs: Operand::from_raw(raw[3])?,
            },
            12 => Operation::And {
                dst: Operand::from_raw(raw[1])?,
                lhs: Operand::from_raw(raw[2])?,
                rhs: Operand::from_raw(raw[3])?,
            },
            13 => Operation::Or {
                dst: Operand::from_raw(raw[1])?,
                lhs: Operand::from_raw(raw[2])?,
                rhs: Operand::from_raw(raw[3])?,
            },
            14 => Operation::Not {
                dst: Operand::from_raw(raw[1])?,
                src: Operand::from_raw(raw[2])?,
            },
            15 => Operation::Rmem {
                dst: Operand::from_raw(raw[1])?,
                src: Operand::from_raw(raw[2])?,
            },
            16 => Operation::Wmem {
                dst: Operand::from_raw(raw[1])?,
                src: Operand::from_raw(raw[2])?,
            },
            17 => Operation::Call {
                tgt: Operand::from_raw(raw[1])?,
            },
            18 => Operation::Ret,
            19 => Operation::Out {
                src: Operand::from_raw(raw[1])?
            },
            20 => Operation::In {
                dst: Operand::from_raw(raw[1])?
            },
            21 => Operation::Noop,
            code => return Err(Error::UnknownOpcode { code }),
        };
        Ok(op)
    }
}