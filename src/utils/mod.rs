use ndarray::{arr2, Array2};
use num_complex::Complex32;

/// Returns the |0> computational basis state in matrix form.
pub fn zero_basis() -> Array2<Complex32> {
    arr2(&[
        [Complex32::new(1.0, 0.0)],
        [Complex32::new(0.0, 0.0)],
    ])
}

/// Returns the Hadamard gate in matrix form.
pub fn hadamard() -> Array2<Complex32> {
    let sqrt2 = 2.0_f32.sqrt();
    let mut gate = arr2(&[
        [Complex32::new(1.0, 0.0), Complex32::new(1.0, 0.0)],
        [Complex32::new(1.0, 0.0), Complex32::new(-1.0, 0.0)],
    ]);

    gate.map_mut(|c| c.unscale(sqrt2))
}
