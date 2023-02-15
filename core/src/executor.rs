use crate::environment::{ExecutionResult, GlobalEnvironment};
use crate::operation::{OpCode, OperationError};
use crate::utils::convert_u256_to_eth_address;
use crate::{construct_dup_op, construct_push_op, construct_swap_op};

use std::{cell::RefCell, rc::Rc};

use color_eyre::{eyre, eyre::bail, Result};
use ethereum_types::{H160, U256};
use evm_components::ExecutionMachine;
use sha3::{Digest, Sha3_256};

// this should be something that user can set through the cli
#[derive(Debug, Default, Clone)]
pub struct ExecutionEnvironment {
    pub value: U256,
    pub caller: H160,
    pub calldata: Vec<u8>,
    pub contract_address: H160,
    // gas_count: u128,
}

#[derive(Default)]
pub struct ExecutionContext {
    // TODO: change logs format
    pub logs: Rc<RefCell<Vec<Vec<U256>>>>,
    pub global_env: Rc<GlobalEnvironment>,
    pub execution_env: ExecutionEnvironment,
    pub execution_machine: ExecutionMachine,
}

impl ExecutionContext {
    pub fn new(execution_env: ExecutionEnvironment, global_env: Rc<GlobalEnvironment>) -> Self {
        Self {
            global_env,
            execution_env,
            logs: Rc::new(RefCell::new(Vec::new())),
            execution_machine: ExecutionMachine::default(),
        }
    }

    pub fn run(&mut self, program: Vec<u8>) -> Result<ExecutionResult> {
        let mut result = Vec::new();

        while let Some(opcode) = program.get(self.execution_machine.pc.get()) {
            let operation = OpCode::from(*opcode);

            match operation {
                OpCode::STOP => return Ok(ExecutionResult { data: result }),

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

                OpCode::SHA3 => {
                    let offset = self.execution_machine.stack.pop()?.as_usize();
                    let size = self.execution_machine.stack.pop()?.as_usize();
                    let value = self.execution_machine.memory.read_bytes(offset, size);

                    let mut sha3 = Sha3_256::new();
                    sha3.update(&value);
                    let digest = sha3.finalize();

                    self.execution_machine
                        .stack
                        .push(U256::from_big_endian(digest.as_slice()))?;
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::ADDRESS => {
                    let address = self.execution_env.contract_address;

                    self.execution_machine
                        .stack
                        .push(U256::from(address.as_bytes()))?;
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::BALANCE => {
                    let address = convert_u256_to_eth_address(self.execution_machine.stack.pop()?);

                    if let Some(account) = self.global_env.global_storage.borrow().get(&address) {
                        self.execution_machine.stack.push(account.balance)?;
                    } else {
                        self.execution_machine.stack.push(U256::zero())?;
                    };

                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::SELFBALANCE => {
                    let address = self.execution_env.caller;

                    if let Some(account) = self.global_env.global_storage.borrow().get(&address) {
                        self.execution_machine.stack.push(account.balance)?;
                    } else {
                        self.execution_machine.stack.push(U256::zero())?;
                    };

                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::CALLER => {
                    let caller = self.execution_env.caller;

                    self.execution_machine
                        .stack
                        .push(U256::from(caller.as_bytes()))?;
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::CODESIZE => {
                    let code_size = program.len();
                    self.execution_machine.stack.push(U256::from(code_size))?;
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::CODECOPY => {
                    let dest_offset = self.execution_machine.stack.pop()?.as_usize();
                    let offset = self.execution_machine.stack.pop()?.as_usize();
                    let size = self.execution_machine.stack.pop()?.as_usize();
                    let code = &program[offset..(offset + size)];

                    self.execution_machine
                        .memory
                        .write_bytes(dest_offset, code.to_vec());
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
                        _ => return Err(eyre::eyre!(OperationError::JumpDestExpected)),
                    }
                }

                OpCode::JUMPI => {
                    let counter = self.execution_machine.stack.pop()?.as_usize();
                    let b = self.execution_machine.stack.pop()?;

                    if !b.is_zero() {
                        let Some(opcode) =  program.get(counter) else {
                            bail!("index out of bounds for program offset")
                        };

                        // check jump destination must be the JUMPDEST opcode
                        match OpCode::from(*opcode) {
                            OpCode::JUMPDEST => self.execution_machine.pc.set_exact(counter),
                            _ => return Err(eyre::eyre!(OperationError::JumpDestExpected)),
                        }
                    } else {
                        self.execution_machine.pc.increment_by(1)
                    }
                }

                OpCode::PC => {
                    self.execution_machine
                        .stack
                        .push(U256::from(self.execution_machine.pc.get()))?;
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::MSIZE => {
                    let size = self.execution_machine.memory.used_capacity();
                    self.execution_machine.stack.push(U256::from(size))?;
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::JUMPDEST => self.execution_machine.pc.increment_by(1),

                OpCode::PUSH(amount) => construct_push_op!(amount, self, program),

                OpCode::POP => {
                    self.execution_machine.stack.pop()?;
                    self.execution_machine.pc.increment_by(1);
                }

                OpCode::DUP(amount) => construct_dup_op!(amount, self),

                OpCode::SWAP(amount) => construct_swap_op!(amount, self),

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

                    let mut value_be = [0u8; 32];
                    value.to_big_endian(&mut value_be);

                    self.execution_machine
                        .memory
                        .write_bytes(offset.as_usize(), value_be.to_vec());
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

                    println!("{value:x?} {}", value.len());

                    for i in value.into_iter() {
                        result.push(i);
                    }

                    return Ok(ExecutionResult { data: result });
                }

                OpCode::INVALID => bail!(OperationError::InvalidOperation(*opcode)),
            }
        }

        Ok(ExecutionResult { data: result })
    }
}
