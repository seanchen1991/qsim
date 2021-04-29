use super::ClassicalRegister;

use crate::ket::Ket;
use crate::gate::Gate;

/// Represents the states a QuantumRegister may be in. A register may either
/// be in superposition, which means the inner `Ket` state may still be
/// affected by quantum gates, or collapsed, which means the inner `Ket`
/// state has been measured and yielded a `ClassicalRegister`.
#[derive(Debug, Clone)]
enum State {
    Superposition(Ket),
    Collapsed(ClassicalRegister),
}

#[derive(Debug, Clone)]
pub struct QuantumRegister {
    state: State
}

impl Default for QuantumRegister {
    fn default() -> Self {
        Self {
            state: State::Superposition(Ket::default())
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
            State::Superposition(ket) => {
                let measurement = ket.measure();
                self.state = State::Collapsed(measurement.clone());
                measurement
            }
        }
    }

    /// Applies the given gate to the state of the QuantumRegister.
    pub fn apply(&mut self, gate: Gate) {
        match self.state {
            State::Superposition(ket) => {
                self.state = State::Superposition(ket.apply(gate));
            }
            State::Collapsed(_) => {}
        }
    }
}
