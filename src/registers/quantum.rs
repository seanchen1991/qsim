use super::ClassicalRegister;

use crate::gate::Gate;
use crate::ket::Ket;

/// Represents the states a QuantumRegister may be in. A register may either
/// be in superposition, which means the inner `Ket` state may still be
/// affected by quantum gates, or collapsed, which means the inner `Ket`
/// state has been measured and yielded a `ClassicalRegister`.
#[derive(Debug, Clone)]
enum State {
    /// The superposition state; the inner `Ket` state of the quantum register can be
    /// affected by quantum gates.
    Superposition(Ket),
    /// The collapsed state; the inner `Ket` state of the quantum register has been
    /// measured, yielding a classical value.
    Collapsed(ClassicalRegister),
}

/// A register that holds on to the quantum system's qubits.
#[derive(Debug, Clone)]
pub struct QuantumRegister {
    state: State,
}

impl Default for QuantumRegister {
    /// QuantumRegister defaults to a single-qubit `Ket` state, which is a 2x1 matrix.
    fn default() -> Self {
        Self {
            state: State::Superposition(Ket::default()),
        }
    }
}

impl QuantumRegister {
    // pub fn new() -> Self {
    //     Self::default()
    // }

    /// Collapses the QuantumRegister into a ClassicalRegister.
    pub fn measure(&mut self) -> ClassicalRegister {
        match self.state {
            State::Collapsed(ref cr) => cr.to_owned(),
            State::Superposition(ref mut ket) => {
                let measurement = ket.measure();
                self.state = State::Collapsed(measurement.clone());
                measurement
            }
        }
    }

    /// Applies the given gate to the state of the QuantumRegister.
    pub fn apply(&mut self, gate: Gate) {
        match self.state {
            State::Superposition(ref mut ket) => {
                ket.apply(gate);
                self.state = State::Superposition(ket.to_owned());
            }
            State::Collapsed(_) => {}
        }
    }
}
