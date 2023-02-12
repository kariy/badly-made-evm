use crate::{evm::GlobalEnvironment, operation::OpCode};

use std::{cell::RefCell, rc::Rc};

use color_eyre::{eyre::bail, Result};
use ethereum_types::{H160, U256};
use evm_components::ExecutionMachine;

#[derive(Debug, Default)]
pub struct ExecutionEnvironment {
    value: U256,
    caller: H160,
    gas_count: u128,
    calldata: Vec<u8>,
    contract_address: H160,
}

pub struct ExecutionContext {
    logs: Rc<RefCell<Vec<Vec<U256>>>>,
    global_env: Rc<GlobalEnvironment>,
    execution_env: ExecutionEnvironment,
    execution_machine: ExecutionMachine,
}

impl ExecutionContext {
    pub fn new(global_env: Rc<GlobalEnvironment>) -> Self {
        Self {
            global_env,
            logs: Rc::new(RefCell::new(Vec::new())),
            execution_machine: ExecutionMachine::new(),
            // temporary
            execution_env: ExecutionEnvironment::default(),
        }
    }

    pub fn run(&mut self, program: Vec<u8>) -> Result<Vec<u8>> {
        let mut return_value = Vec::new();

        while let Some(opcode) = program.get(self.execution_machine.pc.get()) {
            let operation = OpCode::from(*opcode);

            match operation {
                OpCode::STOP => return Ok(return_value),

                OpCode::ADD => {
                    let a = self.execution_machine.stack.pop()?;
                    let b = self.execution_machine.stack.pop()?;
                    let (res, _) = a.overflowing_add(b);
                    self.execution_machine.stack.push(res)?;
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::SUB => {
                    let a = self.execution_machine.stack.pop()?;
                    let b = self.execution_machine.stack.pop()?;
                    let (res, _) = a.overflowing_sub(b);
                    self.execution_machine.stack.push(res)?;
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::MUL => {
                    let a = self.execution_machine.stack.pop()?;
                    let b = self.execution_machine.stack.pop()?;
                    let (res, _) = a.overflowing_mul(b);
                    self.execution_machine.stack.push(res)?;
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::DIV => {
                    let a = self.execution_machine.stack.pop()?;
                    let b = self.execution_machine.stack.pop()?;
                    let res = a.checked_div(b).unwrap_or(U256::zero());
                    self.execution_machine.stack.push(res)?;
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::MOD => {
                    let a = self.execution_machine.stack.pop()?;
                    let b = self.execution_machine.stack.pop()?;
                    self.execution_machine.stack.push(a % b)?;
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::JUMP => {
                    let offset = self.execution_machine.stack.pop()?.as_usize();

                    // check jump destination must be the JUMPDEST opcode
                    let Some(opcode) =  program.get(offset) else {
                        bail!("index out of bounds for program offset")
                    };

                    match OpCode::from(*opcode) {
                        OpCode::JUMPDEST => self.execution_machine.pc.set_exact(offset),
                        _ => bail!("jump destination must be the JUMPDEST opcode"),
                    }
                }

                OpCode::PC => {
                    self.execution_machine
                        .stack
                        .push(U256::from(self.execution_machine.pc.get()))?;
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::JUMPDEST => {
                    // doesnt do anything
                }

                OpCode::PUSH1 => {
                    let Some(value) =  program.get(self.execution_machine.pc.get() + 1) else {
                        bail!("expect PUSH operation followed by a number : PUSH1")
                    };
                    self.execution_machine
                        .stack
                        .push(U256::from(*value))
                        .unwrap();
                    self.execution_machine.pc.increment_by(2);
                }

                OpCode::PUSH2 => {
                    let start = self.execution_machine.pc.get() + 1;
                    let end = start + 1;
                    let value = &program[start..=end];

                    self.execution_machine
                        .stack
                        .push(U256::from_big_endian(value))
                        .unwrap();
                    self.execution_machine.pc.increment_by(3);
                }

                OpCode::PUSH3 => {
                    let start = self.execution_machine.pc.get() + 1;
                    let end = start + 2;
                    let value = &program[start..=end];

                    self.execution_machine
                        .stack
                        .push(U256::from_big_endian(value))
                        .unwrap();
                    self.execution_machine.pc.increment_by(4);
                }

                OpCode::POP => {
                    self.execution_machine.stack.pop()?;
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::DUP1 => {
                    let item = self.execution_machine.stack.get_from_top(0)?;
                    self.execution_machine.stack.push(item)?;
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::SWAP1 => {
                    let a = self.execution_machine.stack.get_from_top(0)?;
                    let b = self.execution_machine.stack.get_from_top(1)?;
                    self.execution_machine.stack.set_from_top(0, b)?;
                    self.execution_machine.stack.set_from_top(1, a)?;
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::MLOAD => {
                    let offset = self.execution_machine.stack.pop()?.as_usize();
                    let word = self
                        .execution_machine
                        .memory
                        .read_slice(offset, offset + 32);
                    self.execution_machine
                        .stack
                        .push(U256::from_big_endian(&word))?;
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::MSTORE => {
                    let offset = self.execution_machine.stack.pop()?;
                    let value = self.execution_machine.stack.pop()?;

                    let mut value_be = vec![0u8; 32];
                    value.to_big_endian(&mut value_be);

                    self.execution_machine
                        .memory
                        .set(offset.as_usize(), value_be);
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::LOG1 => {
                    let _offset = self.execution_machine.stack.pop()?.as_usize();
                    let _size = self.execution_machine.stack.pop()?;
                    let topic1 = self.execution_machine.stack.pop()?;

                    let log = vec![topic1];

                    self.logs.borrow_mut().push(log);
                    self.execution_machine.pc.increment_by(1);
                }
            }
        }

        Ok(return_value)
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
        assert_eq!(context.execution_machine.stack.height(), 1);
        assert_eq!(
            U256::from(0x09),
            context.execution_machine.stack.pop().unwrap()
        );
    }

    #[test]
    fn swap_operations() {
        let program = vec![0x60, 0x69, 0x60, 0x33, 0x90];
        let mut context = ExecutionContext::new(Rc::new(GlobalEnvironment {}));

        assert!(context.run(program).is_ok());
        assert_eq!(
            context.execution_machine.stack.get_from_top(0).unwrap(),
            U256::from(0x69)
        );
        assert_eq!(
            context.execution_machine.stack.get_from_top(1).unwrap(),
            U256::from(0x33)
        );
    }

    #[test]
    fn dup_operations() {
        let program = vec![0x60, 0x69, 0x60, 0x33, 0x80];
        let mut context = ExecutionContext::new(Rc::new(GlobalEnvironment {}));

        assert!(context.run(program).is_ok());
        assert_eq!(context.execution_machine.stack.height(), 3);
        assert_eq!(
            context.execution_machine.stack.get_from_top(0).unwrap(),
            U256::from(0x33)
        );
        assert_eq!(
            context.execution_machine.stack.get_from_top(1).unwrap(),
            U256::from(0x33)
        );
    }

    #[test]
    fn push_operations() {
        let program = vec![
            0x62, 0x42, 0x00, 0x69, 0x60, 0x33, 0x61, 0x00, 0x23, 0x60, 0x99,
        ];
        let mut context = ExecutionContext::new(Rc::new(GlobalEnvironment {}));

        assert!(context.run(program).is_ok());
        assert_eq!(context.execution_machine.stack.height(), 4);
        assert_eq!(
            context.execution_machine.stack.get_from_top(0).unwrap(),
            U256::from(0x99)
        );
        assert_eq!(
            context.execution_machine.stack.get_from_top(1).unwrap(),
            U256::from(0x0023)
        );
        assert_eq!(
            context.execution_machine.stack.get_from_top(3).unwrap(),
            U256::from(0x420069)
        );
    }

    #[test]
    fn memory_operations() {
        let program = vec![
            0x62, 0x00, 0x23, 0x44, 0x60, 0x00, 0x52, 0x60, 0x00, 0x51, 0x60, 0x00, 0x51,
        ];
        let mut context = ExecutionContext::new(Rc::new(GlobalEnvironment {}));

        assert!(context.run(program).is_ok());

        let value = context.execution_machine.memory.read_slice(0, 32);

        assert!(context.execution_machine.stack.height() == 2);
        assert!(32 == context.execution_machine.memory.used_capacity());
        assert_eq!(U256::from_big_endian(&value), U256::from(0x002344));
        assert_eq!(
            U256::from(0x002344),
            context.execution_machine.stack.get_from_top(0).unwrap()
        );
        assert_eq!(
            U256::from(0x002344),
            context.execution_machine.stack.get_from_top(1).unwrap()
        );
    }
}
