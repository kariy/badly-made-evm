mod memory;
mod program_counter;
mod stack;

use std::default;

use memory::Memory;
use program_counter::ProgramCounter;
use stack::Stack;

#[derive(Debug)]
pub struct ExecutionMachine {
    pub stack: Stack,
    pub memory: Memory,
    pub pc: ProgramCounter,
}

impl default::Default for ExecutionMachine {
    fn default() -> Self {
        Self {
            stack: Stack::new(1024),
            memory: Memory::default(),
            pc: ProgramCounter::default(),
        }
    }
}
