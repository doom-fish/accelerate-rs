# accelerate-rs

Safe Rust bindings for Apple's
[`Accelerate`](https://developer.apple.com/documentation/accelerate)
framework on macOS using a Swift bridge over the C APIs.

The GitHub repository is `accelerate-rs`; the published crates.io package is
`apple-accelerate`.

## Install

```bash
cargo add apple-accelerate
```

Enable the `raw-ffi` feature if you also want the underlying C declarations:

```bash
cargo add apple-accelerate --features raw-ffi
```

## Quick start

```rust,no_run
use apple_accelerate::{
    add_f32, integrate, solve_linear_system_f32, sdot, QuadratureOptions,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let added = add_f32(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0])?;
    assert_eq!(added, vec![5.0, 7.0, 9.0]);

    let dot = sdot(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0])?;
    assert_eq!(dot, 32.0);

    // LAPACK matrices are column-major.
    let solution = solve_linear_system_f32(&[3.0, 1.0, 1.0, 2.0], 2, &[9.0, 8.0])?;
    assert!((solution[0] - 2.0).abs() < 1.0e-5);
    assert!((solution[1] - 3.0).abs() < 1.0e-5);

    let integral = integrate(|x| x * x, 0.0, 1.0, QuadratureOptions::default())?;
    assert!((integral.integral - (1.0 / 3.0)).abs() < 1.0e-9);
    Ok(())
}
```

## v0.2.2 surface

- `vDSP`: FFT setup, biquad setup, vector arithmetic, reductions, and window generation
- `vForce`: element-wise transcendental and root functions over `f32` slices
- `BLAS`: `sdot`, row-major `sgemv`, and row-major `sgemm`
- `LAPACK`: LU factorization and linear solves for column-major single-precision matrices
- `BNNS`: safe ReLU/sigmoid vector activations plus the existing thin unsafe filter owner
- `Sparse`: sparse-vector dot products and sparse-to-dense accumulation
- `vImage`: ARGB8888 rotate / box-convolve / scale and Planar8 contrast stretch
- `simd`: SIMD4 add / dot / length / normalize helpers
- `Quadrature`: one-dimensional adaptive numerical integration with Rust closures
- `raw-ffi` feature: re-exports exhaustive audited C declarations for `vDSP`, `vForce`, `BLAS`, `LAPACK`, `BNNS`, `Sparse`, `vImage`, and the remaining vecLib helper families surfaced by the SDK audit

## Smoke examples

```bash
for ex in examples/*.rs; do
  cargo run --example "$(basename "$ex" .rs)"
done
```

The numbered examples cover every logical area of the crate.
