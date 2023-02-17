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
    CALLDATALOAD,
    CALLDATASIZE,
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
    PUSH(usize),
    // Duplication Operations
    DUP(usize),
    // Exchange Operations
    SWAP(usize),
    // LOG0,
    LOG(usize),
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
            0x35 => Self::CALLDATALOAD,
            0x36 => Self::CALLDATASIZE,

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

            0x60..=0x7F => Self::PUSH((value - 0x5F) as usize),

            0x80..=0x8F => Self::DUP((value - 0x7F) as usize),

            0x90..=0x9F => Self::SWAP((value - 0x8F) as usize),
            // 0xA0 => Self::LOG0,
            0xA0..=0xA4 => Self::LOG((value - 0xA0) as usize),

            0xF3 => Self::RETURN,

            0xFE => Self::INVALID,
            _ => Self::INVALID,
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
                Self::CALLDATALOAD => "CALLDATALOAD",
                Self::CALLDATASIZE => "CALLDATASIZE",

                Self::SELFBALANCE => "SELFBALANCE",

                Self::MLOAD => "MLOAD",
                Self::MSTORE => "MSTORE",
                Self::MSTORE8 => "MSTORE8",
                Self::JUMP => "JUMP",
                Self::JUMPI => "JUMPI",
                Self::PC => "PC",
                Self::MSIZE => "MSIZE",
                Self::JUMPDEST => "JUMPDEST",

                Self::PUSH(amount) => Box::leak(Box::new(format!("PUSH{}", amount))),

                Self::DUP(amount) => Box::leak(Box::new(format!("DUP{}", amount))),

                Self::SWAP(amount) => Box::leak(Box::new(format!("SWAP{}", amount))),

                Self::LOG(amount) => Box::leak(Box::new(format!("LOG{}", amount))),

                Self::CALLVALUE => "CALLVALUE",
                Self::RETURN => "RETURN",

                Self::INVALID => "INVALID",
            }
        )
    }
}

#[macro_export]
macro_rules! construct_push_op {
    ($a:expr, $self:expr, $program:expr) => {{
        let start = $self.execution_machine.pc.get() + 1;
        let end = start + ($a - 1);
        let value = &$program[start..=end];

        $self
            .execution_machine
            .stack
            .push(U256::from_big_endian(value))
            .unwrap();
        $self.execution_machine.pc.increment_by(1 + $a);
    }};
}

#[macro_export]
macro_rules! construct_dup_op {
    ($a:expr, $self:expr) => {{
        let item = $self.execution_machine.stack.get_from_top($a - 1)?;
        $self.execution_machine.stack.push(item)?;
        $self.execution_machine.pc.increment_by(1);
    }};
}

#[macro_export]
macro_rules! construct_swap_op {
    ($a:expr, $self:expr) => {{
        let a = $self.execution_machine.stack.get_from_top(0)?;
        let b = $self.execution_machine.stack.get_from_top($a)?;
        $self.execution_machine.stack.set_from_top(0, b)?;
        $self.execution_machine.stack.set_from_top(1, a)?;
        $self.execution_machine.pc.increment_by(1);
    }};
}

#[macro_export]
macro_rules! construct_log_op {
    ($a:expr, $self:expr) => {{
        let _ = $self.execution_machine.stack.pop()?;
        let _ = $self.execution_machine.stack.pop()?;

        let mut log: Vec<U256> = Vec::new();

        for _ in 0..$a {
            let topic = $self.execution_machine.stack.pop()?;
            log.push(topic);
        }

        $self.logs.borrow_mut().push(log);
        $self.execution_machine.pc.increment_by(1);
    }};
}
