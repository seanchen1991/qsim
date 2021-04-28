use num_complex::Complex;

pub const MAX_SIZE: usize = 32;

#[derive(Debug, Clone)]
pub struct Ket<const N: usize> {
    size: usize,
    coefficients: [Complex; N],
}

impl<const N: usize> Ket<{ N }> {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            coefficients: [Complex::zero(); MAX_SIZE]
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
        let size = 2usize.pow(c.values.len() as u32);
        let ket = Self::<MAX_SIZE>::new(size);

        ket.coefficients[u32::from(c) as usize] = Complex::one();

        ket
    }
}
