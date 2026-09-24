# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2026-09-24

### Security

- `sparse_add_to_dense_f32` and `sparse_dot_dense_f32` checked only the last
  index, so a strictly increasing list starting with a negative index (for
  example `[-5, 3]`) wrote or read the dense vector out of bounds. Every index
  is now checked to be non-negative and below the dense length, and the
  sparse-sparse product rejects negative indices too.
- The vImage wrappers accepted a Planar8 buffer where ARGB8888 was expected
  and wrote four times the row width at a one-byte stride, past the end of the
  buffer. `ImageBuffer` now carries its pixel format and every wrapper checks
  formats and row bytes before calling vImage.

### Fixed

- Zero dimensions no longer reach CBLAS or LAPACK with a leading dimension of
  0, which made the default CBLAS handler print an error and call `exit()`
  (for example `sgemv_row_major(3, 0, ..)`, `sgemm_row_major` with an inner
  dimension of 0, or `lu_decompose_f32(&[], 0)`). The bridge also passes
  `max(1, N)` leading dimensions.
- `solve_linear_system_f32` no longer panics dividing by zero for an empty
  system.
- The vForce wrappers no longer abort for slices longer than `i32::MAX`
  elements; they call vForce in chunks.
- `SparseMatrixF32` serializes its `&self` calls, so concurrent solves can no
  longer race on the implicit commit of pending inserts.
- A panicking quadrature integrand makes `integrate` return
  `Error::IntegrandPanicked` instead of `Ok` with a wrong integral, and the
  closure is not called again after it panicked.
- `FftSetup::new` no longer traps for `log2n` values that do not fit an `Int`
  and no longer lets `vDSP_create_fftsetup` crash for `log2n >= 62`;
  `FftSetup::fft_zip` rejects directions other than forward and inverse.
- The `ImageBuffer` constructors no longer overflow computing the row size
  when the height is 0, and sizes are bounded so the bridge cannot trap.
- `contrast_stretch_planar8` checks that both buffers have the same size.
- The raw `simd_float4` has the 16-byte alignment of the C type.
- `build.rs` no longer adds the toolchain's Swift 5.5 back-deployment
  directory (`usr/lib/swift-5.5/macosx`) to the link search path or rpath.
  Its old `libswift_Concurrency.dylib` could shadow the SDK's
  `libswift_Concurrency.tbd` for the whole binary and break linking next to
  Swift bridges that use newer concurrency APIs.

### Changed

- **Breaking:** `ImageBuffer` has a `PixelFormat`. The ARGB8888 alpha
  operations and the planar conversions accept only ARGB8888 and Planar8
  buffers; rotate, scale, and box convolve accept any interleaved 8888 order
  when source and destination agree.
- **Breaking:** `lu_decompose_f32` returns the factorization when `sgetrf_`
  reports a zero pivot; check `LuDecompositionF32::info()` or
  `is_singular()`. Only illegal-argument codes are errors.
- **Breaking:** `hamming_window`, `hamming_window_f64`, `blackman_window`, and
  `blackman_window_f64` return `Result<Vec<_>>`.
- **Breaking:** `FftSetup::new` returns `None` for `log2n` above 40 or a radix
  other than `fft_radix::RADIX2`, `RADIX3`, or `RADIX5`.
- **Breaking:** `Error` is `#[non_exhaustive]` and has an `IntegrandPanicked`
  variant.
- **Breaking:** the raw `ffi::quadrature_status` is an `i32` newtype with
  associated constants instead of an enum.
- `integrate` no longer requires a `'static` closure, and it rejects
  `max_intervals` values whose workspace size would overflow.
- `sgemv_row_major` and `sgemm_row_major` compute `y = beta * y` (or
  `C = beta * C`) themselves when the inner dimension is 0.
- Requires `apple-cf` 0.11 and `doom-fish-utils` 0.4.1; `rust-version` is
  1.82.
- The README and the coverage documents state what is covered and how the
  coverage figures are computed; the package description no longer claims
  exhaustive raw bindings.

### Added

- `PixelFormat` (Planar8, PlanarF, and ARGB, RGBA, and BGRA in 8888 and FFFF),
  `ImageBuffer::new`, `ImageBuffer::with_row_bytes`, and the `format`,
  `width`, `height`, and `row_bytes` accessors. `ImageBuffer` implements
  `Debug`.
