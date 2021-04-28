use std::fmt;

#[derive(Clone, Debug)]
pub struct ClassicalRegister {
    values: Vec<bool>,
}

impl fmt::Display for ClassicalRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.values.len() == 1 {
            write!(f, "{}", self.values[0])
        } else {
            write!(f, "{:?}", self.values)
        }
    }
}

impl Default for ClassicalRegister {
    fn default() -> Self {
        Self::new(1)
    }
}

impl ClassicalRegister {
    pub fn new(capacity: usize) -> Self {
        Self { values: vec![false; capacity] }
    }
}

#[derive(Debug)]
pub struct QuantumRegister {
    capacity: usize,
    collapsed: Option<bool>,

}
