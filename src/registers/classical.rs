use std::fmt;
use byteorder::{ByteOrder, BigEndian};

pub const REGISTER_SIZE: usize = 4;

#[derive(Clone, Debug)]
pub struct ClassicalRegister([u8; REGISTER_SIZE]);

impl Default for ClassicalRegister {
    fn default() -> Self {
        Self([0; REGISTER_SIZE])
    }
}

impl From<u32> for ClassicalRegister {
    fn from(n: u32) -> Self {
        let mut values = [0; 4];
        BigEndian::write_u32(&mut values, n);

        Self(values)
    }
}

impl From<&ClassicalRegister> for u32 {
    fn from(c: &ClassicalRegister) -> Self {
        let values = c.0.clone();
        u32::from_be_bytes(values.try_into().expect("Failed to convert ClassicalRegister to u32"))
    }
}
