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
    POP,
    JUMP,
    PC,
    JUMPDEST,
    // Push Operations
    PUSH1,
    PUSH2,
    PUSH3,
    // Duplication Operations
    DUP1,
    // Exchange Operations
    SWAP1,
    MLOAD,
    MSTORE,
    MSTORE8,
    // LOG0,
    LOG1,
    CALLVALUE,
    RETURN,
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
            0x34 => Self::CALLVALUE,
            0x50 => Self::POP,
            0x51 => Self::MLOAD,
            0x52 => Self::MSTORE,
            0x53 => Self::MSTORE8,
            0x57 => Self::JUMP,
            0x58 => Self::PC,
            0x5B => Self::JUMPDEST,
            0x60 => Self::PUSH1,
            0x61 => Self::PUSH2,
            0x62 => Self::PUSH3,
            0x80 => Self::DUP1,
            0x90 => Self::SWAP1,
            // 0xA0 => Self::LOG0,
            0xA1 => Self::LOG1,
            0xF3 => Self::RETURN,
            _ => panic!("unsupported operation"),
        }
    }
}

impl Into<&'static str> for OpCode {
    fn into(self) -> &'static str {
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
            Self::JUMP => "JUMP",
            Self::PC => "PC",
            Self::JUMPDEST => "JUMPDEST",
            Self::PUSH1 => "PUSH1",
            Self::PUSH2 => "PUSH2",
            Self::PUSH3 => "PUSH3",
            Self::DUP1 => "DUP1",
            Self::SWAP1 => "SWAP1",
            Self::MLOAD => "MLOAD",
            Self::MSTORE => "MSTORE",
            Self::MSTORE8 => "MSTORE8",
            // Self::LOG0 => "LOG0",
            Self::LOG1 => "LOG1",
            Self::CALLVALUE => "CALLVALUE",
            Self::RETURN => "RETURN",
        }
    }
}
