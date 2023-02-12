pub enum OpCode {
    STOP,
    ADD,
    MUL,
    SUB,
    DIV,
    MOD,
    POP,
    JUMP,
    PC,
    JUMPDEST,
    PUSH1,
    PUSH2,
    PUSH3,
    DUP1,
    SWAP1,
    MLOAD,
    MSTORE,
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::STOP,
            0x01 => Self::ADD,
            0x02 => Self::MUL,
            0x03 => Self::SUB,
            0x04 => Self::DIV,
            0x06 => Self::MOD,
            0x50 => Self::POP,
            0x51 => Self::MLOAD,
            0x52 => Self::MSTORE,
            0x57 => Self::JUMP,
            0x58 => Self::PC,
            0x5B => Self::JUMPDEST,
            0x60 => Self::PUSH1,
            0x61 => Self::PUSH2,
            0x62 => Self::PUSH3,
            0x80 => Self::DUP1,
            0x90 => Self::SWAP1,
            _ => panic!("unknown opcode"),
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
            Self::MOD => "MOD",
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
        }
    }
}
