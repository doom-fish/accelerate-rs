# accelerate-rs coverage audit v2 (corrected)

The first revision of this file claimed to re-verify a sample of about 300
symbols against the generated FFI modules. It was not a real verification:

- 36 of the 54 symbols it listed do not exist in the SDK, for example
  `vDSP_FFT_CreateSetup`, `vDSP_fft16`, `vDSP_matmul`, `vForce_asin`,
  `vImage_Buffer_Init` (the real name is `vImageBuffer_Init`),
  `vImage_GaussianBlur_ARGB8888`, `BNNS_FilterApplyConvolution`, and
  `SparseFactor_Dense_Float`.
- 2 (`SparseConvertFromCoordinate`, `SparseConvertFromOpaque`) are overload
  façades that the crate does not declare; only their typed entry points are
  declared.
- It mapped symbols to `src/ffi/generated/vdsp.rs` and
  `src/ffi/generated/vforce.rs`, which were never compiled. Both files were
  deleted in v0.4.0.
- Its per-family counts (for example "vForce (14 symbols)") and its sample
  size were invented.

## Headline numbers

The figures are those of `COVERAGE_AUDIT.md`: 3764 counted symbols, 1647
declared (43.8%), 108 not bindable on stable Rust, and 2009 EXEMPT, mostly
the BLAS and LAPACK C interfaces of which the crate declares five functions.
The earlier "100%" divided VERIFIED by the count without EXEMPT.

## Verified sample

Each row was checked against the SDK 26.5 export list (`Accelerate.tbd`),
the header that declares it, and the crate file that declares the raw
binding. `src/ffi/*.rs` files are hand-written; `src/ffi/generated/*.rs`
files are bindgen output. Most raw declarations have no safe wrapper.

