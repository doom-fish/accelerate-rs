# accelerate-rs coverage audit (vs MacOSX26.2.sdk)

This audit tracks the raw FFI layer (`raw-ffi` feature). It counts C
declarations, not safe wrappers: the safe API covers a small subset, listed
in `COVERAGE.md`.

## What the numbers measure

The v0.2.2 audit counted 3764 public symbols (functions, typedefs, and
constants) in the Accelerate, vecLib, and vImage headers of MacOSX26.2.sdk:

SDK_PUBLIC_SYMBOLS: 3764
VERIFIED: 1647 (1755 in v0.2.2; 108 were removed in v0.4.0, see below)
GAPS: 0
NOT_BINDABLE: 108
EXEMPT: 2009

The "100%" the earlier revisions of this file reported was VERIFIED divided
by (SDK_PUBLIC_SYMBOLS − EXEMPT), that is 1755 / 1755. Measured against all
3764 counted symbols, the raw layer declares 1647, or 43.8%.

EXEMPT is not a list of irrelevant symbols. It is mostly the BLAS and LAPACK
C interfaces: in SDK 26.5, `cblas.h` declares 148 `cblas_*` functions and
`clapack.h` about 1,470 LAPACK routines, and the crate declares only
`cblas_sdot`, `cblas_sgemv`, `cblas_sgemm`, `sgetrf_`, and `sgesv_`. The other
41 EXEMPT rows are the Sparse header-only artifacts listed at the end of this
file.

Deprecated functions were outside the counted set. Some of them are declared
anyway (for example `vDSP_fft3_zop` and `vDSP_fft5_zop`, added in v0.4.0),
others are not (for example `BNNSFilterCreateLayerActivation` and
`BNNSFilterApplyBatch`). The numbers were not regenerated against newer SDKs.

## Changes in v0.4.0

- 108 declarations that passed C vector types (`vFloat`, `vUInt32`, ...) by
  value were removed: Rust arrays use a different calling convention from C
  vectors, and stable Rust rejects SIMD types in `extern` blocks. They are
  counted as NOT_BINDABLE above. `vUInt32` remains only as storage inside the
  16-byte aligned vBigNum unions, whose functions take pointers.
  - `vBasicOps.h` (63): vU8Divide, vS8Divide, vU16Divide, vS16Divide,
    vU32Divide, vS32Divide, vU64Divide, vS64Divide, vU128Divide, vS128Divide,
    vU8HalfMultiply, vS8HalfMultiply, vU16HalfMultiply, vS16HalfMultiply,
    vU32HalfMultiply, vS32HalfMultiply, vU32FullMulEven, vU32FullMulOdd,
    vS32FullMulEven, vS32FullMulOdd, vU64FullMulEven, vU64FullMulOdd,
    vU64HalfMultiply, vS64HalfMultiply, vS64FullMulEven, vS64FullMulOdd,
    vU128HalfMultiply, vS128HalfMultiply, vU64Sub, vU64SubS, vU128Sub,
    vU128SubS, vS64Sub, vS128Sub, vS64SubS, vS128SubS, vU64Add, vU64AddS,
    vU128Add, vU128AddS, vS64Add, vS64AddS, vS128Add, vS128AddS, vU64Neg,
    vS64Neg, vU128Neg, vS128Neg, vLL64Shift, vA64Shift, vLR64Shift,
    vLL64Shift2, vA64Shift2, vLR64Shift2, vLL128Shift, vLR128Shift,
    vA128Shift, vL64Rotate, vR64Rotate, vL64Rotate2, vR64Rotate2, vL128Rotate,
    vR128Rotate
  - `vfp.h` (45): vceilf, vfloorf, vtruncf, vnintf, vexpf, vexp2f, vexpm1f,
    vlogf, vlog2f, vlog10f, vlog1pf, vlogbf, vscalbf, vpowf, vipowf, vsinf,
    vcosf, vsincosf, vtanf, vsinpif, vcospif, vtanpif, vasinf, vacosf, vatanf,
    vatan2f, vsinhf, vcoshf, vtanhf, vasinhf, vacoshf, vatanhf, vrecf, vsqrtf,
    vrsqrtf, vdivf, vfmodf, vremainderf, vremquof, vfabsf, vcopysignf,
    vsignbitf, vnextafterf, vclassifyf, vtablelookup. Most have array
    counterparts in vForce (`vvfloorf`, `vvexpf`, ...), which are declared.
- `quadrature_status` is an `i32` newtype with associated constants instead
  of a Rust enum.
- `src/ffi/generated/vdsp.rs` and `vforce.rs`, which were never compiled,
  were deleted.

## Raw FFI layout

The raw declarations are provided by:

- `src/ffi/vdsp.rs` + `src/ffi/generated/vdsp_missing.rs`
- `src/ffi/vforce.rs` + `src/ffi/generated/vforce_missing.rs`
- `src/ffi/blas.rs`
- `src/ffi/lapack.rs` + `src/ffi/generated/lapack_missing.rs` (types only)
- `src/ffi/bnns.rs` + `src/ffi/generated/bnns_missing.rs`
- `src/ffi/sparse.rs` + `src/ffi/generated/sparse_missing.rs`
- `src/ffi/vimage.rs` + `src/ffi/generated/vimage_missing.rs`
- `src/ffi/quadrature.rs`, `src/ffi/simd.rs`
- `src/ffi/veclib_extras.rs` + `src/ffi/generated/veclib_extras.rs`

These modules are re-exported under `apple_accelerate::ffi` when the
`raw-ffi` feature is enabled.

The files under `src/ffi/generated/` come from `tools/raw-ffi-gen`. The
generator originally took its symbol list from the GAPS table of this file.
It now refreshes the symbol set already checked in (it reproduces the
v0.3.3 files byte for byte against SDK 26.5), re-applies the `apple-cf`
re-exports, drops declarations that pass C vector types by value, and emits
bindgen's compile-time layout assertions, which pass on aarch64 and x86_64.
The hand-written structs in `src/ffi/*.rs` carry matching assertions checked
against clang.

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
