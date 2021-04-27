use crate::qubit::Qubit;

/// A type that hands out and reclaims qubits.
pub trait QuantumDevice<Q: Qubit> {
    /// Hands out an owned qubit.
    fn allocate(&mut self) -> Result<Q>;

    /// Reclaims a qubit.
    fn deallocate(&mut self, q: Q) -> Result<()>;
}