- `LuDecompositionF32::info` and `LuDecompositionF32::is_singular`.
- Raw `vDSP_fft3_zop`, `vDSP_fft3_zopD`, `vDSP_fft5_zop`, and
  `vDSP_fft5_zopD` declarations.
- Compile-time layout assertions for the generated and hand-written raw
  structs.

### Removed

- **Breaking:** 108 raw vecLib declarations that passed C vector types by
  value (`vfp.h` functions such as `vfloorf` and `vBasicOps.h` functions such
  as `vU64Add`), and the unused `vFloat`, `vUInt8`, `vSInt8`, `vUInt16`,
  `vSInt16`, and `vSInt32` array aliases. Rust arrays use a different calling
  convention from C vectors, and stable Rust cannot declare SIMD types in
  `extern` blocks.
- The never-compiled `src/ffi/generated/vdsp.rs` and
  `src/ffi/generated/vforce.rs`.

## [0.3.3] - 2026-06-06

- Bounds-checked the vDSP biquad delay buffer and the FFT `log2n`, and caught
  panics in the quadrature trampoline.

## [0.3.2] - 2026-05-18

- chore: re-export OS primitives (Boolean) from apple-cf

## [0.3.1] - 2026-05-18

- Widen apple-cf version bound to `<0.10` so 0.9.x resolves.

## 0.3.0 - 2026-05-18

- Re-exported the raw `CFTypeRef` and `CFStringRef` vImage aliases from `apple-cf`
- Added `apple-cf` as a dependency for the shared Core Foundation raw types

## 0.2.4 - 2026-05-18

- Added API docs across the safe wrapper surface and leaked raw declarations, raising safe-surface public-item coverage to 100.0%
- Documented the Swift bridge helpers plus the non-generated raw FFI modules with one-line Accelerate references

## 0.2.3 - 2026-05-17

- Added missing SAFETY comments to unsafe blocks in `quadrature_trampoline` callback
- Added missing SAFETY comments to unsafe FFI calls in `bnns` constructors and apply methods

## 0.2.2 - 2026-05-17

- Expanded `raw-ffi` to the full audited `Accelerate`/`vecLib`/`vImage` surface for `vDSP`, `vForce`, `BLAS`, `LAPACK`, `BNNS`, `Sparse`, and `vImage`
- Added generated raw FFI supplements under `src/ffi/generated/` plus `ffi::veclib_extras` for the remaining vecLib helper families surfaced by the SDK audit
- Added the checked-in `tools/raw-ffi-gen` generator used to refresh the audited raw declarations
- Refreshed `COVERAGE.md` / `COVERAGE_AUDIT.md` to document the exhaustive raw surface and the remaining sparse header-only audit artifacts

## 0.2.1 - 2026-05-17

- Added `vImage` ARGB8888 alpha helpers plus ARGB8888 ↔ Planar8 conversion wrappers
- Added double-precision `vDSP` vector arithmetic, reductions, and window generators
- Added sparse matrix construction/query helpers and triangular dense solve wrappers
- Added safe BNNS Graph compile-options configuration wrappers for macOS 15+
- Refreshed `COVERAGE_AUDIT.md` and documented the remaining long-tail gaps as low priority

## 0.2.0 - 2026-05-16

- Switched the crate to a Swift bridge over Accelerate's C APIs
- Added optional `raw-ffi` re-exports for the wrapped Accelerate declarations
- Added `vForce`, `LAPACK`, `Sparse`, `simd`, and `Quadrature` modules
- Expanded `BNNS` with safe activation helpers while preserving the unsafe filter owner
- Added per-area examples and smoke tests across all nine logical areas
- Added `COVERAGE.md` documenting the v0.2.0 Accelerate surface audit

## 0.1.0 - 2026-05-16

- Initial release of `apple-accelerate`
- Added safe wrappers for common `vDSP`, `CBLAS`, and `vImage` workflows
- Added thin unsafe BNNS filter wrappers over the deprecated-but-available layer APIs
- Added smoke examples for FFT/vector math and BLAS + vImage usage
