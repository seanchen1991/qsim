use rand;
use ndarray::{arr2, Array2};
use num_complex::Complex32;

use crate::gate::Gate;
use crate::registers::ClassicalRegister;

/// Wrapper type around a 2x1 matrix of complex numbers that represents 
/// the computational state of the qubit in superposition.
#[derive(Debug, Clone, PartialEq)]
pub struct Ket(Array2<Complex32>);

impl Ket {
    // pub fn new() -> Self {
    //     Self::default()
    // }
    
    /// Applies the given gate to the inner matrix by performing matrix multiplication.
    pub fn apply(&mut self, gate: Gate) -> Self {
        let old_ket = &self.0;
        Self(gate.inner.dot(old_ket))
    }
    
    /// Collapses the Ket into a ClassicalRegister. A Ket may only be collapsed once.
    ///
    /// The algorithm chooses a random float between 0 and 1, partitions 
    /// `[0, 1 + epsilon)` using the Ket coefficients' square modulii. 
    /// Then it randomly chooses one of the coefficients and returns the 
    /// matching state. 
    pub fn measure(&mut self) -> ClassicalRegister {
        let sample = rand::random::<f32>() % 1.0;
        let mut acc = 0_f32;

        ClassicalRegister::from(0)
    }
}

impl Default for Ket {
    /// `Ket` defaults to the |0> computational basis state.
    fn default() -> Self {
        Self(
            arr2(&[
                [Complex32::new(1.0, 0.0)],
                [Complex32::new(0.0, 0.0)],
            ])
        )
    }
}

