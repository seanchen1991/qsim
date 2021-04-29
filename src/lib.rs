#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

mod ket;
mod error;

pub mod gate;
pub mod registers;
pub mod quantum_computer;
