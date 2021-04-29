use super::QuantumComputer;

use crate::gate::Gate;
use crate::registers::{ClassicalRegister, QuantumRegister};

/// Represents all of the possible states a quantum computer can be in.
#[derive(Debug, Clone)]
enum State {
    /// The quantum computer has been initialized; the qubits could be in any
    /// arbitrary state.
    Initialized,
    /// The quantum computer is running; the qubits could be in any
    /// arbitrary superposition.
    Running(QuantumRegister),
    /// The quantum computer's state has collapsed and yielded one or more
    /// classical values.
    Collapsed(ClassicalRegister),
}

#[derive(Debug)]
pub struct QComputer {
    /// The number of qubits this quantum computer has at its disposal.
    capacity: usize,
    /// The state the quantum computer is in.
    state: State,
}

impl Default for QComputer {
    fn default() -> Self {
        Self {
            capacity: 1,
            state: State::Initialized,
        }
    }
}

// impl QComputer {
//     pub fn new(capacity: usize) -> Self {
//         Self {
//             capacity,
//             state: State::Initialized,
//             qregister: QuantumRegister::new(),
//             cregister: None,
//         }
//     }
// }

impl QuantumComputer for QComputer {
    fn apply(self, gate: Gate) -> Self {
        match self.state {
            State::Collapsed(_) => {
                // log::info!("Can't apply a gate to an already measured quantum register");
                self
            }
            State::Initialized => {
                let mut qr = QuantumRegister::default();
                qr.apply(gate);
                Self {
                    capacity: self.capacity,
                    state: State::Running(qr),
                }
            }
            State::Running(mut qr) => {
                qr.apply(gate);
                Self {
                    capacity: self.capacity,
                    state: State::Running(qr),
                }
            }
        }
    }

    fn measure(&mut self) -> ClassicalRegister {
        match self.state {
            State::Initialized => ClassicalRegister::default(),
            State::Running(ref mut qr) => {
                let measurement = qr.measure();
                self.state = State::Collapsed(measurement.clone());
                measurement
            }
            State::Collapsed(ref cr) => cr.to_owned(),
        }
    }

    // Not sure if this method should initialize and return a new instance
    // or just reset the state of the current instance.
    // For now, just reset the state of the current instance.
    fn reset(&mut self) {
        self.state = State::Initialized;
    }
}
