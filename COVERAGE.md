# apple-accelerate v0.4.0 coverage

This file lists what the safe wrappers cover for each Accelerate area. Each
area has one Rust module, one Swift bridge file, at least one example, and at
least one test. The safe surface is deliberately small; everything else is at
best reachable through the `raw-ffi` declarations described below.

| Area | Swift bridge | Rust module | Examples | Tests | Covered surface |
| --- | --- | --- | --- | --- | --- |
| vDSP | `swift-bridge/Sources/AppleAccelerateBridge/VDSP.swift` | `src/vdsp.rs` | `01_vdsp_fft.rs` | `tests/vdsp_tests.rs` | FFT setup and radix-2 `vDSP_fft_zip`, biquad setup/apply, add/sub/dot, min/max/mean/sum, Hamming/Blackman windows |
| vForce | `swift-bridge/Sources/AppleAccelerateBridge/VForce.swift` | `src/vforce.rs` | `03_vforce_transcendentals.rs` | `tests/vforce_tests.rs` | `sin`, `cos`, `exp`, `log`, `sqrt` over `f32` slices |
| BLAS | `swift-bridge/Sources/AppleAccelerateBridge/BLAS.swift` | `src/blas.rs` | `02_blas_vimage.rs`, `04_blas_linear_algebra.rs` | `tests/blas_tests.rs` | `sdot`, row-major `sgemv`, row-major `sgemm` |
| LAPACK | `swift-bridge/Sources/AppleAccelerateBridge/LAPACK.swift` | `src/lapack.rs` | `05_lapack_decompositions.rs` | `tests/lapack_tests.rs` | `sgetrf_` LU factorization, `sgesv_` linear solve |
| BNNS | `swift-bridge/Sources/AppleAccelerateBridge/BNNS.swift` | `src/bnns.rs` | `06_bnns_activation.rs` | `tests/bnns_tests.rs` | ReLU/sigmoid activations, BNNS Graph compile options (macOS 15+), unsafe owner for legacy filters |
| Sparse | `swift-bridge/Sources/AppleAccelerateBridge/Sparse.swift` | `src/sparse.rs` | `07_sparse_linear_algebra.rs` | `tests/sparse_tests.rs` | Sparse-vector dot products and add-to-dense, sparse matrix construction and triangular solves |
| vImage | `swift-bridge/Sources/AppleAccelerateBridge/VImage.swift` | `src/vimage.rs` | `02_blas_vimage.rs`, `08_vimage_processing.rs` | `tests/vimage_tests.rs` | Format-checked `ImageBuffer`; ARGB8888 rotate, box convolve, scale, alpha blend, clip, (un)premultiply; Planar8 contrast stretch; ARGB8888 ↔ Planar8 |
| simd | `swift-bridge/Sources/AppleAccelerateBridge/SIMD.swift` | `src/simd.rs` | `09_simd_math.rs` | `tests/simd_tests.rs` | Four-lane add, dot, length, normalize |
| Quadrature | `swift-bridge/Sources/AppleAccelerateBridge/Quadrature.swift` | `src/quadrature.rs` | `10_quadrature_integration.rs` | `tests/quadrature_tests.rs` | Closure-based one-dimensional integration with QNG, QAG, and QAGS |

## Raw FFI

The `raw-ffi` feature re-exports about 1,580 C function declarations through
`apple_accelerate::ffi`. They live in `src/ffi/` (hand-written) and
`src/ffi/generated/` (bindgen output refreshed by `tools/raw-ffi-gen`, with
layout assertions for every generated struct). Raw declarations are unsafe
to call and are not counted as safe coverage. Known holes:

- BLAS and LAPACK: only `cblas_sdot`, `cblas_sgemv`, `cblas_sgemm`, `sgetrf_`,
  and `sgesv_` are declared.
- 108 vecLib functions that pass C vector types by value (`vfp.h`,
  `vBasicOps.h`) are omitted because stable Rust cannot declare that calling
  convention. `COVERAGE_AUDIT.md` lists them.
- Some deprecated functions, such as the legacy BNNS layer constructors
  (`BNNSFilterCreateLayerActivation`, `BNNSFilterApplyBatch`), are not declared.

## Not wrapped

- BNNS Graph compile and execute: raw only. The Swift-only
  `BNNSGraph.Builder` (macOS 26) is not available.
- Sparse direct and iterative solvers (`SparseFactor`, `SparseSolve`, CG,
  GMRES, LSMR): raw only.
- vDSP DFT routines and the deprecated radix-3/5 FFTs (`vDSP_fft3_zop`,
  `vDSP_fft5_zop`): raw only.
- LAPACK beyond LU and solve; the wrappers use the CLAPACK interface that
  macOS 13.3 deprecated.
- Core ML's `MLTensor` is outside Accelerate and outside this crate.