| Symbol | Header | Declared in | Safe wrapper |
| --- | --- | --- | --- |
| `vDSP_create_fftsetup` | `vDSP.h` | `src/ffi/vdsp.rs` | `FftSetup::new` |
| `vDSP_fft_zip` | `vDSP.h` | `src/ffi/vdsp.rs` | `FftSetup::fft_zip` |
| `vDSP_fft_zop` | `vDSP.h` | `src/ffi/generated/vdsp_missing.rs` | raw only |
| `vDSP_fft_zrip` | `vDSP.h` | `src/ffi/generated/vdsp_missing.rs` | raw only |
| `vDSP_fft3_zop` | `vDSP.h` | `src/ffi/vdsp.rs` | raw only |
| `vDSP_fft5_zop` | `vDSP.h` | `src/ffi/vdsp.rs` | raw only |
| `vDSP_DFT_zop_CreateSetup` | `vDSP.h` | `src/ffi/generated/vdsp_missing.rs` | raw only |
| `vDSP_DFT_zrop_CreateSetup` | `vDSP.h` | `src/ffi/generated/vdsp_missing.rs` | raw only |
| `vDSP_DFT_Execute` | `vDSP.h` | `src/ffi/generated/vdsp_missing.rs` | raw only |
| `vDSP_biquad` | `vDSP.h` | `src/ffi/vdsp.rs` | `BiquadSetup::apply` |
| `vDSP_biquadm` | `vDSP.h` | `src/ffi/generated/vdsp_missing.rs` | raw only |
| `vDSP_conv` | `vDSP.h` | `src/ffi/generated/vdsp_missing.rs` | raw only |
| `vDSP_deq22` | `vDSP.h` | `src/ffi/generated/vdsp_missing.rs` | raw only |
| `vDSP_desamp` | `vDSP.h` | `src/ffi/generated/vdsp_missing.rs` | raw only |
| `vDSP_dotpr` | `vDSP.h` | `src/ffi/vdsp.rs` | `dot_f32` |
| `vDSP_hamm_window` | `vDSP.h` | `src/ffi/vdsp.rs` | `hamming_window` |
| `vDSP_hann_window` | `vDSP.h` | `src/ffi/generated/vdsp_missing.rs` | raw only |
| `vDSP_mmul` | `vDSP.h` | `src/ffi/generated/vdsp_missing.rs` | raw only |
| `vDSP_vadd` | `vDSP.h` | `src/ffi/vdsp.rs` | `add_f32` |
| `vDSP_vclip` | `vDSP.h` | `src/ffi/generated/vdsp_missing.rs` | raw only |
| `vDSP_vdiv` | `vDSP.h` | `src/ffi/generated/vdsp_missing.rs` | raw only |
| `vDSP_vmul` | `vDSP.h` | `src/ffi/generated/vdsp_missing.rs` | raw only |
| `vDSP_vsmul` | `vDSP.h` | `src/ffi/generated/vdsp_missing.rs` | raw only |
| `vDSP_vsub` | `vDSP.h` | `src/ffi/vdsp.rs` | `sub_f32` |
| `vvsinf` | `vForce.h` | `src/ffi/vforce.rs` | `sin_f32` |
| `vvcosf` | `vForce.h` | `src/ffi/vforce.rs` | `cos_f32` |
| `vvasinf` | `vForce.h` | `src/ffi/generated/vforce_missing.rs` | raw only |
| `vvatan2f` | `vForce.h` | `src/ffi/generated/vforce_missing.rs` | raw only |
| `vvcbrtf` | `vForce.h` | `src/ffi/generated/vforce_missing.rs` | raw only |
| `vvcopysignf` | `vForce.h` | `src/ffi/generated/vforce_missing.rs` | raw only |
| `vvcoshf` | `vForce.h` | `src/ffi/generated/vforce_missing.rs` | raw only |
| `vvatanhf` | `vForce.h` | `src/ffi/generated/vforce_missing.rs` | raw only |
| `vvfloorf` | `vForce.h` | `src/ffi/generated/vforce_missing.rs` | raw only |
| `vvpowf` | `vForce.h` | `src/ffi/generated/vforce_missing.rs` | raw only |
| `vImageBuffer_Init` | `vImage_Utilities.h` | `src/ffi/generated/vimage_missing.rs` | raw only |
| `vImageConvert_ARGB8888toRGB888` | `Conversion.h` | `src/ffi/generated/vimage_missing.rs` | raw only |
| `vImageConvert_RGB888toARGB8888` | `Conversion.h` | `src/ffi/generated/vimage_missing.rs` | raw only |
| `vImagePermuteChannels_ARGB8888` | `Conversion.h` | `src/ffi/generated/vimage_missing.rs` | raw only |
| `vImageTentConvolve_ARGB8888` | `Convolution.h` | `src/ffi/generated/vimage_missing.rs` | raw only |
| `vImageAffineWarp_ARGB8888` | `Geometry.h` | `src/ffi/generated/vimage_missing.rs` | raw only |
| `vImageDilate_ARGB8888` | `Morphology.h` | `src/ffi/generated/vimage_missing.rs` | raw only |
| `vImageScale_ARGB8888` | `Geometry.h` | `src/ffi/vimage.rs` | `scale_argb8888` |
| `vImagePremultiplyData_ARGB8888` | `Alpha.h` | `src/ffi/vimage.rs` | `premultiply_argb8888` |
| `BNNSFilterCreateLayerFullyConnected` | `BNNS/bnns.h` | `src/ffi/bnns.rs` | `bnns::Filter::from_fully_connected` (unsafe) |
| `BNNSGraphCompileFromFile` | `BNNS/bnns_graph.h` | `src/ffi/generated/bnns_missing.rs` | raw only |
| `BNNSGraphContextMake` | `BNNS/bnns_graph.h` | `src/ffi/generated/bnns_missing.rs` | raw only |
| `BNNSGraphContextExecute` | `BNNS/bnns_graph.h` | `src/ffi/generated/bnns_missing.rs` | raw only |
| `BNNSGraphCompileOptionsMakeDefault` | `BNNS/bnns_graph.h` | `src/ffi/bnns.rs` | `BnnsGraphCompileOptions::new` |
| `sparse_matrix_create_float` | `Sparse/BLAS.h` | `src/ffi/sparse.rs` | `SparseMatrixF32::new` |
| `sparse_inner_product_dense_float` | `Sparse/BLAS.h` | `src/ffi/sparse.rs` | `sparse_dot_dense_f32` |
| `_SparseSymbolicFactorLU` | `Sparse/SolveImplementation.h` | `src/ffi/generated/sparse_missing.rs` | raw only |
| `_SparseFactorLU_Double` | `Sparse/SolveImplementationTyped.h` (via `_SPARSE_VARIANT`) | `src/ffi/generated/sparse_missing.rs` | raw only |
| `_SparseSolveSubfactor_Double` | `Sparse/SolveImplementationTyped.h` (via `_SPARSE_VARIANT`) | `src/ffi/generated/sparse_missing.rs` | raw only |
| `_SparseGMRESSolve_Double` | `Sparse/SolveImplementationTyped.h` (via `_SPARSE_VARIANT`) | `src/ffi/generated/sparse_missing.rs` | raw only |
| `_SparseCGSolve_Double` | `Sparse/SolveImplementationTyped.h` (via `_SPARSE_VARIANT`) | `src/ffi/generated/sparse_missing.rs` | raw only |
| `vU256Add` | `vBigNum.h` | `src/ffi/generated/veclib_extras.rs` | raw only |
| `vU512FullMultiply` | `vBigNum.h` | `src/ffi/generated/veclib_extras.rs` | raw only |
| `vLL256Shift` | `vBigNum.h` | `src/ffi/generated/veclib_extras.rs` | raw only |
| `BLASSetThreading` | `thread_api.h` | `src/ffi/generated/veclib_extras.rs` | raw only |
| `cblas_sdot` | `cblas_new.h` | `src/ffi/blas.rs` | `sdot` |
| `cblas_sgemv` | `cblas_new.h` | `src/ffi/blas.rs` | `sgemv_row_major` |
| `cblas_sgemm` | `cblas_new.h` | `src/ffi/blas.rs` | `sgemm_row_major` |
| `sgetrf_` | `clapack.h` | `src/ffi/lapack.rs` | `lu_decompose_f32` |
| `sgesv_` | `clapack.h` | `src/ffi/lapack.rs` | `solve_linear_system_f32` |
| `quadrature_integrate` | `Quadrature/Integration.h` | `src/ffi/quadrature.rs` | `integrate` |

## Known gaps

- BLAS and LAPACK beyond `cblas_sdot`, `cblas_sgemv`, `cblas_sgemm`,
  `sgetrf_`, and `sgesv_`.
- The 108 `vfp.h` and `vBasicOps.h` functions that pass C vector types by
  value; `COVERAGE_AUDIT.md` lists them.
- Deprecated functions outside the counted set, for example
  `BNNSFilterCreateLayerActivation` and `BNNSFilterApplyBatch`.
- Safe wrappers for BNNS Graph execution, the Sparse solvers, the vDSP DFT
  routines, and the radix-3/5 FFTs; see `COVERAGE.md`.
