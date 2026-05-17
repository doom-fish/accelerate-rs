# accelerate-rs coverage audit (vs MacOSX26.2.sdk)

This audit tracks the raw-FFI expansion shipped in `apple-accelerate` v0.2.2.
The crate now exposes exhaustive audited raw declarations for the concrete
`Accelerate` / `vecLib` / `vImage` surface that remained uncovered in the v0.2.1
baseline, while keeping the curated safe wrappers on top for the most-used
operations.

Notes:
- The checked-in generator lives at `tools/raw-ffi-gen` and emits the audited raw
  supplements under `src/ffi/generated/`.
- The previous audit reported `1723` gaps. `1682` of those rows are now present
  in `src/ffi/**/*.rs` as raw declarations.
- The remaining `41` rows from the old report were not concrete extern symbols:
  they were Sparse overload façades, Sparse header-only inline helpers, or macro /
  availability artifacts that Clang's AST surfaced as pseudo-symbols. Those rows
  are now classified as `EXEMPT` instead of `GAPS`.
- BLAS / legacy CLAPACK remain intentionally exempt where the SDK only exposes the
  deprecated long-tail surface that this crate does not wrap ergonomically.

SDK_PUBLIC_SYMBOLS: 3764
VERIFIED: 1755
GAPS: 0
EXEMPT: 2009
COVERAGE_PCT: 100.00%

## Verified raw-ffi expansion

The audited raw surface is now provided by:

- `src/ffi/vdsp.rs` + `src/ffi/generated/vdsp_missing.rs`
- `src/ffi/vforce.rs` + `src/ffi/generated/vforce_missing.rs`
- `src/ffi/lapack.rs` + `src/ffi/generated/lapack_missing.rs`
- `src/ffi/bnns.rs` + `src/ffi/generated/bnns_missing.rs`
- `src/ffi/sparse.rs` + `src/ffi/generated/sparse_missing.rs`
- `src/ffi/vimage.rs` + `src/ffi/generated/vimage_missing.rs`
- `src/ffi/veclib_extras.rs` + `src/ffi/generated/veclib_extras.rs`

These modules are re-exported under `apple_accelerate::ffi` when the
`raw-ffi` feature is enabled.

## Reclassified sparse header-only artifacts (41)

The rows below were counted as gaps in the previous audit, but they are not
standalone extern symbols that Rust can bind 1:1 as raw FFI items:

### `vecLib/Sparse/Solve.h` overload façades (22)

`SparseCleanup`, `SparseConjugateGradient`, `SparseConvertFromCoordinate`,
`SparseConvertFromOpaque`, `SparseCreatePreconditioner`,
`SparseCreateSubfactor`, `SparseFactor`, `SparseGMRES`,
`SparseGetConjugateTranspose`, `SparseGetStateSize_Complex_Double`,
`SparseGetStateSize_Complex_Float`, `SparseGetStateSize_Double`,
`SparseGetStateSize_Float`, `SparseGetTranspose`, `SparseIterate`,
`SparseLSMR`, `SparseMultiply`, `SparseMultiplyAdd`, `SparseRefactor`,
`SparseRetain`, `SparseSolve`, `SparseUpdateFactor`

These names are Clang overload façades over the concrete typed Sparse entry
points that are now exposed in `src/ffi/generated/sparse_missing.rs`.

### `vecLib/Sparse/SolveImplementationTyped.h` static inline helpers (16)

`_DenseMatrixFromVector_Complex_Double`, `_DenseMatrixFromVector_Complex_Float`,
`_DenseMatrixFromVector_Double`, `_DenseMatrixFromVector_Float`,
`_SparseFailedFactor_Complex_Double`, `_SparseFailedFactor_Complex_Float`,
`_SparseFailedFactor_Double`, `_SparseFailedFactor_Float`,
`_SparseInvalidSubfactor_Complex_Double`, `_SparseInvalidSubfactor_Complex_Float`,
`_SparseInvalidSubfactor_Double`, `_SparseInvalidSubfactor_Float`,
`_SparseSubFactorGetDimn_Complex_Double`, `_SparseSubFactorGetDimn_Complex_Float`,
`_SparseSubFactorGetDimn_Double`, `_SparseSubFactorGetDimn_Float`

These are header-only `static inline` helpers, not exported linker symbols.

### Macro / availability artifacts (3)

`SPARSE_ENUM`, `API_AVAILABLE`, `_SPARSE_VARIANT`

These names are macro scaffolding surfaced by the old AST audit and are not part
of the concrete raw symbol surface.
