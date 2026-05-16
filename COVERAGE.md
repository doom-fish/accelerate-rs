# apple-accelerate v0.2.0 coverage audit

This audit tracks the requested v0.2.0 surface for `apple-accelerate`: one Rust
module, one Swift bridge file, at least one example, and at least one test for
all requested Accelerate logical areas.

| Area | Swift bridge | Rust module | Examples | Tests | Status | Covered surface |
| --- | --- | --- | --- | --- | --- | --- |
| vDSP | `swift-bridge/Sources/AppleAccelerateBridge/VDSP.swift` | `src/vdsp.rs` | `01_vdsp_fft.rs` | `tests/vdsp_tests.rs` | ✅ implemented | FFT setup/apply, biquad setup/apply, add/sub/dot, min/max/mean/sum, Hamming/Blackman windows |
| vForce | `swift-bridge/Sources/AppleAccelerateBridge/VForce.swift` | `src/vforce.rs` | `03_vforce_transcendentals.rs` | `tests/vforce_tests.rs` | ✅ implemented | `sin`, `cos`, `exp`, `log`, `sqrt` over `f32` slices |
| BLAS | `swift-bridge/Sources/AppleAccelerateBridge/BLAS.swift` | `src/blas.rs` | `02_blas_vimage.rs`, `04_blas_linear_algebra.rs` | `tests/blas_tests.rs` | ✅ implemented | `sdot`, row-major `sgemv`, row-major `sgemm` |
| LAPACK | `swift-bridge/Sources/AppleAccelerateBridge/LAPACK.swift` | `src/lapack.rs` | `05_lapack_decompositions.rs` | `tests/lapack_tests.rs` | ✅ implemented | `sgetrf_` LU factorization, `sgesv_` linear solve |
| BNNS | `swift-bridge/Sources/AppleAccelerateBridge/BNNS.swift` | `src/bnns.rs` | `06_bnns_activation.rs` | `tests/bnns_tests.rs` | ✅ implemented | Safe ReLU/sigmoid activation helpers plus the existing unsafe filter owner |
| Sparse | `swift-bridge/Sources/AppleAccelerateBridge/Sparse.swift` | `src/sparse.rs` | `07_sparse_linear_algebra.rs` | `tests/sparse_tests.rs` | ✅ implemented | Sparse-vector/dense-vector dot, sparse-vector/sparse-vector dot, sparse add-to-dense |
| vImage | `swift-bridge/Sources/AppleAccelerateBridge/VImage.swift` | `src/vimage.rs` | `02_blas_vimage.rs`, `08_vimage_processing.rs` | `tests/vimage_tests.rs` | ✅ implemented | ARGB8888 rotate / box-convolve / scale and Planar8 contrast stretch |
| simd | `swift-bridge/Sources/AppleAccelerateBridge/SIMD.swift` | `src/simd.rs` | `09_simd_math.rs` | `tests/simd_tests.rs` | ✅ implemented | SIMD4 add / dot / length / normalize |
| Quadrature | `swift-bridge/Sources/AppleAccelerateBridge/Quadrature.swift` | `src/quadrature.rs` | `10_quadrature_integration.rs` | `tests/quadrature_tests.rs` | ✅ implemented | Closure-based one-dimensional integration with QNG / QAG / QAGS options |

## Raw FFI

The raw Accelerate declarations remain available behind the `raw-ffi` feature.
They are organized per-area under `src/ffi/` and re-exported through
`apple_accelerate::ffi` when the feature is enabled.

## Deferred

- None for the requested nine-area v0.2.0 surface.
