use super::ClassicalRegister;

use crate::key::Ket;

#[derive(Debug, Clone)]
pub struct QuantumRegister {
    width: usize,
    collapsed: bool,
    ket: Ket,
}

impl QuantumRegister {
    /// Initializes a new QuantumRegister with `capacity` qubits.
    pub fn new(capacity: usize) -> Self {
        todo!();
    }

    /// Collapses the QuantumRegister into a ClassicalRegister.
    pub fn measure(&mut self) -> ClassicalRegister {
        todo!();
    }

    /// Applies the given gate to the state of the QuantumRegister.
    pub fn apply(&mut self, gate: Gate) {
        todo!();
    }
}
