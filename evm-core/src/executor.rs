use crate::{evm::GlobalEnvironment, operation::OpCode};

use std::{cell::RefCell, rc::Rc};

use color_eyre::{eyre::bail, Result};
use ethereum_types::{H160, U256};
use evm_components::ExecutionMachine;

#[derive(Debug, Default)]
pub struct ExecutionEnvironment {
    pub value: U256,
    pub caller: H160,
    pub calldata: Vec<u8>,
    pub contract_address: H160,
    // gas_count: u128,
}

pub struct ExecutionContext {
    pub logs: Rc<RefCell<Vec<Vec<U256>>>>,
    pub global_env: Rc<GlobalEnvironment>,
    pub execution_env: ExecutionEnvironment,
    pub execution_machine: ExecutionMachine,
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

                // technically we wont be accepting non-hex values, so...
                OpCode::DIV | OpCode::SDIV => {
                    let a = self.execution_machine.stack.pop()?;
                    let b = self.execution_machine.stack.pop()?;
                    let res = a.checked_div(b).unwrap_or(U256::zero());
                    self.execution_machine.stack.push(res)?;
                    self.execution_machine.pc.increment_by(1);
                }

                // technically we wont be accepting non-hex values, so...
                OpCode::MOD | OpCode::SMOD => {
                    let a = self.execution_machine.stack.pop()?;
                    let b = self.execution_machine.stack.pop()?;
                    self.execution_machine.stack.push(a % b)?;
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::ADDMOD => {
                    let a = self.execution_machine.stack.pop()?;
                    let b = self.execution_machine.stack.pop()?;
                    let n = self.execution_machine.stack.pop()?;

                    let (c, _) = a.overflowing_add(b);
                    let value = c % n;

                    self.execution_machine.stack.push(value)?;
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::MULMOD => {
                    let a = self.execution_machine.stack.pop()?;
                    let b = self.execution_machine.stack.pop()?;
                    let n = self.execution_machine.stack.pop()?;

                    let (c, _) = a.overflowing_mul(b);
                    let value = c % n;

                    self.execution_machine.stack.push(value)?;
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::EXP => {
                    let a = self.execution_machine.stack.pop()?;
                    let exponent = self.execution_machine.stack.pop()?;
                    let (value, _) = a.overflowing_pow(exponent);
                    self.execution_machine.stack.push(value)?;
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::LT | OpCode::SLT => {
                    let a = self.execution_machine.stack.pop()?;
                    let b = self.execution_machine.stack.pop()?;
                    let result = if a < b { U256::one() } else { U256::zero() };

                    self.execution_machine.stack.push(result)?;
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::GT | OpCode::SGT => {
                    let a = self.execution_machine.stack.pop()?;
                    let b = self.execution_machine.stack.pop()?;
                    let result = if a > b { U256::one() } else { U256::zero() };

                    self.execution_machine.stack.push(result)?;
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::EQ => {
                    let a = self.execution_machine.stack.pop()?;
                    let b = self.execution_machine.stack.pop()?;
                    let result = if a == b { U256::one() } else { U256::zero() };

                    self.execution_machine.stack.push(result)?;
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::ISZERO => {
                    let a = self.execution_machine.stack.pop()?;
                    let result = if a.is_zero() {
                        U256::one()
                    } else {
                        U256::zero()
                    };

                    self.execution_machine.stack.push(result)?;
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::AND => {
                    let a = self.execution_machine.stack.pop()?;
                    let b = self.execution_machine.stack.pop()?;
                    let result = a & b;

                    self.execution_machine.stack.push(result)?;
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::OR => {
                    let a = self.execution_machine.stack.pop()?;
                    let b = self.execution_machine.stack.pop()?;
                    let result = a | b;

                    self.execution_machine.stack.push(result)?;
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::XOR => {
                    let a = self.execution_machine.stack.pop()?;
                    let b = self.execution_machine.stack.pop()?;
                    let result = a ^ b;

                    self.execution_machine.stack.push(result)?;
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::NOT => {
                    let a = self.execution_machine.stack.pop()?;
                    let result = !a;

                    self.execution_machine.stack.push(result)?;
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::BYTE => {
                    let i = self.execution_machine.stack.pop()?.as_usize();
                    let x = self.execution_machine.stack.pop()?;

                    let mut bytes = Vec::new();
                    x.to_big_endian(&mut bytes);

                    self.execution_machine.stack.push(U256::from(bytes[i]))?;
                    self.execution_machine.pc.increment_by(1);
                }

                // SHR and SHL are inverted bcs U256 operates in little endian
                OpCode::SHL => {
                    let shift = self.execution_machine.stack.pop()?;
                    let value = self.execution_machine.stack.pop()?;
                    let result = value >> shift;

                    self.execution_machine.stack.push(result)?;
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::SHR => {
                    let shift = self.execution_machine.stack.pop()?;
                    let value = self.execution_machine.stack.pop()?;
                    let result = value << shift;

                    self.execution_machine.stack.push(result)?;
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
                    let word = self.execution_machine.memory.read_bytes(offset, 32);
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
                        .write_bytes(offset.as_usize(), value_be);
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::MSTORE8 => {
                    let offset = self.execution_machine.stack.pop()?.as_usize();
                    let value = self.execution_machine.stack.pop()?;
                    let byte = value.byte(31);
                    self.execution_machine
                        .memory
                        .write_bytes(offset, vec![byte]);
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::LOG1 => {
                    let _ = self.execution_machine.stack.pop()?.as_usize();
                    let _ = self.execution_machine.stack.pop()?;
                    let topic1 = self.execution_machine.stack.pop()?;

                    let log = vec![topic1];

                    self.logs.borrow_mut().push(log);
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::CALLVALUE => {
                    let callvalue = self.execution_env.value;
                    self.execution_machine.stack.push(callvalue)?;
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::RETURN => {
                    let offset = self.execution_machine.stack.pop()?.as_usize();
                    let size = self.execution_machine.stack.pop()?.as_usize();
                    let value = self.execution_machine.memory.read_bytes(offset, size);

                    for i in value.into_iter() {
                        return_value.push(i);
                    }

                    return Ok(return_value);
                }
            }
        }

        Ok(return_value)
    }
}
