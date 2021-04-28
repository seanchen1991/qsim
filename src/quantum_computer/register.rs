use std::fmt;
use byteorder::{ByteOrder, BigEndian};

#[derive(Clone, Debug)]
pub struct ClassicalRegister {
    values: Vec<u8>,
}

impl ClassicalRegister {
    pub fn new(capacity: usize) -> Self {
        Self { values: vec![false; capacity] }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

impl Default for ClassicalRegister {
    fn default() -> Self {
        Self::new(1)
    }
}

impl From<u32> for ClassicalRegister {
    fn from(n: u32) -> Self {
        let mut buf = [0; 4];
        BigEndian::write_u32(&mut buf, n);

        Self { values: buf.to_vec() }
    }
}

impl From<&ClassicalRegister> for u32 {
    fn from(c: &ClassicalRegister) -> Self {
        let values = c.values.clone();
        u32::from_be_bytes(values.try_into().expect("Failed to convert ClassicalRegister to u32"))
    }
}

#[derive(Debug)]
pub struct QuantumRegister {
    capacity: usize,
    collapsed: Option<bool>,

}
