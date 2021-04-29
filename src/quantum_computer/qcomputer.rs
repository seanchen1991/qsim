use super::QuantumComputer;

use crate::gate::Gate;
use crate::registers::{ClassicalRegister, QuantumRegister};

/// Represents all of the possible states a quantum computer can be in.
#[derive(Debug, PartialEq, Eq)]
enum State {
    /// The quantum computer has been initialized; the qubits could be in any
    /// arbitrary state.
    Initialized,
    /// The quantum computer is running; the qubits could be in any
    /// arbitrary superposition.
    Running,
    /// The quantum computer's state has collapsed and yielded one or more
    /// classical values.
    Collapsed,
}

#[derive(Debug)]
pub struct QComputer {
    state: State,
    /// The number of qubits this quantum computer has at its disposal.
    capacity: usize,
    /// Register that holds the quantum computer's qubits.
    qregister: QuantumRegister,
    /// Register of classical values that only exists once the quantum register
    /// has been measured.
    cregister: Option<ClassicalRegister>,
}

impl Default for QComputer {
    fn default() -> Self {
        Self {
            capacity: 1,
            state: State::Initialized,
            qregister: QuantumRegister::default(),
            cregister: None,
        }
    }
}

impl QComputer {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            state: State::Initialized,
            qregister: QuantumRegister::default(),
            cregister: None,
        }
    }
}

impl QuantumComputer for QComputer {
    fn apply(self, gate: Gate) -> Self {
        assert_ne!(self.state, State::Collapsed);

        self.state = State::Running;
        self.qregister.apply(gate);

        self
    }

    fn measure(&mut self) -> ClassicalRegister {
        match self.cregister {
            Some(ref values) => values.to_owned(),
            None => {
                let values = self.qregister.measure();
                self.cregister = Some(values.clone());
                self.state = State::Collapsed;
                values
            }
        }
    }

    // Not sure if this method should initialize and return a new instance
    // or just reset the state of the current instance.
    // For now, just reset the state of the current instance.
    fn reset(&mut self) {
        self.state = State::Initialized;
    }
}
