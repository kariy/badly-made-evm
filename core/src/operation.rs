use core::fmt;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum OperationError {
    #[error("unsupported operation {0}")]
    InvalidOperation(u8),
    #[error("expect PUSH operation followed by a value : {0}")]
    PushValueExpected(OpCode),
    #[error("JUMP destination must be a JUMPDEST instruction")]
    JumpDestExpected,
}

#[derive(Debug)]
pub enum OpCode {
    // Stop and Arithmetic Operations
    STOP,
    ADD,
    MUL,
    SUB,
    DIV,
    SDIV,
    MOD,
    SMOD,
    ADDMOD,
    MULMOD,
    EXP,
    // Comparison & Bitwise Logic Operations
    LT,
    GT,
    SLT,
    SGT,
    EQ,
    ISZERO,
    AND,
    OR,
    XOR,
    NOT,
    BYTE,
    SHL,
    SHR,
    // SHA3
    SHA3,
    // Environmental Information
    ADDRESS,
    BALANCE,
    CALLER,
    CALLVALUE,
    CODESIZE,
    CODECOPY,
    // Block Information
    SELFBALANCE,
    // Stack Memory Storage and Flow Operations
    POP,
    MLOAD,
    MSTORE,
    MSTORE8,
    JUMP,
    JUMPI,
    JUMPDEST,
    PC,
    MSIZE,
    // Push Operations
    PUSH1,
    PUSH2,
    PUSH3,
    // Duplication Operations
    DUP1,
    // Exchange Operations
    SWAP1,
    // LOG0,
    LOG1,
    RETURN,

    INVALID,
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::STOP,
            0x01 => Self::ADD,
            0x02 => Self::MUL,
            0x03 => Self::SUB,
            0x04 => Self::DIV,
            0x05 => Self::SDIV,
            0x06 => Self::MOD,
            0x07 => Self::SMOD,
            0x08 => Self::ADDMOD,
            0x09 => Self::MULMOD,
            0x0A => Self::EXP,

            0x10 => Self::LT,
            0x11 => Self::GT,
            0x12 => Self::SLT,
            0x13 => Self::SGT,
            0x14 => Self::EQ,
            0x15 => Self::ISZERO,
            0x16 => Self::AND,
            0x17 => Self::OR,
            0x18 => Self::XOR,
            0x19 => Self::NOT,
            0x1A => Self::BYTE,
            0x1B => Self::SHR,
            0x1C => Self::SHL,

            0x20 => Self::SHA3,

            0x30 => Self::ADDRESS,
            0x31 => Self::BALANCE,
            0x33 => Self::CALLER,
            0x34 => Self::CALLVALUE,
            0x38 => Self::CODESIZE,
            0x39 => Self::CODECOPY,

            0x47 => Self::SELFBALANCE,

            0x50 => Self::POP,
            0x51 => Self::MLOAD,
            0x52 => Self::MSTORE,
            0x53 => Self::MSTORE8,
            0x56 => Self::JUMP,
            0x57 => Self::JUMPI,
            0x58 => Self::PC,
            0x59 => Self::MSIZE,

            0x5B => Self::JUMPDEST,
            0x60 => Self::PUSH1,
            0x61 => Self::PUSH2,
            0x62 => Self::PUSH3,
            0x80 => Self::DUP1,
            0x90 => Self::SWAP1,
            // 0xA0 => Self::LOG0,
            0xA1 => Self::LOG1,
            0xF3 => Self::RETURN,

            0xFE | _ => Self::INVALID,
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::STOP => "STOP",
                Self::POP => "POP",
                Self::ADD => "ADD",
                Self::SUB => "SUB",
                Self::MUL => "MUL",
                Self::DIV => "DIV",
                Self::SDIV => "SDIV",
                Self::MOD => "MOD",
                Self::SMOD => "SMOD",
                Self::ADDMOD => "ADDMOD",
                Self::MULMOD => "MULMOD",
                Self::EXP => "EXP",

                Self::LT => "LT",
                Self::GT => "GT",
                Self::SLT => "SLT",
                Self::SGT => "SGT",
                Self::EQ => "EQ",
                Self::ISZERO => "ISZERO",
                Self::AND => "AND",
                Self::OR => "OR",
                Self::XOR => "XOR",
                Self::NOT => "NOT",
                Self::BYTE => "BYTE",
                Self::SHL => "SHL",
                Self::SHR => "SHR",

                Self::SHA3 => "SHA3",

                Self::ADDRESS => "ADDRESS",
                Self::BALANCE => "BALANCE",
                Self::CALLER => "CALLER",
                Self::CODESIZE => "CODESIZE",
                Self::CODECOPY => "CODECOPY",

                Self::SELFBALANCE => "SELFBALANCE",

                Self::MLOAD => "MLOAD",
                Self::MSTORE => "MSTORE",
                Self::MSTORE8 => "MSTORE8",
                Self::JUMP => "JUMP",
                Self::JUMPI => "JUMPI",
                Self::PC => "PC",
                Self::MSIZE => "MSIZE",
                Self::JUMPDEST => "JUMPDEST",

                Self::PUSH1 => "PUSH1",
                Self::PUSH2 => "PUSH2",
                Self::PUSH3 => "PUSH3",
                Self::DUP1 => "DUP1",
                Self::SWAP1 => "SWAP1",
                // Self::LOG0 => "LOG0",
                Self::LOG1 => "LOG1",
                Self::CALLVALUE => "CALLVALUE",
                Self::RETURN => "RETURN",

                Self::INVALID => "INVALID",
            }
        )
    }
}
