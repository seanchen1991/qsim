pub mod qcomputer;

pub use qcomputer::*;
pub use crate::gate::Gate;
pub use crate::registers::ClassicalRegister;

/// A QuantumComputer is the main way of working with a quantum device, whether
/// it be simulated or backed by actual quantum hardware.
pub trait QuantumComputer {
    /// Applies the given gate to the QuantumComputer.
    fn apply(self, gate: Gate) -> Self;

    /// Collapses the qubits in the QuantumComputer down into classical values.
    fn measure(&mut self) -> ClassicalRegister;

    /// Resets the state of the quantum computer instance.
    fn reset(&mut self) -> ();
}
