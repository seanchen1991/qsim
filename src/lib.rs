#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

mod error;
mod ket;

pub mod gate;
pub mod quantum_computer;
pub mod registers;
