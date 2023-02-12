#[derive(Debug, Default)]
pub struct Memory(Vec<u8>);

impl Memory {
    pub fn new() -> Self {
        Self::default()
    }

    // if it goes beyond its current size, writes 0s
    pub fn read_slice(&self, from: usize, to: usize) -> Vec<u8> {
        let size = self.0.len();
        if to > size {
            [self.0[from..size].to_vec(), vec![0u8; to - size]].concat()
        } else {
            self.0[from..to].to_vec()
        }
    }

    // if the size of `value` + offset is bigger than the current memory size,
    // then expand memory first before putting in the value.
    pub fn set(&mut self, offset: usize, value: Vec<u8>) {
        let to = offset + value.len();
        if to > self.0.len() {
            self.0.resize_with(to, || 0)
        }
        self.0.splice(offset..to, value);
    }

    /// Returns the amount of memory used in bytes
    pub fn used_capacity(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_when_size_is_smaller_than_value() {
        let value: Vec<u8> = vec![4, 2, 0];
        let mut memory = Memory(vec![6, 9]);
        memory.set(1, value);
        assert_eq!(memory.0, &[6, 4, 2, 0]);
    }

    #[test]
    fn set_in_position_middle() {
        let value: Vec<u8> = vec![4, 2, 0];
        let mut memory = Memory(vec![6, 9, 1, 9, 7]);
        memory.set(1, value);
        assert_eq!(memory.0, &[6, 4, 2, 0, 7]);
    }

    #[test]
    fn read_slice_when_size_is_less_than_slice() {
        let memory = Memory(vec![6, 9, 1, 9, 7, 2, 4]);
        let value = memory.read_slice(5, 10);
        assert_eq!(value, vec![2, 4, 0, 0, 0])
    }

    #[test]
    fn read_word() {
        let memory = Memory(vec![6; 128]);
        assert_eq!(memory.read_slice(0, 32).len(), 32);
    }
}
