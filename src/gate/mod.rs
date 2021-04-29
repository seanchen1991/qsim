use ndarray::{arr2, Array2};
use num_complex::{Complex, Complex32};

/// Representation of a quantum gate.
#[derive(Clone, Debug, PartialEq)]
pub struct Gate {
    pub inner: Array2<Complex32>,
}

impl Gate {
    /// Produces the Identity gate.
    pub fn identity() -> Self {
        let matrix = arr2(&[
            [Complex32::new(1.0, 0.0), Complex32::new(0.0, 0.0)],
            [Complex32::new(0.0, 0.0), Complex32::new(1.0, 0.0)],
        ]);

        Self { inner: matrix }
    }

    /// Produces the Hadamard gate.
    pub fn hadamard() -> Self {
        let sqrt2 = 2.0_f32.sqrt();
        let mut matrix = arr2(&[
            [Complex32::new(1.0, 0.0), Complex32::new(1.0, 0.0)],
            [Complex32::new(1.0, 0.0), Complex32::new(-1.0, 0.0)],
        ]);

        Self {
            inner: matrix.map_mut(|c| c.unscale(sqrt2)),
        }
    }

    /// Produces the Pauli-X gate, the analog of the classical NOT gate.
    pub fn pauli_x() -> Self {
        let matrix = arr2(&[
            [Complex32::new(0.0, 0.0), Complex32::new(1.0, 0.0)],
            [Complex32::new(1.0, 0.0), Complex32::new(0.0, 0.0)],
        ]);

        Self { inner: matrix }
    }

    /// Produces the Pauli-Y gate, the analog of the classical NOT gate.
    pub fn pauli_y() -> Self {
        let matrix = arr2(&[
            [Complex32::new(0.0, 0.0), -Complex::i()],
            [Complex::i(), Complex32::new(0.0, 0.0)],
        ]);

        Self { inner: matrix }
    }

    /// Produces the Pauli-Z gate, the analog of the classical NOT gate.
    pub fn pauli_z() -> Self {
        let matrix = arr2(&[
            [Complex32::new(1.0, 0.0), Complex32::new(0.0, 0.0)],
            [Complex32::new(0.0, 0.0), -Complex32::new(1.0, 0.0)],
        ]);

        Self { inner: matrix }
    }
}
