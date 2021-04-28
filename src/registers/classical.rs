#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use std::fmt;
use std::convert::TryInto;
use byteorder::{ByteOrder, BigEndian};

pub const REGISTER_SIZE: usize = 4;

#[derive(Clone, Debug)]
pub struct ClassicalRegister([u8; REGISTER_SIZE]);

impl Default for ClassicalRegister {
    fn default() -> Self {
        Self([0; REGISTER_SIZE])
    }
}

impl fmt::Display for ClassicalRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", BigEndian::read_u32(&self.0))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn double_conversion_is_identity(xs: u32) -> bool {
        let there = ClassicalRegister::from(xs);
        let back = u32::from(&there);

        xs == back
    }
}

