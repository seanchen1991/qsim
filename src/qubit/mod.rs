use crate::error::Result;

/// An interface for a qubit
pub trait Qubit {
    /// Measures the qubit, comsuming it and returning a boolean.
    fn measure(self) -> Result<bool>;

    /// Applies the Hadamard gate to the qubit.
    fn h(&mut self) -> Result<()>;

    /// Resets the qubit to the |0> state.
    fn reset(&mut self) -> Result<()>;
}
