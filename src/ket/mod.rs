use num_complex::Complex32;

use crate::gate::Gate;
use crate::register::ClassicalRegister;

pub const MAX_SIZE: usize = 32;

#[derive(Debug, Clone)]
pub struct Ket {
    size: usize,
    coefficients: [Complex32; MAX_SIZE],
}

impl Ket {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            coefficients: [Complex32::new(0.0, 0.0); MAX_SIZE],
        }
    }

    pub fn apply(&mut self, gate: Gate) {
        todo!();
    }
}

impl Default for Ket {
    fn default() -> Self {
        Self::new(2)
    }
}

impl From<&ClassicalRegister> for Ket {
    fn from(c: &ClassicalRegister) -> Self {
        let size = 2usize.pow(c.0.len() as u32);
        let ket = Self::new(size);

        ket.coefficients[u32::from(c) as usize] = Complex32::new(1.0, 0.0);

        ket
    }
}
