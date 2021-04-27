mod gate;
mod register;

pub mod qcomputer;

pub use gate::Gate;
pub use register::ClassicalRegister;

/// A QuantumComputer is the main way of working with a quantum device, whether
/// it be simulated or backed by actual quantum hardware.
pub trait QuantumComputer {
    /// Applies the given gate to the QuantumComputer.
    fn apply(&mut self, gate: Gate) -> Self;

    /// Collapses all of the qubits in the QuantumComputer down into a single
    /// classical value.
    fn measure(self) -> ClassicalRegister;

    /// Resets the state of the quantum computer instance.
    fn reset(&mut self) -> ();
}
