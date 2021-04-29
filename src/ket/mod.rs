use ndarray::{arr2, Array2};
use num_complex::Complex32;

use crate::gate::Gate;
use crate::registers::ClassicalRegister;

#[derive(Debug, Clone)]
pub struct Ket(Array2<Complex32>);

impl Ket {
    // pub fn new() -> Self {
    //     Self::default()
    // }

    pub fn apply(&mut self, gate: Gate) -> Self {
        let old_ket = &self.0;
        Self(old_ket.dot(&gate))
    }

    pub fn measure(&mut self) -> ClassicalRegister {
        todo!();
    }
}

impl Default for Ket {
    /// `Ket` defaults to the |0> computation basis state.
    fn default() -> Self {
        Self(
            arr2(&[
                [Complex32::new(1.0, 0.0)],
                [Complex32::new(0.0, 0.0)],
            ])
        )
    }
}

