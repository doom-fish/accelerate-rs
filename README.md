# accelerate-rs

Safe Rust bindings for Apple's
[`Accelerate`](https://developer.apple.com/documentation/accelerate)
framework on macOS using a Swift bridge over the C APIs.

The GitHub repository is `accelerate-rs`; the published crates.io package is
`apple-accelerate`.

## Requirements

- macOS 11 or later, the deployment target of the Swift bridge.
- `BnnsGraphCompileOptions` needs macOS 15; `BnnsGraphCompileOptions::new()`
  returns `None` on older systems.

## Install

```toml
[dependencies]
apple-accelerate = "0.4"
```

Enable the `raw-ffi` feature if you also want the underlying C declarations:

```toml
[dependencies]
apple-accelerate = { version = "0.4", features = ["raw-ffi"] }
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

## Surface

- `vDSP`: radix-2 FFT (`FftSetup::fft_zip`), biquad filtering, vector arithmetic, reductions, and Hamming/Blackman windows
- `vForce`: element-wise `sin`, `cos`, `exp`, `log`, and `sqrt` over `f32` slices
- `BLAS`: `sdot`, row-major `sgemv`, and row-major `sgemm`
- `LAPACK`: LU factorization (singular matrices are reported through `info()`) and linear solves for column-major single-precision matrices
- `BNNS`: ReLU/sigmoid vector activations, BNNS Graph compile options, and a thin unsafe owner for the legacy BNNS filters
- `Sparse`: sparse-vector dot products, sparse-to-dense accumulation, and a sparse matrix with triangular solves
- `vImage`: `ImageBuffer` with a checked `PixelFormat` and row stride; ARGB8888 rotate, box convolve, scale, alpha blend, clip, premultiply and unpremultiply, Planar8 contrast stretch, and ARGB8888 ↔ Planar8 conversion
- `simd`: four-lane add, dot, length, and normalize
- `Quadrature`: one-dimensional adaptive integration of Rust closures

## Raw FFI

The `raw-ffi` feature re-exports C declarations under `apple_accelerate::ffi`:
about 1,580 functions, mostly generated with bindgen from the vDSP, vForce,
vImage, BNNS, and Sparse headers, plus the vBigNum and BLAS threading helpers.
It does not cover the whole framework:

- BLAS and LAPACK are declared only for `cblas_sdot`, `cblas_sgemv`,
  `cblas_sgemm`, `sgetrf_`, and `sgesv_`.
- vecLib functions that pass C vector types such as `vFloat` or `vUInt32` by
  value (`vfloorf`, `vexpf`, `vU64Add`, and 105 others) are omitted, because
  stable Rust cannot declare the C vector calling convention. The array
  functions in vForce (`vvfloorf`, `vvexpf`, and so on) cover most of the
  floating-point ones.
- Some deprecated functions are absent, for example the legacy BNNS layer
  constructors such as `BNNSFilterCreateLayerActivation`.

`COVERAGE_AUDIT.md` explains what its coverage figures measure.

## Not wrapped

These have no safe wrapper; some are reachable through `raw-ffi`:

- BNNS Graph compilation and execution (`BNNSGraphCompileFromFile`,
  `BNNSGraphContextMake`, `BNNSGraphContextExecute`) are raw only, and the
  Swift-only `BNNSGraph.Builder` is not available at all.
- The Sparse solvers (`SparseFactor`, `SparseSolve`, and the CG, GMRES, and
  LSMR iterative methods) are raw only.
- The vDSP DFT routines and the radix-3/5 FFTs (`vDSP_fft3_zop`,
  `vDSP_fft5_zop`, deprecated since macOS 10.11) are raw only. `FftSetup`
  accepts radix-3/5 setups but only drives the radix-2 `vDSP_fft_zip`.
- LAPACK beyond LU factorization and solves. The wrappers call the CLAPACK
  interface, which Apple deprecated in macOS 13.3 in favour of the new LAPACK
  headers; moving to it would raise the minimum macOS version.
- Core ML's `MLTensor` is not part of Accelerate and is out of scope for this
  crate.

## Smoke examples

```bash
for ex in examples/*.rs; do
  cargo run --example "$(basename "$ex" .rs)"
done
```

The numbered examples cover every logical area of the crate.
