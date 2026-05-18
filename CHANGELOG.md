# Changelog

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
