use ndarray::{self, Array2};

/// The |0> computational basis state.
const KET_0: &[&[i32]] = &[&[1], &[0]];

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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
