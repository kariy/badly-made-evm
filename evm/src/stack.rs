use color_eyre::{eyre, Result};
use ethereum_types::U256;

pub(crate) struct Stack {
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

    /// `index` cannot be bigger than stack height and cannot be negative
    pub fn set(&mut self, index: usize, value: U256) -> Result<U256> {
        let Some(pos) =  self.inner.get_mut(index) else {
            return Err(eyre::eyre!("index out of bound"));
        };

        *pos = value;
        Ok(value)
    }

    pub fn get(&self, index: usize) -> Option<U256> {
        self.inner.get(index).copied()
    }

    // return error if already at max capacity
    pub fn push(&mut self, value: U256) -> Result<()> {
        if self.inner.len() == self.max_capacity {
            return Err(eyre::eyre!("stack overflow"));
        }
        self.inner.push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Option<U256> {
        self.inner.pop()
    }

    pub fn height(&self) -> usize {
        self.inner.len()
    }

    pub fn max_height(&self) -> usize {
        self.max_capacity
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
        assert_eq!(stack.pop(), Some(U256::from(9)));
    }

    #[test]
    fn set_value_for_exact_stack_level() {
        let mut stack = Stack::new(10);
        stack.push(U256::from(1)).unwrap();
        stack.push(U256::from(1)).unwrap();
        stack.push(U256::from(1)).unwrap();
        stack.push(U256::from(9)).unwrap();
        stack.set(2, U256::from(128)).unwrap();

        let value = stack.get(2);
        assert_eq!(value, Some(U256::from(128)))
    }
}
