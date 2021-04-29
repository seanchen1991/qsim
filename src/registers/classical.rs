use byteorder::{BigEndian, ByteOrder};
use std::fmt;

pub const REGISTER_SIZE: usize = 4;

#[derive(Debug, Clone)]
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
        BigEndian::read_u32(&c.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::quickcheck;

    #[quickcheck]
    fn double_conversion_is_identity(xs: u32) -> bool {
        let there = ClassicalRegister::from(xs);
        let back = u32::from(&there);

        xs == back
    }
}
