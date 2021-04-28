use ndarray::{arr2, Array2};
use num_complex::Complex32;

/// Representation of a quantum gate.
#[derive(Clone, Debug, PartialEq)]
pub struct Gate(Array2<Complex32>);

impl Gate {
    /// Produces the Identity gate.
    pub fn identity() -> Self {
        let matrix = arr2(&[
            [Complex32::new(1.0, 0.0), Complex32::new(0.0, 0.0)],
            [Complex32::new(0.0, 0.0), Complex32::new(1.0, 0.0)],
        ]);

        Self(matrix)
    }

    /// Produces the Hadamard gate.
    pub fn hadamard() -> Self {
        let sqrt2 = 2.0_f32.sqrt();
        let mut matrix = arr2(&[
            [Complex32::new(1.0, 0.0), Complex32::new(1.0, 0.0)],
            [Complex32::new(1.0, 0.0), Complex32::new(-1.0, 0.0)],
        ]);

        Self(matrix.map_mut(|c| c.unscale(sqrt2)))
    }
}
