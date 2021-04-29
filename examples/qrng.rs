use qsim::gate::Gate;
use qsim::quantum_computer::{
    QuantumComputer,
    QComputer,
};

fn qrng() -> u32 {
    let qc = QComputer::default();

    let mut qc = qc.apply(Gate::hadamard())
        .apply(Gate::pauli_x());

    qc.measure().into()
}

fn main() {
    for _ in 0..10 {
        println!("Qrng: {}", qrng());
    }
}

