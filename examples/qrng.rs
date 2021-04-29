use qsim::gate::Gate;
use qsim::quantum_computer::{QComputer, QuantumComputer};

fn qrng() -> u32 {
    let qc = QComputer::default();

    let mut qc = qc.apply(Gate::hadamard());

    qc.measure().into()
}

fn main() {
    for _ in 0..10 {
        println!("Qrng: {}", qrng());
    }
}
