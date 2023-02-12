use ethereum_types::U256;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StackError {
    #[error("stack underflow")]
    StackUnderflow,
    #[error("stack overflow")]
    StackOverflow,
    #[error("index out of bounds")]
    IndexOutOfBounds,
}

type Result<T> = std::result::Result<T, StackError>;

pub struct Stack {
    max_capacity: usize,
    inner: Vec<U256>,
}

impl Stack {
    pub fn new(max_capacity: usize) -> Self {
        Self {
            max_capacity,
            inner: Vec::new(),
        }
    }

    /// `offset` the offset amount from the top of the stack
    pub fn set_from_top(&mut self, offset: usize, value: U256) -> Result<U256> {
        let index = (self.height() - 1) - offset;
        let pos = self
            .inner
            .get_mut(index)
            .ok_or_else(|| StackError::IndexOutOfBounds)?;

        *pos = value;
        Ok(value)
    }

    /// `offset` the offset amount from the top of the stack
    pub fn get_from_top(&self, offset: usize) -> Result<U256> {
        let index = (self.height() - 1) - offset;
        self.inner
            .get(index)
            .ok_or_else(|| StackError::IndexOutOfBounds)
            .copied()
    }

    // return error if already at max capacity
    pub fn push(&mut self, value: U256) -> Result<()> {
        if self.inner.len() == self.max_capacity {
            return Err(StackError::StackOverflow);
        }
        self.inner.push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<U256> {
        self.inner.pop().ok_or_else(|| StackError::StackUnderflow)
    }

    pub fn height(&self) -> usize {
        self.inner.len()
    }

    pub fn max_height(&self) -> usize {
        self.max_capacity
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_to_stack() {
        let mut stack = Stack::new(10);
        stack.push(U256::from(1)).unwrap();
        stack.push(U256::from(1)).unwrap();
        stack.push(U256::from(1)).unwrap();
        stack.push(U256::from(9)).unwrap();

        assert_eq!(stack.height(), 4);
        assert_eq!(stack.pop().ok(), Some(U256::from(9)));
    }

    #[test]
    fn set_value_for_exact_stack_level_from_top() {
        let mut stack = Stack::new(10);
        stack.push(U256::from(1)).unwrap();
        stack.push(U256::from(1)).unwrap();
        stack.push(U256::from(1)).unwrap();
        stack.push(U256::from(9)).unwrap();
        stack.set_from_top(1, U256::from(128)).unwrap();

        let value = stack.get_from_top(1).ok();
        assert_eq!(value, Some(U256::from(128)))
    }
}
