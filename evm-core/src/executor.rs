use crate::{evm::GlobalEnvironment, operation::OpCode};

use std::rc::Rc;

use color_eyre::{eyre::bail, Result};
use ethereum_types::U256;
use evm_components::{memory::Memory, program_counter::ProgramCounter, stack::Stack};

pub struct ExecutionContext {
    pc: ProgramCounter,
    stack: Stack,
    memory: Memory,
    global_env: Rc<GlobalEnvironment>,
}

impl ExecutionContext {
    pub fn new(global_env: Rc<GlobalEnvironment>) -> Self {
        Self {
            global_env,
            memory: Memory::new(),
            pc: ProgramCounter::new(),
            stack: Stack::new(1024),
        }
    }

    pub fn run(&mut self, program: Vec<u8>) -> Result<Vec<u8>> {
        let return_data: Vec<u8> = Vec::new();

        while let Some(opcode) = program.get(self.pc.get()) {
            let operation = Into::<OpCode>::into(*opcode);

            match operation {
                OpCode::STOP => return Ok(return_data),

                OpCode::ADD => {
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    let (res, _) = a.overflowing_add(b);
                    self.stack.push(res)?;
                    self.pc.increment_by(1);
                }

                OpCode::SUB => {
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    let (res, _) = a.overflowing_sub(b);
                    self.stack.push(res)?;
                    self.pc.increment_by(1);
                }

                OpCode::MUL => {
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    let (res, _) = a.overflowing_mul(b);
                    self.stack.push(res)?;
                    self.pc.increment_by(1);
                }

                OpCode::DIV => {
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    let res = a.checked_div(b).unwrap_or(U256::zero());
                    self.stack.push(res)?;
                    self.pc.increment_by(1);
                }

                OpCode::MOD => {
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    self.stack.push(a % b)?;
                    self.pc.increment_by(1);
                }

                OpCode::JUMP => {
                    let offset = self.stack.pop()?.as_usize();

                    // check jump destination must be the JUMPDEST opcode
                    let Some(opcode) =  program.get(offset) else {
                        bail!("index out of bounds for program offset")
                    };

                    match Into::<OpCode>::into(*opcode) {
                        OpCode::JUMPDEST => self.pc.set_exact(offset),
                        _ => bail!("jump destination must be the JUMPDEST opcode"),
                    }
                }

                OpCode::PC => {
                    self.stack.push(U256::from(self.pc.get()))?;
                    self.pc.increment_by(1);
                }

                OpCode::JUMPDEST => {
                    // doesnt do anything
                }

                OpCode::PUSH1 => {
                    let Some(value) =  program.get(self.pc.get() + 1) else {
                        bail!("expect PUSH operation followed by a number : PUSH1")
                    };
                    self.stack.push(U256::from(*value)).unwrap();
                    self.pc.increment_by(2);
                }

                OpCode::PUSH2 => {
                    let start = self.pc.get() + 1;
                    let end = start + 1;
                    let value = &program[start..=end];

                    self.stack.push(U256::from_big_endian(value)).unwrap();
                    self.pc.increment_by(3);
                }

                OpCode::PUSH3 => {
                    let start = self.pc.get() + 1;
                    let end = start + 2;
                    let value = &program[start..=end];

                    self.stack.push(U256::from_big_endian(value)).unwrap();
                    self.pc.increment_by(4);
                }

                OpCode::POP => {
                    self.stack.pop()?;
                    self.pc.increment_by(1);
                }

                OpCode::DUP1 => {
                    let item = self.stack.get_from_top(0)?;
                    self.stack.push(item)?;
                    self.pc.increment_by(1);
                }

                OpCode::SWAP1 => {
                    let a = self.stack.get_from_top(0)?;
                    let b = self.stack.get_from_top(1)?;
                    self.stack.set_from_top(0, b)?;
                    self.stack.set_from_top(1, a)?;
                    self.pc.increment_by(1);
                }

                OpCode::MLOAD => {
                    let offset = self.stack.pop()?.as_usize();
                    let word = self.memory.read_slice(offset, offset + 32);
                    self.stack.push(U256::from_big_endian(&word))?;
                    self.pc.increment_by(1);
                }

                OpCode::MSTORE => {
                    let offset = self.stack.pop()?;
                    let value = self.stack.pop()?;

                    let mut value_be = vec![0u8; 32];
                    value.to_big_endian(&mut value_be);

                    self.memory.set(offset.as_usize(), value_be);
                    self.pc.increment_by(1);
                }

                _ => unimplemented!("other opcodes"),
            }
        }

        return Ok(return_data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_arithmetic_operations() {
        let program = vec![
            0x60, 0x03, 0x60, 0x03, 0x01, 0x60, 0x03, 0x01, 0x60, 0x1B, 0x04, 0x60, 0x03, 0x02,
        ];
        let mut context = ExecutionContext::new(Rc::new(GlobalEnvironment {}));

        assert!(context.run(program).is_ok());
        assert_eq!(context.stack.height(), 1);
        assert_eq!(U256::from(0x09), context.stack.pop().unwrap());
    }

    #[test]
    fn swap_operations() {
        let program = vec![0x60, 0x69, 0x60, 0x33, 0x90];
        let mut context = ExecutionContext::new(Rc::new(GlobalEnvironment {}));

        assert!(context.run(program).is_ok());
        assert_eq!(context.stack.get_from_top(0).unwrap(), U256::from(0x69));
        assert_eq!(context.stack.get_from_top(1).unwrap(), U256::from(0x33));
    }

    #[test]
    fn dup_operations() {
        let program = vec![0x60, 0x69, 0x60, 0x33, 0x80];
        let mut context = ExecutionContext::new(Rc::new(GlobalEnvironment {}));

        assert!(context.run(program).is_ok());
        assert_eq!(context.stack.height(), 3);
        assert_eq!(context.stack.get_from_top(0).unwrap(), U256::from(0x33));
        assert_eq!(context.stack.get_from_top(1).unwrap(), U256::from(0x33));
    }

    #[test]
    fn push_operations() {
        let program = vec![
            0x62, 0x42, 0x00, 0x69, 0x60, 0x33, 0x61, 0x00, 0x23, 0x60, 0x99,
        ];
        let mut context = ExecutionContext::new(Rc::new(GlobalEnvironment {}));

        assert!(context.run(program).is_ok());
        assert_eq!(context.stack.height(), 4);
        assert_eq!(context.stack.get_from_top(0).unwrap(), U256::from(0x99));
        assert_eq!(context.stack.get_from_top(1).unwrap(), U256::from(0x0023));
        assert_eq!(context.stack.get_from_top(3).unwrap(), U256::from(0x420069));
    }

    #[test]
    fn memory_operations() {
        let program = vec![
            0x62, 0x00, 0x23, 0x44, 0x60, 0x00, 0x52, 0x60, 0x00, 0x51, 0x60, 0x00, 0x51,
        ];
        let mut context = ExecutionContext::new(Rc::new(GlobalEnvironment {}));

        assert!(context.run(program).is_ok());

        let value = context.memory.read_slice(0, 32);

        assert!(context.stack.height() == 2);
        assert!(32 == context.memory.used_capacity());
        assert_eq!(U256::from_big_endian(&value), U256::from(0x002344));
        assert_eq!(U256::from(0x002344), context.stack.get_from_top(0).unwrap());
        assert_eq!(U256::from(0x002344), context.stack.get_from_top(1).unwrap());
    }
}
