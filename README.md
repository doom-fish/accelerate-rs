# accelerate-rs

Safe Rust bindings for Apple's
[`Accelerate`](https://developer.apple.com/documentation/accelerate)
framework on macOS using pure C FFI.

The GitHub repository is `accelerate-rs`; the published crates.io package is
`apple-accelerate`.

## Install

```bash
cargo add apple-accelerate
```

## Quick start

```rust,no_run
use apple_accelerate::{add_f32, sgemm_row_major};

let added = add_f32(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]).expect("vector add");
assert_eq!(added, vec![5.0, 7.0, 9.0]);

let mut output = vec![0.0_f32; 4];
sgemm_row_major(
    2,
    2,
    3,
    1.0,
    &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
    &[7.0, 8.0, 9.0, 10.0, 11.0, 12.0],
    0.0,
    &mut output,
)
.expect("sgemm");
assert_eq!(output, vec![58.0, 64.0, 139.0, 154.0]);
```

## v0.1 surface

- Safe `vDSP` helpers for FFT setup, vector arithmetic, dot product, reductions, and window generation
- Safe `CBLAS` helpers for `sdot`, row-major `sgemv`, and row-major `sgemm`
- Safe `vImage` helpers for ARGB8888 rotate / box-convolve / scale and Planar8 contrast stretch
- Thin unsafe BNNS filter wrapper for convolution / fully connected create-apply-destroy workflows
- Raw `ffi` module exposing the underlying C types and functions

## Smoke examples

```bash
cargo run --example 01_vdsp_fft
cargo run --example 02_blas_vimage
```
