# Changelog

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
