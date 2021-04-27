use ndarray::{arr2, Array2};
use num_complex::Complex32;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Returns the |0> computational basis state in matrix form.
pub fn zero_basis() -> Array2<Complex32> {
    arr2(&[
        [Complex32::new(1.0, 0.0)],
        [Complex32::new(0.0, 0.0)],
    ])
}

/// Returns the Hadamard gate in matrix form.
pub fn hadamard() -> Array2<Complex32> {
    let sqrt2 = 2.0_f32.sqrt();
    let mut gate = arr2(&[
        [Complex32::new(1.0, 0.0), Complex32::new(1.0, 0.0)],
        [Complex32::new(1.0, 0.0), Complex32::new(-1.0, 0.0)],
    ]);

    gate.map_mut(|c| c.unscale(sqrt2))
}

/// An interface for a qubit
pub trait Qubit {
    /// Measures the qubit, comsuming it and returning a boolean.
    fn measure(self) -> Result<bool>;

    /// Applies the Hadamard gate to the qubit.
    fn h(&mut self) -> Result<()>;

    /// Resets the qubit to the |0> state.
    fn reset(&mut self) -> Result<()>;
}

/// A type that hands out and reclaims qubits.
pub trait QuantumDevice<Q: Qubit> {
    /// Hands out an owned qubit.
    fn allocate(&mut self) -> Result<Q>;

    /// Reclaims a qubit.
    fn deallocate(&mut self, q: Q) -> Result<()>;
}
