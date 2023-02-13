mod memory;
mod program_counter;
mod stack;

use memory::Memory;
use program_counter::ProgramCounter;
use stack::Stack;

pub struct ExecutionMachine {
    pub stack: Stack,
    pub memory: Memory,
    pub pc: ProgramCounter,
}

impl ExecutionMachine {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            pc: ProgramCounter::new(),
            stack: Stack::new(1024),
        }
    }
}
