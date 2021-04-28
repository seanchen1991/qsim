use crate::key::Ket;

pub struct QuantumRegister {
    width: usize,
    collapsed: bool,
    ket: Ket,
}
