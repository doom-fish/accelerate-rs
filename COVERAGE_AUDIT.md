# accelerate-rs coverage audit (vs MacOSX26.2.sdk)

Automated against the full public header set for `Accelerate.framework`, `vecLib.framework`, and `vImage.framework` in the macOS SDK. This audit counts public functions, typedef'd enum/struct/opaque-pointer types, and extern consts discovered via Clang AST dumps. Plain scalar typedef aliases and macro constants are intentionally excluded because the audit instructions target callable/public surface symbols.

Notes:
- `simd` is intentionally excluded: the crate's `simd` module wraps Swift's `simd`, not `Accelerate.framework`.
- `raw-ffi` is included in VERIFIED/EXEMPT because the instructions require auditing all public `src/**/*.rs` exports, including feature-gated ones.
- Apple now deprecates large parts of classic CBLAS/LAPACK/LinearAlgebra and legacy BNNS layer-builder APIs; those rows are EXEMPT and do not count toward coverage.
- Largest current gap families: vImage (602), vDSP (454), Sparse (330), vForce (79), vBigNum (77), BNNS (68), vBasicOps (63), vfp (45).
- Remaining unwrapped surface is now intentionally low-priority long-tail work: more vImage format permutations, specialized vDSP transform families, broader sparse matrix construction/query helpers, and BNNS Graph execution/runtime APIs.

SDK_PUBLIC_SYMBOLS: 3764
VERIFIED: 73
GAPS: 1723
EXEMPT: 1968
COVERAGE_PCT: 4.06%

## 🟢 VERIFIED

| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| vImageBoxConvolve_ARGB8888 | function | vImage/Convolution.h | box_convolve_argb8888, ffi::vImageBoxConvolve_ARGB8888 (raw-ffi) |
| vImageRotate_ARGB8888 | function | vImage/Geometry.h | ffi::vImageRotate_ARGB8888 (raw-ffi), rotate_argb8888 |
| vImageScale_ARGB8888 | function | vImage/Geometry.h | ffi::vImageScale_ARGB8888 (raw-ffi), scale_argb8888 |
| vImageContrastStretch_Planar8 | function | vImage/Histogram.h | contrast_stretch_planar8, ffi::vImageContrastStretch_Planar8 (raw-ffi) |
| vImage_Buffer | typedef | vImage/vImage_Types.h | ImageBuffer, ffi::vImage_Buffer (raw-ffi) |
| quadrature_integrate | function | vecLib/Quadrature/Integration.h | ffi::quadrature_integrate (raw-ffi), integrate |
| quadrature_integrate_function | typedef | vecLib/Quadrature/Integration.h | ffi::quadrature_integrate_function (raw-ffi), integrate (callback bridge) |
| quadrature_integrate_options | typedef | vecLib/Quadrature/Integration.h | QuadratureOptions, ffi::quadrature_integrate_options (raw-ffi) |
| quadrature_integrator | typedef | vecLib/Quadrature/Integration.h | QuadratureIntegrator, QuadratureOptions::integrator, ffi::quadrature_integrator (raw-ffi) |
| quadrature_status | typedef | vecLib/Quadrature/Quadrature.h | QuadratureOutput, ffi::quadrature_status (raw-ffi), integrate |
| sparse_inner_product_dense_float | function | vecLib/Sparse/BLAS.h | ffi::sparse_inner_product_dense_float (raw-ffi), sparse_dot_dense_f32 |
| sparse_inner_product_sparse_float | function | vecLib/Sparse/BLAS.h | ffi::sparse_inner_product_sparse_float (raw-ffi), sparse_dot_sparse_f32 |
| sparse_vector_add_with_scale_dense_float | function | vecLib/Sparse/BLAS.h | ffi::sparse_vector_add_with_scale_dense_float (raw-ffi), sparse_add_to_dense_f32 |
| DSPSplitComplex | typedef | vecLib/vDSP.h | ffi::DSPSplitComplex (raw-ffi) |
| FFTSetup | typedef | vecLib/vDSP.h | FftSetup, ffi::FFTSetup (raw-ffi) |
| vDSP_biquad | function | vecLib/vDSP.h | BiquadSetup::apply, ffi::vDSP_biquad (raw-ffi) |
| vDSP_biquad_CreateSetup | function | vecLib/vDSP.h | BiquadSetup::new, ffi::vDSP_biquad_CreateSetup (raw-ffi) |
| vDSP_biquad_DestroySetup | function | vecLib/vDSP.h | BiquadSetup::drop (via raw-ffi only), ffi::vDSP_biquad_DestroySetup (raw-ffi) |
| vDSP_biquad_Setup | typedef | vecLib/vDSP.h | BiquadSetup, ffi::vDSP_biquad_Setup (raw-ffi) |
| vDSP_blkman_window | function | vecLib/vDSP.h | blackman_window, ffi::vDSP_blkman_window (raw-ffi) |
| vDSP_create_fftsetup | function | vecLib/vDSP.h | FftSetup::new, ffi::vDSP_create_fftsetup (raw-ffi) |
| vDSP_destroy_fftsetup | function | vecLib/vDSP.h | FftSetup::drop (via raw-ffi only), ffi::vDSP_destroy_fftsetup (raw-ffi) |
| vDSP_dotpr | function | vecLib/vDSP.h | dot_f32, ffi::vDSP_dotpr (raw-ffi) |
| vDSP_fft_zip | function | vecLib/vDSP.h | FftSetup::fft_zip, ffi::vDSP_fft_zip (raw-ffi) |
| vDSP_hamm_window | function | vecLib/vDSP.h | ffi::vDSP_hamm_window (raw-ffi), hamming_window |
| vDSP_maxv | function | vecLib/vDSP.h | ffi::vDSP_maxv (raw-ffi), max_f32 |
| vDSP_meanv | function | vecLib/vDSP.h | ffi::vDSP_meanv (raw-ffi), mean_f32 |
| vDSP_minv | function | vecLib/vDSP.h | ffi::vDSP_minv (raw-ffi), min_f32 |
| vDSP_sve | function | vecLib/vDSP.h | ffi::vDSP_sve (raw-ffi), sum_f32 |
| vDSP_vadd | function | vecLib/vDSP.h | add_f32, ffi::vDSP_vadd (raw-ffi) |
| vDSP_vsub | function | vecLib/vDSP.h | ffi::vDSP_vsub (raw-ffi), sub_f32 |
| vvcosf | function | vecLib/vForce.h | cos_f32, ffi::vvcosf (raw-ffi) |
| vvexpf | function | vecLib/vForce.h | exp_f32, ffi::vvexpf (raw-ffi) |
| vvlogf | function | vecLib/vForce.h | ffi::vvlogf (raw-ffi), log_f32 |
| vvsinf | function | vecLib/vForce.h | ffi::vvsinf (raw-ffi), sin_f32 |
| vvsqrtf | function | vecLib/vForce.h | ffi::vvsqrtf (raw-ffi), sqrt_f32 |
| vImageAlphaBlend_ARGB8888 | function | vImage/Alpha.h | alpha_blend_argb8888, ffi::vImageAlphaBlend_ARGB8888 (raw-ffi) |
| vImageClipToAlpha_ARGB8888 | function | vImage/Alpha.h | clip_to_alpha_argb8888, ffi::vImageClipToAlpha_ARGB8888 (raw-ffi) |
| vImagePremultiplyData_ARGB8888 | function | vImage/Alpha.h | ffi::vImagePremultiplyData_ARGB8888 (raw-ffi), premultiply_argb8888 |
| vImageUnpremultiplyData_ARGB8888 | function | vImage/Alpha.h | ffi::vImageUnpremultiplyData_ARGB8888 (raw-ffi), unpremultiply_argb8888 |
| vImageConvert_ARGB8888toPlanar8 | function | vImage/Conversion.h | convert_argb8888_to_planar8, ffi::vImageConvert_ARGB8888toPlanar8 (raw-ffi) |
| vImageConvert_Planar8toARGB8888 | function | vImage/Conversion.h | convert_planar8_to_argb8888, ffi::vImageConvert_Planar8toARGB8888 (raw-ffi) |
| BNNSGraphCompileOptionsDestroy | function | vecLib/BNNS/bnns_graph.h | BnnsGraphCompileOptions::drop (via bridge), ffi::BNNSGraphCompileOptionsDestroy (raw-ffi) |
| BNNSGraphCompileOptionsGetGenerateDebugInfo | function | vecLib/BNNS/bnns_graph.h | BnnsGraphCompileOptions::generate_debug_info, ffi::BNNSGraphCompileOptionsGetGenerateDebugInfo (raw-ffi) |
| BNNSGraphCompileOptionsGetOptimizationPreference | function | vecLib/BNNS/bnns_graph.h | BnnsGraphCompileOptions::optimization_preference, ffi::BNNSGraphCompileOptionsGetOptimizationPreference (raw-ffi) |
| BNNSGraphCompileOptionsGetTargetSingleThread | function | vecLib/BNNS/bnns_graph.h | BnnsGraphCompileOptions::target_single_thread, ffi::BNNSGraphCompileOptionsGetTargetSingleThread (raw-ffi) |
| BNNSGraphCompileOptionsMakeDefault | function | vecLib/BNNS/bnns_graph.h | BnnsGraphCompileOptions::new, ffi::BNNSGraphCompileOptionsMakeDefault (raw-ffi) |
| BNNSGraphCompileOptionsSetGenerateDebugInfo | function | vecLib/BNNS/bnns_graph.h | BnnsGraphCompileOptions::set_generate_debug_info, ffi::BNNSGraphCompileOptionsSetGenerateDebugInfo (raw-ffi) |
| BNNSGraphCompileOptionsSetOptimizationPreference | function | vecLib/BNNS/bnns_graph.h | BnnsGraphCompileOptions::set_optimization_preference, ffi::BNNSGraphCompileOptionsSetOptimizationPreference (raw-ffi) |
| BNNSGraphCompileOptionsSetTargetSingleThread | function | vecLib/BNNS/bnns_graph.h | BnnsGraphCompileOptions::set_target_single_thread, ffi::BNNSGraphCompileOptionsSetTargetSingleThread (raw-ffi) |
| bnns_graph_compile_options_t | typedef | vecLib/BNNS/bnns_graph.h | BnnsGraphCompileOptions, ffi::bnns_graph_compile_options_t (raw-ffi) |
| sparse_commit | function | vecLib/Sparse/BLAS.h | SparseMatrixF32::commit, ffi::sparse_commit (raw-ffi) |
| sparse_get_matrix_nonzero_count | function | vecLib/Sparse/BLAS.h | SparseMatrixF32::nonzero_count, ffi::sparse_get_matrix_nonzero_count (raw-ffi) |
| sparse_get_matrix_number_of_columns | function | vecLib/Sparse/BLAS.h | SparseMatrixF32::columns, ffi::sparse_get_matrix_number_of_columns (raw-ffi) |
| sparse_get_matrix_number_of_rows | function | vecLib/Sparse/BLAS.h | SparseMatrixF32::rows, ffi::sparse_get_matrix_number_of_rows (raw-ffi) |
| sparse_insert_entry_float | function | vecLib/Sparse/BLAS.h | SparseMatrixF32::insert_entry, ffi::sparse_insert_entry_float (raw-ffi) |
| sparse_matrix_create_float | function | vecLib/Sparse/BLAS.h | SparseMatrixF32::new, ffi::sparse_matrix_create_float (raw-ffi) |
| sparse_matrix_destroy | function | vecLib/Sparse/BLAS.h | SparseMatrixF32::drop (via bridge), ffi::sparse_matrix_destroy (raw-ffi) |
| sparse_matrix_triangular_solve_dense_float | function | vecLib/Sparse/BLAS.h | SparseMatrixF32::triangular_solve_matrix_row_major, ffi::sparse_matrix_triangular_solve_dense_float (raw-ffi) |
| sparse_set_matrix_property | function | vecLib/Sparse/BLAS.h | SparseMatrixF32::set_property, ffi::sparse_set_matrix_property (raw-ffi) |
| sparse_vector_triangular_solve_dense_float | function | vecLib/Sparse/BLAS.h | SparseMatrixF32::triangular_solve_vector, ffi::sparse_vector_triangular_solve_dense_float (raw-ffi) |
| sparse_matrix_float | typedef | vecLib/Sparse/Types.h | SparseMatrixF32, ffi::sparse_matrix_float (raw-ffi) |
| sparse_matrix_property | typedef | vecLib/Sparse/Types.h | ffi::sparse_matrix_property (raw-ffi), sparse_matrix_property |
| sparse_status | typedef | vecLib/Sparse/Types.h | ffi::sparse_status (raw-ffi), sparse_status |
| vDSP_blkman_windowD | function | vecLib/vDSP.h | blackman_window_f64, ffi::vDSP_blkman_windowD (raw-ffi) |
| vDSP_dotprD | function | vecLib/vDSP.h | dot_f64, ffi::vDSP_dotprD (raw-ffi) |
| vDSP_hamm_windowD | function | vecLib/vDSP.h | ffi::vDSP_hamm_windowD (raw-ffi), hamming_window_f64 |
| vDSP_maxvD | function | vecLib/vDSP.h | ffi::vDSP_maxvD (raw-ffi), max_f64 |
| vDSP_meanvD | function | vecLib/vDSP.h | ffi::vDSP_meanvD (raw-ffi), mean_f64 |
| vDSP_minvD | function | vecLib/vDSP.h | ffi::vDSP_minvD (raw-ffi), min_f64 |
| vDSP_sveD | function | vecLib/vDSP.h | ffi::vDSP_sveD (raw-ffi), sum_f64 |
| vDSP_vaddD | function | vecLib/vDSP.h | add_f64, ffi::vDSP_vaddD (raw-ffi) |
| vDSP_vsubD | function | vecLib/vDSP.h | ffi::vDSP_vsubD (raw-ffi), sub_f64 |

## 🔴 GAPS

| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| vImageAlphaBlend_ARGBFFFF | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageAlphaBlend_NonpremultipliedToPremultiplied_ARGB8888 | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageAlphaBlend_NonpremultipliedToPremultiplied_ARGBFFFF | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageAlphaBlend_NonpremultipliedToPremultiplied_Planar8 | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageAlphaBlend_NonpremultipliedToPremultiplied_PlanarF | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageAlphaBlend_Planar8 | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageAlphaBlend_PlanarF | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageClipToAlpha_ARGBFFFF | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageClipToAlpha_Planar8 | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageClipToAlpha_PlanarF | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageClipToAlpha_RGBA8888 | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageClipToAlpha_RGBAFFFF | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePremultipliedAlphaBlendDarken_RGBA8888 | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePremultipliedAlphaBlendLighten_RGBA8888 | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePremultipliedAlphaBlendMultiply_RGBA8888 | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePremultipliedAlphaBlendScreen_RGBA8888 | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePremultipliedAlphaBlendWithPermute_ARGB8888 | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePremultipliedAlphaBlendWithPermute_RGBA8888 | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePremultipliedAlphaBlend_ARGB8888 | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePremultipliedAlphaBlend_ARGBFFFF | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePremultipliedAlphaBlend_BGRA8888 | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePremultipliedAlphaBlend_BGRAFFFF | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePremultipliedAlphaBlend_Planar8 | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePremultipliedAlphaBlend_PlanarF | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePremultipliedConstAlphaBlend_ARGB8888 | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePremultipliedConstAlphaBlend_ARGBFFFF | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePremultipliedConstAlphaBlend_Planar8 | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePremultipliedConstAlphaBlend_PlanarF | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePremultiplyData_ARGB16Q12 | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePremultiplyData_ARGB16U | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePremultiplyData_ARGBFFFF | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePremultiplyData_Planar8 | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePremultiplyData_PlanarF | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePremultiplyData_RGBA16F | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePremultiplyData_RGBA16Q12 | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePremultiplyData_RGBA16U | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePremultiplyData_RGBA8888 | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePremultiplyData_RGBAFFFF | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageUnpremultiplyData_ARGB16Q12 | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageUnpremultiplyData_ARGB16U | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageUnpremultiplyData_ARGBFFFF | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageUnpremultiplyData_Planar8 | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageUnpremultiplyData_PlanarF | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageUnpremultiplyData_RGBA16F | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageUnpremultiplyData_RGBA16Q12 | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageUnpremultiplyData_RGBA16U | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageUnpremultiplyData_RGBA8888 | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageUnpremultiplyData_RGBAFFFF | function | vImage/Alpha.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePNGDecompressionFilter | function | vImage/BasicImageTypes.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageBufferFill_ARGB16F | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageBufferFill_ARGB16S | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageBufferFill_ARGB16U | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageBufferFill_ARGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageBufferFill_ARGBFFFF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageBufferFill_CbCr16S | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageBufferFill_CbCr16U | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageBufferFill_CbCr8 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageByteSwap_Planar16U | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageClip_PlanarF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_12UTo16U | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_16Fto16Q12 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_16Fto16U | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_16Q12to16F | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_16Q12to16U | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_16Q12to8 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_16Q12toF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_16SToF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_16UTo12U | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_16UToF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_16UToPlanar8 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_16Uto16F | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_16Uto16Q12 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_420Yp8_Cb8_Cr8ToARGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_420Yp8_CbCr8ToARGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_422CbYpCrYp16ToARGB16U | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_422CbYpCrYp16ToARGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_422CbYpCrYp8ToARGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_422CbYpCrYp8_AA8ToARGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_422CrYpCbYpCbYpCbYpCrYpCrYp10ToARGB16Q12 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_422CrYpCbYpCbYpCbYpCrYpCrYp10ToARGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_422YpCbYpCr8ToARGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_444AYpCbCr16ToARGB16U | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_444AYpCbCr16ToARGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_444AYpCbCr8ToARGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_444CbYpCrA8ToARGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_444CrYpCb10ToARGB16Q12 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_444CrYpCb10ToARGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_444CrYpCb8ToARGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_8to16Q12 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB1555toARGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB1555toPlanar8 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB1555toRGB565 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB16Q12To422CrYpCbYpCbYpCbYpCrYpCrYp10 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB16Q12To444CrYpCb10 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB16Q12ToARGB2101010 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB16Q12ToRGBA1010102 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB16Q12ToXRGB2101010 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB16UTo422CbYpCrYp16 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB16UTo444AYpCbCr16 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB16UToARGB2101010 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB16UToARGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB16UToRGBA1010102 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB16UToXRGB2101010 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB16UtoARGB8888_dithered | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB16UtoPlanar16U | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB16UtoRGB16U | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB2101010ToARGB16F | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB2101010ToARGB16Q12 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB2101010ToARGB16U | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB2101010ToARGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB2101010ToARGBFFFF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB8888To420Yp8_Cb8_Cr8 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB8888To420Yp8_CbCr8 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB8888To422CbYpCrYp16 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB8888To422CbYpCrYp8 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB8888To422CbYpCrYp8_AA8 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB8888To422CrYpCbYpCbYpCbYpCrYpCrYp10 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB8888To422YpCbYpCr8 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB8888To444AYpCbCr16 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB8888To444AYpCbCr8 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB8888To444CbYpCrA8 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB8888To444CrYpCb10 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB8888To444CrYpCb8 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB8888ToARGB16U | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB8888ToARGB2101010 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB8888ToRGB16U | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB8888ToRGBA1010102 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB8888ToXRGB2101010 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB8888toARGB1555 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB8888toARGB1555_dithered | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB8888toPlanar16Q12 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB8888toPlanarF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB8888toRGB565 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB8888toRGB565_dithered | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGB8888toRGB888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGBFFFFToARGB2101010 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGBFFFFToXRGB2101010 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGBFFFFtoARGB8888_dithered | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGBFFFFtoPlanar8 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGBFFFFtoPlanarF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGBFFFFtoRGBFFF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ARGBToYpCbCr_GenerateConversion | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_BGRA16UtoRGB16U | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_BGRA8888toRGB565 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_BGRA8888toRGB565_dithered | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_BGRA8888toRGB888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_BGRAFFFFtoRGBFFF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_BGRX8888ToPlanar8 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_BGRXFFFFToPlanarF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ChunkyToPlanar8 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_ChunkyToPlanarF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_FTo16S | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_FTo16U | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_Fto16Q12 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_Indexed1toPlanar8 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_Indexed2toPlanar8 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_Indexed4toPlanar8 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_Planar16FtoPlanar8 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_Planar16FtoPlanarF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_Planar16Q12toARGB16F | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_Planar16Q12toARGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_Planar16Q12toRGB16F | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_Planar16Q12toRGB888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_Planar16UtoARGB16U | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_Planar16UtoPlanar8_dithered | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_Planar16UtoRGB16U | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_Planar1toPlanar8 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_Planar2toPlanar8 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_Planar4toPlanar8 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_Planar8To16U | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_Planar8ToARGBFFFF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_Planar8ToBGRX8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_Planar8ToBGRXFFFF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_Planar8ToXRGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_Planar8ToXRGBFFFF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_Planar8toARGB1555 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_Planar8toIndexed1 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_Planar8toIndexed2 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_Planar8toIndexed4 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_Planar8toPlanar1 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_Planar8toPlanar16F | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_Planar8toPlanar2 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_Planar8toPlanar4 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_Planar8toPlanarF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_Planar8toRGB565 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_Planar8toRGB888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_PlanarFToARGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_PlanarFToBGRX8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_PlanarFToBGRXFFFF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_PlanarFToXRGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_PlanarFToXRGBFFFF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_PlanarFtoARGBFFFF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_PlanarFtoPlanar16F | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_PlanarFtoPlanar8 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_PlanarFtoPlanar8_dithered | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_PlanarFtoRGBFFF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_PlanarToChunky8 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_PlanarToChunkyF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGB16UToARGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGB16UtoARGB16U | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGB16UtoBGRA16U | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGB16UtoPlanar16U | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGB16UtoRGB888_dithered | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGB16UtoRGBA16U | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGB565toARGB1555 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGB565toARGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGB565toBGRA8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGB565toPlanar8 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGB565toRGB888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGB565toRGBA5551 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGB565toRGBA8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGB888toARGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGB888toBGRA8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGB888toPlanar16Q12 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGB888toPlanar8 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGB888toRGB565_dithered | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGB888toRGBA8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGBA1010102ToARGB16Q12 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGBA1010102ToARGB16U | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGBA1010102ToARGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGBA16UtoRGB16U | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGBA5551toRGB565 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGBA5551toRGBA8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGBA8888toRGB565 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGBA8888toRGB565_dithered | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGBA8888toRGB888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGBA8888toRGBA5551 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGBA8888toRGBA5551_dithered | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGBAFFFFtoRGBFFF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGBFFFtoARGBFFFF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGBFFFtoBGRAFFFF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGBFFFtoPlanarF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGBFFFtoRGB888_dithered | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_RGBFFFtoRGBAFFFF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_XRGB2101010ToARGB16F | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_XRGB2101010ToARGB16Q12 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_XRGB2101010ToARGB16U | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_XRGB2101010ToARGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_XRGB2101010ToARGBFFFF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_XRGB8888ToPlanar8 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_XRGBFFFFToPlanarF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_YpCbCrToARGB_GenerateConversion | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageCopyBuffer | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageExtractChannel_ARGB16U | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageExtractChannel_ARGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageExtractChannel_ARGBFFFF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageFlatten_ARGB16Q12 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageFlatten_ARGB16U | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageFlatten_ARGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageFlatten_ARGB8888ToRGB888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageFlatten_ARGBFFFF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageFlatten_ARGBFFFFToRGBFFF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageFlatten_BGRA8888ToRGB888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageFlatten_BGRAFFFFToRGBFFF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageFlatten_RGBA16Q12 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageFlatten_RGBA16U | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageFlatten_RGBA8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageFlatten_RGBA8888ToRGB888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageFlatten_RGBAFFFF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageFlatten_RGBAFFFFToRGBFFF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageOverwriteChannelsWithPixel_ARGB16U | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageOverwriteChannelsWithPixel_ARGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageOverwriteChannelsWithPixel_ARGBFFFF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageOverwriteChannelsWithScalar_ARGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageOverwriteChannelsWithScalar_ARGBFFFF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageOverwriteChannelsWithScalar_Planar16F | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageOverwriteChannelsWithScalar_Planar16S | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageOverwriteChannelsWithScalar_Planar16U | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageOverwriteChannelsWithScalar_Planar8 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageOverwriteChannelsWithScalar_PlanarF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageOverwriteChannels_ARGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageOverwriteChannels_ARGBFFFF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePermuteChannelsWithMaskedInsert_ARGB16U | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePermuteChannelsWithMaskedInsert_ARGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePermuteChannelsWithMaskedInsert_ARGBFFFF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePermuteChannels_ARGB16F | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePermuteChannels_ARGB16U | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePermuteChannels_ARGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePermuteChannels_ARGBFFFF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePermuteChannels_RGB888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageSelectChannels_ARGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageSelectChannels_ARGBFFFF | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageTableLookUp_ARGB8888 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageTableLookUp_Planar8 | function | vImage/Conversion.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageBoxConvolve_Planar8 | function | vImage/Convolution.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvolveFloatKernel_ARGB8888 | function | vImage/Convolution.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvolveMultiKernel_ARGB8888 | function | vImage/Convolution.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvolveMultiKernel_ARGBFFFF | function | vImage/Convolution.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvolveWithBias_ARGB16F | function | vImage/Convolution.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvolveWithBias_ARGB8888 | function | vImage/Convolution.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvolveWithBias_ARGBFFFF | function | vImage/Convolution.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvolveWithBias_Planar16F | function | vImage/Convolution.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvolveWithBias_Planar8 | function | vImage/Convolution.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvolveWithBias_PlanarF | function | vImage/Convolution.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvolve_ARGB16F | function | vImage/Convolution.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvolve_ARGB8888 | function | vImage/Convolution.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvolve_ARGBFFFF | function | vImage/Convolution.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvolve_Planar16F | function | vImage/Convolution.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvolve_Planar8 | function | vImage/Convolution.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvolve_PlanarF | function | vImage/Convolution.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageRichardsonLucyDeConvolve_ARGB8888 | function | vImage/Convolution.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageRichardsonLucyDeConvolve_ARGBFFFF | function | vImage/Convolution.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageRichardsonLucyDeConvolve_Planar8 | function | vImage/Convolution.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageRichardsonLucyDeConvolve_PlanarF | function | vImage/Convolution.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageSepConvolve_ARGB8888 | function | vImage/Convolution.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageSepConvolve_Planar16F | function | vImage/Convolution.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageSepConvolve_Planar16U | function | vImage/Convolution.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageSepConvolve_Planar8 | function | vImage/Convolution.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageSepConvolve_Planar8to16U | function | vImage/Convolution.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageSepConvolve_PlanarF | function | vImage/Convolution.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageTentConvolve_ARGB8888 | function | vImage/Convolution.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageTentConvolve_Planar8 | function | vImage/Convolution.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageAffineWarpCG_ARGB16S | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageAffineWarpCG_ARGB16U | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageAffineWarpCG_ARGB8888 | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageAffineWarpCG_ARGBFFFF | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageAffineWarpCG_Planar8 | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageAffineWarpCG_PlanarF | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageAffineWarpD_ARGB16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageAffineWarpD_ARGB16S | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageAffineWarpD_ARGB16U | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageAffineWarpD_ARGB8888 | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageAffineWarpD_ARGBFFFF | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageAffineWarpD_CbCr16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageAffineWarpD_Planar16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageAffineWarpD_Planar8 | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageAffineWarpD_PlanarF | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageAffineWarp_ARGB16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageAffineWarp_ARGB16S | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageAffineWarp_ARGB16U | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageAffineWarp_ARGB8888 | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageAffineWarp_ARGBFFFF | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageAffineWarp_CbCr16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageAffineWarp_Planar16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageAffineWarp_Planar8 | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageAffineWarp_PlanarF | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageDestroyResamplingFilter | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageGetPerspectiveWarp | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageGetResamplingFilterExtent | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageGetResamplingFilterSize | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalReflect_ARGB16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalReflect_ARGB16S | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalReflect_ARGB16U | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalReflect_ARGB8888 | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalReflect_ARGBFFFF | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalReflect_CbCr16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalReflect_Planar16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalReflect_Planar16U | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalReflect_Planar8 | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalReflect_PlanarF | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalShearD_ARGB16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalShearD_ARGB16S | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalShearD_ARGB16U | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalShearD_ARGB8888 | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalShearD_ARGBFFFF | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalShearD_CbCr16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalShearD_CbCr16S | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalShearD_CbCr16U | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalShearD_Planar16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalShearD_Planar8 | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalShearD_PlanarF | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalShear_ARGB16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalShear_ARGB16S | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalShear_ARGB16U | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalShear_ARGB8888 | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalShear_ARGBFFFF | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalShear_CbCr16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalShear_CbCr16S | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalShear_CbCr16U | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalShear_CbCr8 | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalShear_Planar16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalShear_Planar16S | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalShear_Planar16U | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalShear_Planar8 | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalShear_PlanarF | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHorizontalShear_XRGB2101010W | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageNewResamplingFilter | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageNewResamplingFilterForFunctionUsingBuffer | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePerspectiveWarp_ARGB16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePerspectiveWarp_ARGB16U | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePerspectiveWarp_ARGB8888 | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePerspectiveWarp_Planar16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePerspectiveWarp_Planar16U | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePerspectiveWarp_Planar8 | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageRotate90_ARGB16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageRotate90_ARGB16S | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageRotate90_ARGB16U | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageRotate90_ARGB8888 | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageRotate90_ARGBFFFF | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageRotate90_CbCr16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageRotate90_Planar16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageRotate90_Planar16U | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageRotate90_Planar8 | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageRotate90_PlanarF | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageRotate_ARGB16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageRotate_ARGB16S | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageRotate_ARGB16U | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageRotate_ARGBFFFF | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageRotate_CbCr16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageRotate_Planar16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageRotate_Planar8 | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageRotate_PlanarF | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageScale_ARGB16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageScale_ARGB16S | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageScale_ARGB16U | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageScale_ARGBFFFF | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageScale_CbCr16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageScale_CbCr16U | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageScale_CbCr8 | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageScale_Planar16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageScale_Planar16S | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageScale_Planar16U | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageScale_Planar8 | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageScale_PlanarF | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageScale_XRGB2101010W | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalReflect_ARGB16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalReflect_ARGB16S | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalReflect_ARGB16U | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalReflect_ARGB8888 | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalReflect_ARGBFFFF | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalReflect_CbCr16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalReflect_Planar16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalReflect_Planar16U | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalReflect_Planar8 | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalReflect_PlanarF | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalShearD_ARGB16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalShearD_ARGB16S | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalShearD_ARGB16U | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalShearD_ARGB8888 | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalShearD_ARGBFFFF | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalShearD_CbCr16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalShearD_CbCr16S | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalShearD_CbCr16U | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalShearD_Planar16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalShearD_Planar8 | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalShearD_PlanarF | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalShear_ARGB16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalShear_ARGB16S | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalShear_ARGB16U | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalShear_ARGB8888 | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalShear_ARGBFFFF | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalShear_CbCr16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalShear_CbCr16S | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalShear_CbCr16U | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalShear_CbCr8 | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalShear_Planar16F | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalShear_Planar16S | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalShear_Planar16U | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalShear_Planar8 | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalShear_PlanarF | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageVerticalShear_XRGB2101010W | function | vImage/Geometry.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageContrastStretch_ARGB8888 | function | vImage/Histogram.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageContrastStretch_ARGBFFFF | function | vImage/Histogram.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageContrastStretch_PlanarF | function | vImage/Histogram.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageEndsInContrastStretch_ARGB8888 | function | vImage/Histogram.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageEndsInContrastStretch_ARGBFFFF | function | vImage/Histogram.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageEndsInContrastStretch_Planar8 | function | vImage/Histogram.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageEndsInContrastStretch_PlanarF | function | vImage/Histogram.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageEqualization_ARGB8888 | function | vImage/Histogram.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageEqualization_ARGBFFFF | function | vImage/Histogram.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageEqualization_Planar8 | function | vImage/Histogram.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageEqualization_PlanarF | function | vImage/Histogram.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHistogramCalculation_ARGB8888 | function | vImage/Histogram.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHistogramCalculation_ARGBFFFF | function | vImage/Histogram.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHistogramCalculation_Planar8 | function | vImage/Histogram.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHistogramCalculation_PlanarF | function | vImage/Histogram.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHistogramSpecification_ARGB8888 | function | vImage/Histogram.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHistogramSpecification_ARGBFFFF | function | vImage/Histogram.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHistogramSpecification_Planar8 | function | vImage/Histogram.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageHistogramSpecification_PlanarF | function | vImage/Histogram.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageDilate_ARGB8888 | function | vImage/Morphology.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageDilate_ARGBFFFF | function | vImage/Morphology.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageDilate_Planar8 | function | vImage/Morphology.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageDilate_PlanarF | function | vImage/Morphology.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageErode_ARGB8888 | function | vImage/Morphology.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageErode_ARGBFFFF | function | vImage/Morphology.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageErode_Planar8 | function | vImage/Morphology.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageErode_PlanarF | function | vImage/Morphology.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageMax_ARGB8888 | function | vImage/Morphology.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageMax_ARGBFFFF | function | vImage/Morphology.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageMax_Planar8 | function | vImage/Morphology.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageMax_PlanarF | function | vImage/Morphology.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageMin_ARGB8888 | function | vImage/Morphology.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageMin_ARGBFFFF | function | vImage/Morphology.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageMin_Planar8 | function | vImage/Morphology.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageMin_PlanarF | function | vImage/Morphology.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageCreateGammaFunction | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageDestroyGammaFunction | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageFloodFill_ARGB16U | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageFloodFill_ARGB8888 | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageFloodFill_Planar16U | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageFloodFill_Planar8 | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageGamma_Planar8toPlanarF | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageGamma_PlanarF | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageGamma_PlanarFtoPlanar8 | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageInterpolatedLookupTable_PlanarF | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageLookupTable_8to64U | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageLookupTable_Planar16 | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageLookupTable_Planar8toPlanar128 | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageLookupTable_Planar8toPlanar16 | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageLookupTable_Planar8toPlanar24 | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageLookupTable_Planar8toPlanar48 | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageLookupTable_Planar8toPlanar96 | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageLookupTable_Planar8toPlanarF | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageLookupTable_PlanarFtoPlanar8 | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageMDTableUsageHint | typedef | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageMatrixMultiply_ARGB8888 | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageMatrixMultiply_ARGB8888ToPlanar8 | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageMatrixMultiply_ARGBFFFF | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageMatrixMultiply_ARGBFFFFToPlanarF | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageMatrixMultiply_Planar16S | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageMatrixMultiply_Planar8 | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageMatrixMultiply_PlanarF | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageMultiDimensionalInterpolatedLookupTable_Planar16Q12 | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageMultiDimensionalInterpolatedLookupTable_PlanarF | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageMultidimensionalTable_Create | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageMultidimensionalTable_Release | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageMultidimensionalTable_Retain | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePiecewiseGamma_Planar16Q12 | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePiecewiseGamma_Planar16Q12toPlanar8 | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePiecewiseGamma_Planar8 | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePiecewiseGamma_Planar8toPlanar16Q12 | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePiecewiseGamma_Planar8toPlanarF | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePiecewiseGamma_PlanarF | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePiecewiseGamma_PlanarFtoPlanar8 | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePiecewisePolynomial_Planar8toPlanarF | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePiecewisePolynomial_PlanarF | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePiecewisePolynomial_PlanarFtoPlanar8 | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImagePiecewiseRational_PlanarF | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageSymmetricPiecewiseGamma_Planar16Q12 | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageSymmetricPiecewiseGamma_PlanarF | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageSymmetricPiecewisePolynomial_PlanarF | function | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImage_InterpolationMethod | typedef | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImage_MultidimensionalTable | typedef | vImage/Transform.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageBuffer_CopyToCVPixelBuffer | function | vImage/vImage_CVUtilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageBuffer_InitForCopyFromCVPixelBuffer | function | vImage/vImage_CVUtilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageBuffer_InitForCopyToCVPixelBuffer | function | vImage/vImage_CVUtilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageBuffer_InitWithCVPixelBuffer | function | vImage/vImage_CVUtilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageCVImageFormat_Copy | function | vImage/vImage_CVUtilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageCVImageFormat_CopyChannelDescription | function | vImage/vImage_CVUtilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageCVImageFormat_CopyConversionMatrix | function | vImage/vImage_CVUtilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageCVImageFormat_Create | function | vImage/vImage_CVUtilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageCVImageFormat_CreateWithCVPixelBuffer | function | vImage/vImage_CVUtilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageCVImageFormat_GetAlphaHint | function | vImage/vImage_CVUtilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageCVImageFormat_GetChannelCount | function | vImage/vImage_CVUtilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageCVImageFormat_GetChannelDescription | function | vImage/vImage_CVUtilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageCVImageFormat_GetChannelNames | function | vImage/vImage_CVUtilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageCVImageFormat_GetChromaSiting | function | vImage/vImage_CVUtilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageCVImageFormat_GetColorSpace | function | vImage/vImage_CVUtilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageCVImageFormat_GetConversionMatrix | function | vImage/vImage_CVUtilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageCVImageFormat_GetFormatCode | function | vImage/vImage_CVUtilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageCVImageFormat_GetUserData | function | vImage/vImage_CVUtilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageCVImageFormat_Release | function | vImage/vImage_CVUtilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageCVImageFormat_Retain | function | vImage/vImage_CVUtilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageCVImageFormat_SetAlphaHint | function | vImage/vImage_CVUtilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageCVImageFormat_SetChromaSiting | function | vImage/vImage_CVUtilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageCVImageFormat_SetColorSpace | function | vImage/vImage_CVUtilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageCVImageFormat_SetUserData | function | vImage/vImage_CVUtilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageChannelDescription | typedef | vImage/vImage_CVUtilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConverter_CreateForCGToCVImageFormat | function | vImage/vImage_CVUtilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConverter_CreateForCVToCGImageFormat | function | vImage/vImage_CVUtilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageCreateMonochromeColorSpaceWithWhitePointAndTransferFunction | function | vImage/vImage_CVUtilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageCreateRGBColorSpaceWithPrimariesAndTransferFunction | function | vImage/vImage_CVUtilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageRGBPrimaries | typedef | vImage/vImage_CVUtilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageTransferFunction | typedef | vImage/vImage_CVUtilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageWhitePoint | typedef | vImage/vImage_CVUtilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| kvImage_ARGBToYpCbCrMatrix_ITU_R_601_4 | const | vImage/vImage_Types.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| kvImage_ARGBToYpCbCrMatrix_ITU_R_709_2 | const | vImage/vImage_Types.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| kvImage_YpCbCrToARGBMatrix_ITU_R_601_4 | const | vImage/vImage_Types.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| kvImage_YpCbCrToARGBMatrix_ITU_R_709_2 | const | vImage/vImage_Types.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageARGBType | typedef | vImage/vImage_Types.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageCVImageFormatRef | typedef | vImage/vImage_Types.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConstCVImageFormatRef | typedef | vImage/vImage_Types.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConverterRef | typedef | vImage/vImage_Types.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageYpCbCrType | typedef | vImage/vImage_Types.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImage_ARGBToYpCbCr | typedef | vImage/vImage_Types.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImage_ARGBToYpCbCrMatrix | typedef | vImage/vImage_Types.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImage_AffineTransform | typedef | vImage/vImage_Types.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImage_AffineTransform_Double | typedef | vImage/vImage_Types.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImage_PerpsectiveTransform | typedef | vImage/vImage_Types.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImage_YpCbCrPixelRange | typedef | vImage/vImage_Types.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImage_YpCbCrToARGB | typedef | vImage/vImage_Types.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImage_YpCbCrToARGBMatrix | typedef | vImage/vImage_Types.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| kvImageDecodeArray_16Q12Format | const | vImage/vImage_Utilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageBuffer_GetSize | function | vImage/vImage_Utilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageBuffer_Init | function | vImage/vImage_Utilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageBuffer_InitWithCGImage | function | vImage/vImage_Utilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageCGImageFormat_GetComponentCount | function | vImage/vImage_Utilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageCGImageFormat_IsEqual | function | vImage/vImage_Utilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConvert_AnyToAny | function | vImage/vImage_Utilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConverter_CreateWithCGColorConversionInfo | function | vImage/vImage_Utilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConverter_CreateWithCGImageFormat | function | vImage/vImage_Utilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConverter_CreateWithColorSyncCodeFragment | function | vImage/vImage_Utilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConverter_GetDestinationBufferOrder | function | vImage/vImage_Utilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConverter_GetNumberOfDestinationBuffers | function | vImage/vImage_Utilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConverter_GetNumberOfSourceBuffers | function | vImage/vImage_Utilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConverter_GetSourceBufferOrder | function | vImage/vImage_Utilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConverter_MustOperateOutOfPlace | function | vImage/vImage_Utilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConverter_Release | function | vImage/vImage_Utilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageConverter_Retain | function | vImage/vImage_Utilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImageCreateCGImageFromBuffer | function | vImage/vImage_Utilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| vImage_CGImageFormat | typedef | vImage/vImage_Utilities.h | No wrapper; crate focuses on common ARGB8888/Planar8 vImage workflows |
| BNNSCopy | function | vecLib/BNNS/bnns.h | No wrapper; BNNS descriptors and graph-adjacent APIs unsupported |
| BNNSCreateNearestNeighbors | function | vecLib/BNNS/bnns.h | No wrapper; BNNS descriptors and graph-adjacent APIs unsupported |
| BNNSCreateRandomGenerator | function | vecLib/BNNS/bnns.h | No wrapper; BNNS descriptors and graph-adjacent APIs unsupported |
| BNNSCreateRandomGeneratorWithSeed | function | vecLib/BNNS/bnns.h | No wrapper; BNNS descriptors and graph-adjacent APIs unsupported |
| BNNSDataLayoutGetRank | function | vecLib/BNNS/bnns.h | No wrapper; BNNS descriptors and graph-adjacent APIs unsupported |
| BNNSDestroyNearestNeighbors | function | vecLib/BNNS/bnns.h | No wrapper; BNNS descriptors and graph-adjacent APIs unsupported |
| BNNSDestroyRandomGenerator | function | vecLib/BNNS/bnns.h | No wrapper; BNNS descriptors and graph-adjacent APIs unsupported |
| BNNSDirectApplyInTopK | function | vecLib/BNNS/bnns.h | No wrapper; BNNS descriptors and graph-adjacent APIs unsupported |
| BNNSDirectApplyReduction | function | vecLib/BNNS/bnns.h | No wrapper; BNNS descriptors and graph-adjacent APIs unsupported |
| BNNSDirectApplyTopK | function | vecLib/BNNS/bnns.h | No wrapper; BNNS descriptors and graph-adjacent APIs unsupported |
| BNNSNDArrayGetDataSize | function | vecLib/BNNS/bnns.h | No wrapper; BNNS descriptors and graph-adjacent APIs unsupported |
| BNNSNearestNeighborsGetInfo | function | vecLib/BNNS/bnns.h | No wrapper; BNNS descriptors and graph-adjacent APIs unsupported |
| BNNSNearestNeighborsLoad | function | vecLib/BNNS/bnns.h | No wrapper; BNNS descriptors and graph-adjacent APIs unsupported |
| BNNSRandomFillCategoricalFloat | function | vecLib/BNNS/bnns.h | No wrapper; BNNS descriptors and graph-adjacent APIs unsupported |
| BNNSRandomFillNormalFloat | function | vecLib/BNNS/bnns.h | No wrapper; BNNS descriptors and graph-adjacent APIs unsupported |
| BNNSRandomFillUniformFloat | function | vecLib/BNNS/bnns.h | No wrapper; BNNS descriptors and graph-adjacent APIs unsupported |
| BNNSRandomFillUniformInt | function | vecLib/BNNS/bnns.h | No wrapper; BNNS descriptors and graph-adjacent APIs unsupported |
| BNNSRandomGeneratorGetState | function | vecLib/BNNS/bnns.h | No wrapper; BNNS descriptors and graph-adjacent APIs unsupported |
| BNNSRandomGeneratorSetState | function | vecLib/BNNS/bnns.h | No wrapper; BNNS descriptors and graph-adjacent APIs unsupported |
| BNNSRandomGeneratorStateSize | function | vecLib/BNNS/bnns.h | No wrapper; BNNS descriptors and graph-adjacent APIs unsupported |
| BNNSTensorGetAllocationSize | function | vecLib/BNNS/bnns.h | No wrapper; BNNS descriptors and graph-adjacent APIs unsupported |
| BNNSTranspose | function | vecLib/BNNS/bnns.h | No wrapper; BNNS descriptors and graph-adjacent APIs unsupported |
| BNNSGraphCompileFromFile | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSGraphCompileOptionsGetOutputFD | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSGraphCompileOptionsGetOutputPath | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSGraphCompileOptionsSetMessageLogCallback | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSGraphCompileOptionsSetMessageLogMask | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSGraphCompileOptionsSetOutputFD | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSGraphCompileOptionsSetOutputPath | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSGraphContextDestroy | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSGraphContextEnableNanAndInfChecks | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSGraphContextExecute | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSGraphContextGetTensor | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSGraphContextGetWorkspaceSize | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSGraphContextMake | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSGraphContextMakeStreaming | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSGraphContextSetArgumentType | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSGraphContextSetBatchSize | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSGraphContextSetDynamicShapes | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSGraphContextSetMessageLogCallback | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSGraphContextSetMessageLogMask | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSGraphContextSetOutputAllocationCallback | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSGraphContextSetStreamingAdvanceCount | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSGraphContextSetWorkspaceAllocationCallback | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSGraphGetArgumentCount | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSGraphGetArgumentIntents | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSGraphGetArgumentInterleaveFactors | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSGraphGetArgumentNames | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSGraphGetArgumentPosition | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSGraphGetFunctionCount | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSGraphGetFunctionNames | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSGraphGetInputCount | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSGraphGetInputNames | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSGraphGetOutputCount | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSGraphGetOutputNames | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSGraphTensorFillStrides | function | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| bnns_graph_argument_t | typedef | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| bnns_graph_context_t | typedef | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| bnns_graph_shape_t | typedef | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| bnns_graph_t | typedef | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| bnns_user_message_data_t | typedef | vecLib/BNNS/bnns_graph.h | No wrapper; crate only wraps BNNS Graph compile-options helpers |
| BNNSActivation | typedef | vecLib/BNNS/bnns_structures.h | No wrapper; BNNS descriptors and graph-adjacent APIs unsupported |
| BNNSFilterParameters | typedef | vecLib/BNNS/bnns_structures.h | No wrapper; BNNS descriptors and graph-adjacent APIs unsupported |
| BNNSLayerParametersReduction | typedef | vecLib/BNNS/bnns_structures.h | No wrapper; BNNS descriptors and graph-adjacent APIs unsupported |
| BNNSMHAProjectionParameters | typedef | vecLib/BNNS/bnns_structures.h | No wrapper; BNNS descriptors and graph-adjacent APIs unsupported |
| BNNSNDArrayDescriptor | typedef | vecLib/BNNS/bnns_structures.h | No wrapper; BNNS descriptors and graph-adjacent APIs unsupported |
| BNNSSparsityParameters | typedef | vecLib/BNNS/bnns_structures.h | No wrapper; BNNS descriptors and graph-adjacent APIs unsupported |
| BNNSTensor | typedef | vecLib/BNNS/bnns_structures.h | No wrapper; BNNS descriptors and graph-adjacent APIs unsupported |
| la_object_t | typedef | vecLib/LinearAlgebra/object.h | No wrapper; current LinearAlgebra APIs unsupported |
| sparse_elementwise_norm_double | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_elementwise_norm_double_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_elementwise_norm_float | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_elementwise_norm_float_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_extract_block_double | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_extract_block_double_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_extract_block_float | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_extract_block_float_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_extract_sparse_column_double | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_extract_sparse_column_double_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_extract_sparse_column_float | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_extract_sparse_column_float_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_extract_sparse_row_double | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_extract_sparse_row_double_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_extract_sparse_row_float | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_extract_sparse_row_float_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_get_block_dimension_for_col | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_get_block_dimension_for_row | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_get_matrix_nonzero_count_for_column | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_get_matrix_nonzero_count_for_row | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_get_matrix_property | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_get_vector_nonzero_count_double | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_get_vector_nonzero_count_double_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_get_vector_nonzero_count_float | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_get_vector_nonzero_count_float_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_inner_product_dense_double | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_inner_product_dense_double_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_inner_product_dense_float_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_inner_product_sparse_double | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_inner_product_sparse_double_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_inner_product_sparse_float_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_insert_block_double | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_insert_block_double_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_insert_block_float | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_insert_block_float_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_insert_col_double | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_insert_col_double_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_insert_col_float | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_insert_col_float_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_insert_entries_double | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_insert_entries_double_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_insert_entries_float | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_insert_entries_float_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_insert_entry_double | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_insert_entry_double_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_insert_entry_float_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_insert_row_double | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_insert_row_double_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_insert_row_float | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_insert_row_float_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_matrix_block_create_double | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_matrix_block_create_double_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_matrix_block_create_float | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_matrix_block_create_float_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_matrix_create_double | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_matrix_create_double_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_matrix_create_float_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_matrix_product_dense_double | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_matrix_product_dense_double_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_matrix_product_dense_float | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_matrix_product_dense_float_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_matrix_product_sparse_double | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_matrix_product_sparse_double_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_matrix_product_sparse_float | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_matrix_product_sparse_float_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_matrix_trace_double | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_matrix_trace_double_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_matrix_trace_float | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_matrix_trace_float_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_matrix_triangular_solve_dense_double | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_matrix_triangular_solve_dense_double_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_matrix_triangular_solve_dense_float_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_matrix_variable_block_create_double | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_matrix_variable_block_create_double_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_matrix_variable_block_create_float | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_matrix_variable_block_create_float_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_matrix_vector_product_dense_double | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_matrix_vector_product_dense_double_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_matrix_vector_product_dense_float | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_matrix_vector_product_dense_float_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_operator_norm_double | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_operator_norm_double_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_operator_norm_float | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_operator_norm_float_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_outer_product_dense_double | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_outer_product_dense_double_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_outer_product_dense_float | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_outer_product_dense_float_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_pack_vector_double | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_pack_vector_double_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_pack_vector_float | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_pack_vector_float_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_permute_cols_double | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_permute_cols_double_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_permute_cols_float | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_permute_cols_float_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_permute_rows_double | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_permute_rows_double_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_permute_rows_float | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_permute_rows_float_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_unpack_vector_double | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_unpack_vector_double_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_unpack_vector_float | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_unpack_vector_float_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_vector_add_with_scale_dense_double | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_vector_add_with_scale_dense_double_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_vector_add_with_scale_dense_float_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_vector_norm_double | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_vector_norm_double_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_vector_norm_float | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_vector_norm_float_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_vector_triangular_solve_dense_double | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_vector_triangular_solve_dense_double_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_vector_triangular_solve_dense_float_complex | function | vecLib/Sparse/BLAS.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| DenseMatrix_Complex_Double | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| DenseMatrix_Complex_Float | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| DenseMatrix_Double | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| DenseMatrix_Float | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| DenseVector_Complex_Double | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| DenseVector_Complex_Float | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| DenseVector_Double | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| DenseVector_Float | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseAttributesComplex_t | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseAttributes_t | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseCGOptions | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseCleanup | function | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseConjugateGradient | function | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseConvertFromCoordinate | function | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseConvertFromOpaque | function | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseCreatePreconditioner | function | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseCreateSubfactor | function | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseFactor | function | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseGMRES | function | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseGMRESOptions | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseGetConjugateTranspose | function | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseGetInertia | function | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseGetStateSize_Complex_Double | function | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseGetStateSize_Complex_Float | function | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseGetStateSize_Double | function | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseGetStateSize_Float | function | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseGetTranspose | function | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseIterate | function | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseIterativeMethod | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseLSMR | function | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseLSMROptions | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseMatrixStructure | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseMatrixStructureComplex | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseMatrix_Complex_Double | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseMatrix_Complex_Float | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseMatrix_Double | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseMatrix_Float | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseMultiply | function | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseMultiplyAdd | function | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseNumericFactorOptions | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseOpaqueFactorization_Complex_Double | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseOpaqueFactorization_Complex_Float | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseOpaqueFactorization_Double | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseOpaqueFactorization_Float | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseOpaquePreconditioner_Complex_Double | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseOpaquePreconditioner_Complex_Float | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseOpaquePreconditioner_Double | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseOpaquePreconditioner_Float | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseOpaqueSubfactor_Complex_Double | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseOpaqueSubfactor_Complex_Float | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseOpaqueSubfactor_Double | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseOpaqueSubfactor_Float | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseOpaqueSymbolicFactorization | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseRefactor | function | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseRetain | function | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseSolve | function | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseSymbolicFactorOptions | typedef | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SparseUpdateFactor | function | vecLib/Sparse/Solve.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| SPARSE_ENUM | function | vecLib/Sparse/SolveImplementation.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseDestroyOpaqueSymbolic | function | vecLib/Sparse/SolveImplementation.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseFromAttributeComplex | function | vecLib/Sparse/SolveImplementation.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseFromKindComplex | function | vecLib/Sparse/SolveImplementation.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseFromStructureComplex | function | vecLib/Sparse/SolveImplementation.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseGetOptionsFromSymbolicFactor | function | vecLib/Sparse/SolveImplementation.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseRetainSymbolic | function | vecLib/Sparse/SolveImplementation.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseSymbolicFactorLU | function | vecLib/Sparse/SolveImplementation.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseSymbolicFactorQR | function | vecLib/Sparse/SolveImplementation.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseSymbolicFactorSymmetric | function | vecLib/Sparse/SolveImplementation.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseToAttributeComplex | function | vecLib/Sparse/SolveImplementation.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseToKindComplex | function | vecLib/Sparse/SolveImplementation.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseToStructureComplex | function | vecLib/Sparse/SolveImplementation.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseTrap | function | vecLib/Sparse/SolveImplementation.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| API_AVAILABLE | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _DenseMatrixFromVector_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _DenseMatrixFromVector_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _DenseMatrixFromVector_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _DenseMatrixFromVector_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SPARSE_VARIANT | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseCGIterate_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseCGIterate_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseCGIterate_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseCGIterate_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseCGSolve_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseCGSolve_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseCGSolve_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseCGSolve_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseConvertFromCoordinate_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseConvertFromCoordinate_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseConvertFromCoordinate_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseConvertFromCoordinate_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseConvertFromOpaque_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseConvertFromOpaque_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseConvertFromOpaque_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseConvertFromOpaque_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseCreatePreconditioner_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseCreatePreconditioner_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseCreatePreconditioner_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseCreatePreconditioner_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseDestroyOpaqueNumeric_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseDestroyOpaqueNumeric_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseDestroyOpaqueNumeric_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseDestroyOpaqueNumeric_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseFactorHermitian_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseFactorHermitian_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseFactorLU_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseFactorLU_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseFactorLU_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseFactorLU_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseFactorQR_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseFactorQR_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseFactorQR_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseFactorQR_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseFactorSymmetric_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseFactorSymmetric_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseFactorSymmetric_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseFactorSymmetric_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseFailedFactor_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseFailedFactor_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseFailedFactor_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseFailedFactor_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseGMRESIterate_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseGMRESIterate_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseGMRESIterate_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseGMRESIterate_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseGMRESSolve_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseGMRESSolve_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseGMRESSolve_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseGMRESSolve_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseGetIterativeStateSize_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseGetIterativeStateSize_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseGetIterativeStateSize_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseGetIterativeStateSize_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseGetOptionsFromNumericFactor_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseGetOptionsFromNumericFactor_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseGetOptionsFromNumericFactor_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseGetOptionsFromNumericFactor_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseGetWorkspaceRequired_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseGetWorkspaceRequired_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseGetWorkspaceRequired_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseGetWorkspaceRequired_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseInvalidSubfactor_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseInvalidSubfactor_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseInvalidSubfactor_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseInvalidSubfactor_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseLSMRIterate_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseLSMRIterate_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseLSMRIterate_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseLSMRIterate_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseLSMRSolve_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseLSMRSolve_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseLSMRSolve_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseLSMRSolve_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseMultiplySubfactor_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseMultiplySubfactor_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseMultiplySubfactor_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseMultiplySubfactor_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseNumericFactorHermitian_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseNumericFactorHermitian_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseNumericFactorLU_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseNumericFactorLU_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseNumericFactorLU_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseNumericFactorLU_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseNumericFactorQR_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseNumericFactorQR_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseNumericFactorQR_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseNumericFactorQR_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseNumericFactorSymmetric_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseNumericFactorSymmetric_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseNumericFactorSymmetric_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseNumericFactorSymmetric_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseRefactorHermitian_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseRefactorHermitian_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseRefactorLU_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseRefactorLU_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseRefactorLU_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseRefactorLU_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseRefactorQR_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseRefactorQR_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseRefactorQR_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseRefactorQR_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseRefactorSymmetric_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseRefactorSymmetric_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseRefactorSymmetric_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseRefactorSymmetric_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseReleaseOpaquePreconditioner_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseReleaseOpaquePreconditioner_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseReleaseOpaquePreconditioner_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseReleaseOpaquePreconditioner_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseRetainNumeric_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseRetainNumeric_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseRetainNumeric_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseRetainNumeric_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseSolveOpaque_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseSolveOpaque_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseSolveOpaque_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseSolveOpaque_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseSolveSubfactor_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseSolveSubfactor_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseSolveSubfactor_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseSolveSubfactor_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseSpMV_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseSpMV_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseSpMV_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseSpMV_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseSubFactorGetDimn_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseSubFactorGetDimn_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseSubFactorGetDimn_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseSubFactorGetDimn_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseUpdatePartialRefactorLU_Complex_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseUpdatePartialRefactorLU_Complex_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseUpdatePartialRefactorLU_Double | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| _SparseUpdatePartialRefactorLU_Float | function | vecLib/Sparse/SolveImplementationTyped.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_matrix_double | typedef | vecLib/Sparse/Types.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_matrix_double_complex | typedef | vecLib/Sparse/Types.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_matrix_float_complex | typedef | vecLib/Sparse/Types.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| sparse_norm | typedef | vecLib/Sparse/Types.h | No wrapper; crate only wraps core sparse vector ops plus basic matrix/triangular solve helpers |
| __CLPK_complex | typedef | vecLib/clapack.h | No wrapper; modern BLAS/LAPACK entry points unsupported |
| __CLPK_doublecomplex | typedef | vecLib/clapack.h | No wrapper; modern BLAS/LAPACK entry points unsupported |
| BLASGetThreading | function | vecLib/thread_api.h | No wrapper in crate |
| BLASSetThreading | function | vecLib/thread_api.h | No wrapper in crate |
| vA128Shift | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vA64Shift | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vA64Shift2 | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vL128Rotate | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vL64Rotate | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vL64Rotate2 | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vLL128Shift | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vLL64Shift | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vLL64Shift2 | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vLR128Shift | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vLR64Shift | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vLR64Shift2 | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vR128Rotate | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vR64Rotate | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vR64Rotate2 | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vS128Add | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vS128AddS | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vS128Divide | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vS128HalfMultiply | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vS128Neg | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vS128Sub | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vS128SubS | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vS16Divide | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vS16HalfMultiply | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vS32Divide | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vS32FullMulEven | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vS32FullMulOdd | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vS32HalfMultiply | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vS64Add | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vS64AddS | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vS64Divide | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vS64FullMulEven | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vS64FullMulOdd | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vS64HalfMultiply | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vS64Neg | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vS64Sub | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vS64SubS | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vS8Divide | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vS8HalfMultiply | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vU128Add | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vU128AddS | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vU128Divide | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vU128HalfMultiply | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vU128Neg | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vU128Sub | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vU128SubS | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vU16Divide | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vU16HalfMultiply | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vU32Divide | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vU32FullMulEven | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vU32FullMulOdd | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vU32HalfMultiply | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vU64Add | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vU64AddS | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vU64Divide | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vU64FullMulEven | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vU64FullMulOdd | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vU64HalfMultiply | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vU64Neg | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vU64Sub | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vU64SubS | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vU8Divide | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vU8HalfMultiply | function | vecLib/vBasicOps.h | No wrapper; vBasicOps family unsupported |
| vA1024Shift | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vA256Shift | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vA512Shift | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vL1024Rotate | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vL256Rotate | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vL512Rotate | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vLL1024Shift | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vLL256Shift | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vLL512Shift | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vLR1024Shift | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vLR256Shift | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vLR512Shift | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vR1024Rotate | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vR256Rotate | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vR512Rotate | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vS1024 | typedef | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vS1024Add | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vS1024AddS | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vS1024Divide | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vS1024HalfMultiply | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vS1024Mod | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vS1024Neg | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vS1024Sub | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vS1024SubS | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vS128 | typedef | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vS128FullMultiply | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vS256 | typedef | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vS256Add | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vS256AddS | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vS256Divide | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vS256FullMultiply | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vS256HalfMultiply | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vS256Mod | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vS256Neg | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vS256Sub | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vS256SubS | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vS512 | typedef | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vS512Add | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vS512AddS | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vS512Divide | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vS512FullMultiply | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vS512HalfMultiply | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vS512Mod | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vS512Neg | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vS512Sub | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vS512SubS | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vU1024 | typedef | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vU1024Add | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vU1024AddS | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vU1024Divide | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vU1024HalfMultiply | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vU1024Mod | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vU1024Neg | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vU1024Sub | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vU1024SubS | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vU128 | typedef | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vU128FullMultiply | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vU256 | typedef | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vU256Add | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vU256AddS | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vU256Divide | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vU256FullMultiply | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vU256HalfMultiply | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vU256Mod | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vU256Neg | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vU256Sub | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vU256SubS | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vU512 | typedef | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vU512Add | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vU512AddS | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vU512Divide | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vU512FullMultiply | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vU512HalfMultiply | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vU512Mod | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vU512Neg | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vU512Sub | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| vU512SubS | function | vecLib/vBigNum.h | No wrapper; vBigNum family unsupported |
| DSPComplex | typedef | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| DSPDoubleComplex | typedef | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| DSPDoubleSplitComplex | typedef | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| FFTSetupD | typedef | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_DCT_CreateSetup | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_DCT_Execute | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_DFT_CreateSetup | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_DFT_DestroySetup | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_DFT_DestroySetupD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_DFT_Execute | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_DFT_ExecuteD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_DFT_Interleaved_CreateSetup | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_DFT_Interleaved_CreateSetupD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_DFT_Interleaved_DestroySetup | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_DFT_Interleaved_DestroySetupD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_DFT_Interleaved_Execute | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_DFT_Interleaved_ExecuteD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_DFT_Interleaved_Setup | typedef | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_DFT_Interleaved_SetupD | typedef | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_DFT_Setup | typedef | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_DFT_SetupD | typedef | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_DFT_zop | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_DFT_zop_CreateSetup | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_DFT_zop_CreateSetupD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_DFT_zrop_CreateSetup | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_DFT_zrop_CreateSetupD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_FFT16_copv | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_FFT16_zopv | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_FFT32_copv | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_FFT32_zopv | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_biquadD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_biquad_CreateSetupD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_biquad_DestroySetupD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_biquad_SetCoefficientsDouble | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_biquad_SetCoefficientsSingle | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_biquad_SetupD | typedef | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_biquadm | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_biquadmD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_biquadm_CopyState | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_biquadm_CopyStateD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_biquadm_CreateSetup | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_biquadm_CreateSetupD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_biquadm_DestroySetup | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_biquadm_DestroySetupD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_biquadm_ResetState | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_biquadm_ResetStateD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_biquadm_SetActiveFilters | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_biquadm_SetActiveFiltersD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_biquadm_SetCoefficientsDouble | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_biquadm_SetCoefficientsDoubleD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_biquadm_SetCoefficientsSingle | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_biquadm_SetCoefficientsSingleD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_biquadm_SetTargetsDouble | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_biquadm_SetTargetsDoubleD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_biquadm_SetTargetsSingle | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_biquadm_SetTargetsSingleD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_biquadm_Setup | typedef | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_biquadm_SetupD | typedef | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_conv | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_convD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_create_fftsetupD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_ctoz | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_ctozD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_deq22 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_deq22D | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_desamp | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_desampD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_destroy_fftsetupD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_distancesq | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_distancesqD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_dotpr2 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_dotpr2D | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_dotpr2_s1_15 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_dotpr2_s8_24 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_dotpr_s1_15 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_dotpr_s8_24 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_f3x3 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_f3x3D | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_f5x5 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_f5x5D | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fft2d_zip | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fft2d_zipD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fft2d_zipt | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fft2d_ziptD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fft2d_zop | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fft2d_zopD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fft2d_zopt | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fft2d_zoptD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fft2d_zrip | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fft2d_zripD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fft2d_zript | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fft2d_zriptD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fft2d_zrop | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fft2d_zropD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fft2d_zropt | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fft2d_zroptD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fft_zipD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fft_zipt | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fft_ziptD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fft_zop | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fft_zopD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fft_zopt | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fft_zoptD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fft_zrip | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fft_zripD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fft_zript | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fft_zriptD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fft_zrop | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fft_zropD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fft_zropt | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fft_zroptD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fftm_zip | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fftm_zipD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fftm_zipt | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fftm_ziptD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fftm_zop | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fftm_zopD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fftm_zopt | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fftm_zoptD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fftm_zrip | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fftm_zripD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fftm_zript | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fftm_zriptD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fftm_zrop | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fftm_zropD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fftm_zropt | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_fftm_zroptD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_hann_window | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_hann_windowD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_imgfir | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_imgfirD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_int24 | typedef | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_maxmgv | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_maxmgvD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_maxmgvi | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_maxmgviD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_maxvi | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_maxviD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_meamgv | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_meamgvD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_measqv | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_measqvD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_minmgv | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_minmgvD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_minmgvi | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_minmgviD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_minvi | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_minviD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_mmov | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_mmovD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_mmul | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_mmulD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_mtrans | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_mtransD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_mvessq | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_mvessqD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_normalize | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_normalizeD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_nzcros | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_nzcrosD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_polar | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_polarD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_rect | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_rectD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_rmsqv | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_rmsqvD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_svdiv | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_svdivD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_sve_svesq | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_sve_svesqD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_svemg | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_svemgD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_svesq | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_svesqD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_svs | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_svsD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_uint24 | typedef | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vaam | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vaamD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vabs | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vabsD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vabsi | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vaddi | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vaddsub | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vaddsubD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vam | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vamD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vasbm | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vasbmD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vasm | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vasmD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vavlin | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vavlinD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vclip | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vclipD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vclipc | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vclipcD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vclr | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vclrD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vcmprs | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vcmprsD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vdbcon | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vdbconD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vdist | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vdistD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vdiv | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vdivD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vdivi | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vdpsp | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_venvlp | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_venvlpD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_veqvi | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfill | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfillD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfilli | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfix16 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfix16D | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfix32 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfix32D | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfix8 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfix8D | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfixr16 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfixr16D | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfixr32 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfixr32D | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfixr8 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfixr8D | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfixru16 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfixru16D | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfixru32 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfixru32D | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfixru8 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfixru8D | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfixu16 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfixu16D | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfixu32 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfixu32D | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfixu8 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfixu8D | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vflt16 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vflt16D | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vflt24 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vflt32 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vflt32D | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vflt8 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vflt8D | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfltsm24 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfltsmu24 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfltu16 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfltu16D | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfltu24 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfltu32 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfltu32D | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfltu8 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfltu8D | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfrac | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vfracD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vgathr | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vgathrD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vgathra | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vgathraD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vgen | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vgenD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vgenp | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vgenpD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_viclip | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_viclipD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vindex | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vindexD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vintb | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vintbD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vlim | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vlimD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vlint | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vlintD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vma | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vmaD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vmax | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vmaxD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vmaxmg | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vmaxmgD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vmin | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vminD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vminmg | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vminmgD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vmma | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vmmaD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vmmsb | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vmmsbD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vmsa | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vmsaD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vmsb | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vmsbD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vmul | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vmulD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vnabs | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vnabsD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vneg | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vnegD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vpoly | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vpolyD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vpythg | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vpythgD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vqint | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vqintD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vramp | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vrampD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vrampmul | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vrampmul2 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vrampmul2D | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vrampmul2_s1_15 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vrampmul2_s8_24 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vrampmulD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vrampmul_s1_15 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vrampmul_s8_24 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vrampmuladd | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vrampmuladd2 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vrampmuladd2D | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vrampmuladd2_s1_15 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vrampmuladd2_s8_24 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vrampmuladdD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vrampmuladd_s1_15 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vrampmuladd_s8_24 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vrsum | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vrsumD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vrvrs | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vrvrsD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vsadd | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vsaddD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vsaddi | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vsbm | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vsbmD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vsbsbm | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vsbsbmD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vsbsm | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vsbsmD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vsdiv | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vsdivD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vsdivi | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vsimps | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vsimpsD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vsma | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vsmaD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vsmfix24 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vsmfixu24 | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vsmsa | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vsmsaD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vsmsb | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vsmsbD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vsmsma | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vsmsmaD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vsmul | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vsmulD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vsort | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vsortD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vsorti | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vsortiD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vspdp | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vsq | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vsqD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vssq | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vssqD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vswap | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vswapD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vswmax | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vswmaxD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vswsum | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vswsumD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vtabi | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vtabiD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vthr | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vthrD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vthres | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vthresD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vthrsc | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vthrscD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vtmerg | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vtmergD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vtrapz | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_vtrapzD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_wiener | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_wienerD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zaspec | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zaspecD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zcoher | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zcoherD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zconv | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zconvD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zcspec | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zcspecD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zdotpr | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zdotprD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zidotpr | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zidotprD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zmma | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zmmaD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zmms | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zmmsD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zmmul | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zmmulD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zmsm | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zmsmD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zrdesamp | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zrdesampD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zrdotpr | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zrdotprD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zrvadd | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zrvaddD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zrvdiv | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zrvdivD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zrvmul | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zrvmulD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zrvsub | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zrvsubD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_ztoc | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_ztocD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_ztrans | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_ztransD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvabs | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvabsD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvadd | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvaddD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvcma | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvcmaD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvcmul | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvcmulD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvconj | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvconjD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvdiv | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvdivD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvfill | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvfillD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvma | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvmaD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvmags | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvmagsD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvmgsa | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvmgsaD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvmmaa | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvmmaaD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvmov | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvmovD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvmul | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvmulD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvneg | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvnegD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvphas | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvphasD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvsma | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvsmaD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvsub | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvsubD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvzsml | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vDSP_zvzsmlD | function | vecLib/vDSP.h | No wrapper; crate covers common f32/f64 vector ops plus FFT/biquad helpers |
| vvacos | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvacosf | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvacosh | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvacoshf | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvasin | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvasinf | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvasinh | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvasinhf | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvatan | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvatan2 | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvatan2f | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvatanf | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvatanh | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvatanhf | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvcbrt | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvcbrtf | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvceil | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvceilf | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvcopysign | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvcopysignf | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvcos | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvcosh | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvcoshf | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvcosisin | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvcosisinf | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvcospi | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvcospif | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvdiv | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvdivf | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvexp | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvexp2 | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvexp2f | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvexpm1 | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvexpm1f | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvfabs | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvfabsf | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvfloor | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvfloorf | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvfmod | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvfmodf | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvint | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvintf | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvlog | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvlog10 | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvlog10f | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvlog1p | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvlog1pf | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvlog2 | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvlog2f | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvlogb | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvlogbf | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvnextafter | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvnextafterf | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvnint | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvnintf | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvpow | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvpowf | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvpows | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvpowsf | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvrec | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvrecf | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvremainder | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvremainderf | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvrsqrt | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvrsqrtf | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvsin | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvsincos | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvsincosf | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvsinh | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvsinhf | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvsinpi | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvsinpif | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvsqrt | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvtan | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvtanf | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvtanh | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvtanhf | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvtanpi | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vvtanpif | function | vecLib/vForce.h | No wrapper; crate only exposes sin/cos/exp/log/sqrt for f32 |
| vacosf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vacoshf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vasinf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vasinhf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vatan2f | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vatanf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vatanhf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vceilf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vclassifyf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vcopysignf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vcosf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vcoshf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vcospif | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vdivf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vexp2f | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vexpf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vexpm1f | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vfabsf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vfloorf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vfmodf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vipowf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vlog10f | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vlog1pf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vlog2f | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vlogbf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vlogf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vnextafterf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vnintf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vpowf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vrecf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vremainderf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vremquof | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vrsqrtf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vscalbf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vsignbitf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vsincosf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vsinf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vsinhf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vsinpif | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vsqrtf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vtablelookup | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vtanf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vtanhf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vtanpif | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |
| vtruncf | function | vecLib/vfp.h | No wrapper; vector floating-point utility family unsupported |

## ⏭️ EXEMPT

| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| BNNSApplyMultiheadAttention | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSApplyMultiheadAttentionBackward | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSArithmeticFilterApplyBackwardBatch | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSArithmeticFilterApplyBatch | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSBandPart | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 13.0→15.0; Use BNNSGraph* APIs |
| BNNSClipByGlobalNorm | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 12.0→15.0; Use BNNSGraph* APIs |
| BNNSClipByNorm | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 12.0→15.0; Use BNNSGraph* APIs |
| BNNSClipByValue | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 12.0→15.0; Use BNNSGraph* APIs |
| BNNSCompareTensor | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSComputeLSTMTrainingCacheCapacity | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSComputeNorm | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 12.0→15.0; Use BNNSGraph* APIs |
| BNNSComputeNormBackward | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 12.0→15.0; Use BNNSGraph* APIs |
| BNNSCropResize | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 13.0→15.0; Use BNNSGraph* APIs |
| BNNSCropResizeBackward | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 13.0→15.0; Use BNNSGraph* APIs |
| BNNSDirectApplyActivationBatch | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSDirectApplyBroadcastMatMul | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→13.0; BNNSMatMul |
| BNNSDirectApplyLSTMBatchBackward | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSDirectApplyLSTMBatchTrainingCaching | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSDirectApplyQuantizer | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 12.0→15.0; Use BNNSGraph* APIs |
| BNNSFilterApply | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. Crate reaches it via BnnsFilter::apply, bnns_relu_f32, bnns_sigmoid_f32, ffi::BNNSFilterApply (raw-ffi). | macos 10.12→15.0; Use BNNSGraph* APIs |
| BNNSFilterApplyBackwardBatch | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSFilterApplyBackwardTwoInputBatch | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSFilterApplyBatch | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 10.12→15.0; Use BNNSGraph* APIs |
| BNNSFilterApplyTwoInput | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSFilterApplyTwoInputBatch | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSFilterCreateConvolutionLayer | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 10.12→11.0; BNNSFilterCreateLayerConvolution |
| BNNSFilterCreateFullyConnectedLayer | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 10.12→11.0; BNNSFilterCreateLayerFullyConnected |
| BNNSFilterCreateFusedLayer | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSFilterCreateLayerActivation | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. Crate reaches it via bnns_relu_f32, bnns_sigmoid_f32. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSFilterCreateLayerArithmetic | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSFilterCreateLayerBroadcastMatMul | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSFilterCreateLayerConvolution | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. Crate reaches it via BnnsFilter::from_convolution, ffi::BNNSFilterCreateLayerConvolution (raw-ffi). | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSFilterCreateLayerDropout | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSFilterCreateLayerEmbedding | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 12.0→15.0; Use BNNSGraph* APIs |
| BNNSFilterCreateLayerFullyConnected | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. Crate reaches it via BnnsFilter::from_fully_connected, ffi::BNNSFilterCreateLayerFullyConnected (raw-ffi). | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSFilterCreateLayerGram | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSFilterCreateLayerLoss | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSFilterCreateLayerMultiheadAttention | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSFilterCreateLayerNormalization | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSFilterCreateLayerPadding | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSFilterCreateLayerPermute | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSFilterCreateLayerPooling | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSFilterCreateLayerReduction | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSFilterCreateLayerResize | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSFilterCreateLayerTensorContraction | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSFilterCreateLayerTransposedConvolution | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSFilterCreatePoolingLayer | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 10.12→11.0; BNNSFilterCreateLayerPooling |
| BNNSFilterCreateVectorActivationLayer | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 10.13→11.0; BNNSFilterCreateLayerActivation |
| BNNSFilterDestroy | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. Crate reaches it via BnnsFilter::drop, bnns_relu_f32, bnns_sigmoid_f32, ffi::BNNSFilterDestroy (raw-ffi). | macos 10.12→15.0; Use BNNSGraph* APIs |
| BNNSFusedFilterApplyBackwardBatch | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSFusedFilterApplyBackwardMultiInputBatch | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 12.0→15.0; Use BNNSGraph* APIs |
| BNNSFusedFilterApplyBatch | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSFusedFilterApplyMultiInputBatch | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 12.0→15.0; Use BNNSGraph* APIs |
| BNNSGather | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 13.0→15.0; Use BNNSGraph* APIs |
| BNNSGatherND | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 13.0→15.0; Use BNNSGraph* APIs |
| BNNSGetPointer | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSLossFilterApplyBackwardBatch | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSLossFilterApplyBatch | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSMatMul | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 13.0→15.0; Use BNNSGraph* APIs |
| BNNSMatMulWorkspaceSize | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 13.0→15.0; Use BNNSGraph* APIs |
| BNNSNDArrayFullyConnectedSparsifySparseCOO | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 13.0→15.0; Use BNNSGraph* APIs |
| BNNSNDArrayFullyConnectedSparsifySparseCSR | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 13.0→15.0; Use BNNSGraph* APIs |
| BNNSNormalizationFilterApplyBackwardBatch | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSNormalizationFilterApplyBatch | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSOptimizerStep | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSPermuteFilterApplyBackwardBatch | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSPoolingFilterApplyBackwardBatch | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSPoolingFilterApplyBackwardBatchEx | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 13.0→15.0; Use BNNSGraph* APIs |
| BNNSPoolingFilterApplyBatch | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSPoolingFilterApplyBatchEx | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 13.0→15.0; Use BNNSGraph* APIs |
| BNNSScatter | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 13.0→15.0; Use BNNSGraph* APIs |
| BNNSScatterND | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 13.0→15.0; Use BNNSGraph* APIs |
| BNNSShuffle | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 13.0→15.0; Use BNNSGraph* APIs |
| BNNSTile | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 13.0→15.0; Use BNNSGraph* APIs |
| BNNSTileBackward | function | vecLib/BNNS/bnns.h | Deprecated on macOS; skipped per audit instructions. | macos 13.0→15.0; Use BNNSGraph* APIs |
| BNNSArithmeticBinary | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSArithmeticTernary | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSArithmeticUnary | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSConvolutionLayerParameters | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 10.12→11.0; BNNSLayerParametersFullyConnected |
| BNNSFullyConnectedLayerParameters | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 10.12→11.0; BNNSLayerParametersPooling |
| BNNSImageStackDescriptor | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 10.12→11.0; BNNS switched to new Layer Parameters data structures |
| BNNSLSTMDataDescriptor | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSLSTMGateDescriptor | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSLayerData | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 10.12→11.0; BNNSLayerParametersConvolution |
| BNNSLayerParametersActivation | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSLayerParametersArithmetic | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSLayerParametersBroadcastMatMul | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSLayerParametersConvolution | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSLayerParametersCropResize | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 13.0→15.0; Use BNNSGraph* APIs |
| BNNSLayerParametersDropout | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSLayerParametersEmbedding | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 12.0→15.0; Use BNNSGraph* APIs |
| BNNSLayerParametersFullyConnected | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSLayerParametersGram | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSLayerParametersLSTM | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSLayerParametersLossBase | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSLayerParametersLossHuber | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSLayerParametersLossSigmoidCrossEntropy | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSLayerParametersLossSoftmaxCrossEntropy | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSLayerParametersLossYolo | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSLayerParametersMultiheadAttention | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSLayerParametersNormalization | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSLayerParametersPadding | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSLayerParametersPermute | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSLayerParametersPooling | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSLayerParametersQuantization | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 12.0→15.0; Use BNNSGraph* APIs |
| BNNSLayerParametersResize | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSLayerParametersTensorContraction | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSOptimizerAdamFields | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSOptimizerAdamWithClippingFields | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSOptimizerRMSPropFields | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSOptimizerRMSPropWithClippingFields | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSOptimizerSGDMomentumFields | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSOptimizerSGDMomentumWithClippingFields | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 11.0→15.0; Use BNNSGraph* APIs |
| BNNSPoolingLayerParameters | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 10.12→11.0; BNNSLayerParametersPooling |
| BNNSVectorDescriptor | typedef | vecLib/BNNS/bnns_structures.h | Deprecated on macOS; skipped per audit instructions. | macos 10.12→11.0; BNNS switched to new Layer Parameters data structures |
| la_difference | function | vecLib/LinearAlgebra/arithmetic.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_elementwise_product | function | vecLib/LinearAlgebra/arithmetic.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_inner_product | function | vecLib/LinearAlgebra/arithmetic.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_matrix_product | function | vecLib/LinearAlgebra/arithmetic.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_outer_product | function | vecLib/LinearAlgebra/arithmetic.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_scale_with_double | function | vecLib/LinearAlgebra/arithmetic.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_scale_with_float | function | vecLib/LinearAlgebra/arithmetic.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_sum | function | vecLib/LinearAlgebra/arithmetic.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_transpose | function | vecLib/LinearAlgebra/arithmetic.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_solve | function | vecLib/LinearAlgebra/linear_systems.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_diagonal_matrix_from_vector | function | vecLib/LinearAlgebra/matrix.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_identity_matrix | function | vecLib/LinearAlgebra/matrix.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_matrix_cols | function | vecLib/LinearAlgebra/matrix.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_matrix_from_double_buffer | function | vecLib/LinearAlgebra/matrix.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_matrix_from_double_buffer_nocopy | function | vecLib/LinearAlgebra/matrix.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_matrix_from_float_buffer | function | vecLib/LinearAlgebra/matrix.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_matrix_from_float_buffer_nocopy | function | vecLib/LinearAlgebra/matrix.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_matrix_rows | function | vecLib/LinearAlgebra/matrix.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_matrix_slice | function | vecLib/LinearAlgebra/matrix.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_matrix_to_double_buffer | function | vecLib/LinearAlgebra/matrix.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_matrix_to_float_buffer | function | vecLib/LinearAlgebra/matrix.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_vector_from_matrix_col | function | vecLib/LinearAlgebra/matrix.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_vector_from_matrix_diagonal | function | vecLib/LinearAlgebra/matrix.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_vector_from_matrix_row | function | vecLib/LinearAlgebra/matrix.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_norm_as_double | function | vecLib/LinearAlgebra/norms.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_norm_as_float | function | vecLib/LinearAlgebra/norms.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_normalized_vector | function | vecLib/LinearAlgebra/norms.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_add_attributes | function | vecLib/LinearAlgebra/object.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_release | function | vecLib/LinearAlgebra/object.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_remove_attributes | function | vecLib/LinearAlgebra/object.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_retain | function | vecLib/LinearAlgebra/object.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_status | function | vecLib/LinearAlgebra/object.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_matrix_from_splat | function | vecLib/LinearAlgebra/splat.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_splat_from_double | function | vecLib/LinearAlgebra/splat.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_splat_from_float | function | vecLib/LinearAlgebra/splat.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_splat_from_matrix_element | function | vecLib/LinearAlgebra/splat.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_splat_from_vector_element | function | vecLib/LinearAlgebra/splat.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_vector_from_splat | function | vecLib/LinearAlgebra/splat.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_vector_length | function | vecLib/LinearAlgebra/vector.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_vector_slice | function | vecLib/LinearAlgebra/vector.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_vector_to_double_buffer | function | vecLib/LinearAlgebra/vector.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| la_vector_to_float_buffer | function | vecLib/LinearAlgebra/vector.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→11.0; This API is deprecated, please use BLAS and LAPACK |
| SetBLASParamErrorProc | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| appleblas_dgeadd | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| appleblas_sgeadd | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.10→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| catlas_caxpby | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| catlas_cset | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| catlas_daxpby | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| catlas_dset | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| catlas_saxpby | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| catlas_sset | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| catlas_zaxpby | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| catlas_zset | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_caxpy | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_ccopy | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_cdotc_sub | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_cdotu_sub | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_cgbmv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_cgemm | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_cgemv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_cgerc | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_cgeru | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_chbmv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_chemm | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_chemv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_cher | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_cher2 | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_cher2k | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_cherk | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_chpmv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_chpr | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_chpr2 | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_crotg | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_cscal | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_csrot | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_csscal | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_cswap | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_csymm | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_csyr2k | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_csyrk | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_ctbmv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_ctbsv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_ctpmv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_ctpsv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_ctrmm | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_ctrmv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_ctrsm | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_ctrsv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_dasum | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_daxpy | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_dcopy | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_ddot | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_dgbmv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_dgemm | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_dgemv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_dger | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_dnrm2 | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_drot | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_drotg | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_drotm | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_drotmg | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_dsbmv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_dscal | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_dsdot | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_dspmv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_dspr | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_dspr2 | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_dswap | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_dsymm | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_dsymv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_dsyr | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_dsyr2 | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_dsyr2k | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_dsyrk | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_dtbmv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_dtbsv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_dtpmv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_dtpsv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_dtrmm | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_dtrmv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_dtrsm | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_dtrsv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_dzasum | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_dznrm2 | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_errprn | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_icamax | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_idamax | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_isamax | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_izamax | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_sasum | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_saxpy | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_scasum | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_scnrm2 | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_scopy | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_sdot | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. Crate reaches it via ffi::cblas_sdot (raw-ffi), sdot. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_sdsdot | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_sgbmv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_sgemm | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. Crate reaches it via ffi::cblas_sgemm (raw-ffi), sgemm_row_major. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_sgemv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. Crate reaches it via ffi::cblas_sgemv (raw-ffi), sgemv_row_major. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_sger | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_snrm2 | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_srot | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_srotg | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_srotm | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_srotmg | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_ssbmv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_sscal | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_sspmv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_sspr | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_sspr2 | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_sswap | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_ssymm | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_ssymv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_ssyr | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_ssyr2 | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_ssyr2k | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_ssyrk | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_stbmv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_stbsv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_stpmv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_stpsv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_strmm | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_strmv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_strsm | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_strsv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_xerbla | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_zaxpy | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_zcopy | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_zdotc_sub | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_zdotu_sub | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_zdrot | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_zdscal | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_zgbmv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_zgemm | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_zgemv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_zgerc | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_zgeru | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_zhbmv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_zhemm | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_zhemv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_zher | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_zher2 | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_zher2k | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_zherk | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_zhpmv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_zhpr | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_zhpr2 | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_zrotg | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_zscal | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_zswap | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_zsymm | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_zsyr2k | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_zsyrk | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_ztbmv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_ztbsv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_ztpmv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_ztpsv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_ztrmm | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_ztrmv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_ztrsm | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cblas_ztrsv | function | vecLib/cblas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated CBLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cbdsqr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgbbrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgbcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgbequ_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgbequb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgbrfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgbsv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgbsvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgbtf2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgbtrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgbtrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgebak_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgebal_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgebd2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgebrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgecon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgeequ_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgeequb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgees_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgeesx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgeev_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgeevx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgegs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgegv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgehd2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgehrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgelq2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgelqf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgels_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgelsd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgelss_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgelsx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgelsy_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgeql2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgeqlf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgeqp3_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgeqpf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgeqr2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgeqrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgerfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgerq2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgerqf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgesc2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgesdd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgesv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgesvd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgesvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgetc2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgetf2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgetrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgetri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgetrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cggbak_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cggbal_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgges_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cggesx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cggev_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cggevx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cggglm_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgghrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgglse_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cggqrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cggrqf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cggsvd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cggsvp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgtcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgtrfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgtsv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgtsvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgttrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgttrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cgtts2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chbev_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chbevd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chbevx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chbgst_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chbgv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chbgvd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chbgvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chbtrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| checon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cheequb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cheev_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cheevd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cheevr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cheevx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chegs2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chegst_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chegv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chegvd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chegvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cherfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chesv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chesvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chetd2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chetf2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chetrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chetrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chetri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chetrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chfrk_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chgeqz_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chla_transtype__ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chpcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chpev_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chpevd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chpevx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chpgst_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chpgv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chpgvd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chpgvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chprfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chpsv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chpsvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chptrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chptrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chptri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chptrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chsein_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| chseqr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clabrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clacgv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clacn2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clacon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clacp2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clacpy_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clacrm_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clacrt_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cladiv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| claed0_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| claed7_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| claed8_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| claein_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| claesy_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| claev2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clag2z_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clags2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clagtm_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clahef_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clahqr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clahr2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clahrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| claic1_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clals0_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clalsa_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clalsd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clangb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clange_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clangt_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clanhb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clanhe_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clanhf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clanhp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clanhs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clanht_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clansb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clansp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clansy_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clantb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clantp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clantr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clapll_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clapmt_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| claqgb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| claqge_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| claqhb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| claqhe_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| claqhp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| claqp2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| claqps_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| claqr0_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| claqr1_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| claqr2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| claqr3_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| claqr4_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| claqr5_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| claqsb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| claqsp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| claqsy_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clar1v_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clar2v_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clarcm_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clarf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clarfb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clarfg_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clarfp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clarft_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clarfx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clargv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clarnv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clarrv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clarscl2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clartg_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clartv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clarz_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clarzb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clarzt_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clascl2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clascl_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| claset_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clasr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| classq_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| claswp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clasyf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clatbs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clatdf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clatps_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clatrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clatrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clatrz_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clatzm_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clauu2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| clauum_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cpbcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cpbequ_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cpbrfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cpbstf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cpbsv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cpbsvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cpbtf2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cpbtrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cpbtrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cpftrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cpftri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cpftrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cpocon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cpoequ_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cpoequb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cporfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cposv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cposvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cpotf2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cpotrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cpotri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cpotrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cppcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cppequ_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cpprfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cppsv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cppsvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cpptrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cpptri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cpptrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cpstf2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cpstrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cptcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cpteqr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cptrfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cptsv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cptsvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cpttrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cpttrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cptts2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| crot_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cspcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cspmv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cspr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| csprfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cspsv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cspsvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| csptrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| csptri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| csptrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| csrscl_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cstedc_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cstegr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cstein_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cstemr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| csteqr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| csycon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| csyequb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| csymv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| csyr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| csyrfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| csysv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| csysvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| csytf2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| csytrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| csytri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| csytrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctbcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctbrfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctbtrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctfsm_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctftri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctfttp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctfttr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctgevc_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctgex2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctgexc_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctgsen_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctgsja_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctgsna_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctgsy2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctgsyl_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctpcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctprfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctptri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctptrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctpttf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctpttr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctrcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctrevc_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctrexc_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctrrfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctrsen_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctrsna_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctrsyl_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctrti2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctrtri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctrtrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctrttf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctrttp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctzrqf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ctzrzf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cung2l_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cung2r_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cungbr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cunghr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cungl2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cunglq_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cungql_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cungqr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cungr2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cungrq_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cungtr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cunm2l_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cunm2r_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cunmbr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cunmhr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cunml2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cunmlq_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cunmql_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cunmqr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cunmr2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cunmr3_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cunmrq_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cunmrz_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cunmtr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cupgtr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| cupmtr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dbdsdc_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dbdsqr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ddisna_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgbbrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgbcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgbequ_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgbequb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgbrfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgbsv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgbsvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgbtf2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgbtrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgbtrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgebak_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgebal_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgebd2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgebrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgecon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgeequ_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgeequb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgees_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgeesx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgeev_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgeevx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgegs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgegv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgehd2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgehrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgejsv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgelq2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgelqf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgels_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgelsd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgelss_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgelsx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgelsy_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgeql2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgeqlf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgeqp3_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgeqpf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgeqr2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgeqrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgerfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgerq2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgerqf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgesc2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgesdd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgesv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgesvd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgesvj_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgesvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgetc2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgetf2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgetrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgetri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgetrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dggbak_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dggbal_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgges_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dggesx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dggev_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dggevx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dggglm_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgghrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgglse_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dggqrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dggrqf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dggsvd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dggsvp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgsvj0_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgsvj1_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgtcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgtrfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgtsv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgtsvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgttrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgttrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dgtts2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dhgeqz_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dhsein_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dhseqr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| disnan_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlabad_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlabrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlacn2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlacon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlacpy_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dladiv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlae2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaebz_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaed0_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaed1_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaed2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaed3_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaed4_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaed5_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaed6_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaed7_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaed8_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaed9_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaeda_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaein_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaev2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaexc_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlag2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlag2s_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlags2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlagtf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlagtm_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlagts_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlagv2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlahqr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlahr2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlahrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaic1_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaisnan_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaln2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlals0_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlalsa_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlalsd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlamc1_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlamc2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlamc3_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlamc4_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlamc5_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlamch_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlamrg_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaneg_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlangb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlange_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlangt_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlanhs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlansb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlansf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlansp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlanst_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlansy_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlantb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlantp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlantr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlanv2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlapll_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlapmt_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlapy2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlapy3_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaqgb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaqge_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaqp2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaqps_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaqr0_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaqr1_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaqr2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaqr3_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaqr4_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaqr5_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaqsb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaqsp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaqsy_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaqtr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlar1v_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlar2v_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlarf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlarfb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlarfg_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlarfp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlarft_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlarfx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlargv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlarnv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlarra_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlarrb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlarrc_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlarrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlarre_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlarrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlarrj_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlarrk_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlarrr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlarrv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlarscl2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlartg_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlartv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaruv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlarz_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlarzb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlarzt_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlas2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlascl2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlascl_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlasd0_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlasd1_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlasd2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlasd3_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlasd4_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlasd5_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlasd6_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlasd7_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlasd8_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlasda_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlasdq_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlasdt_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaset_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlasq1_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlasq2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlasq3_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlasq4_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlasq5_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlasq6_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlasr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlasrt_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlassq_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlasv2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlaswp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlasy2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlasyf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlat2s_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlatbs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlatdf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlatps_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlatrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlatrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlatrz_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlatzm_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlauu2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dlauum_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dopgtr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dopmtr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dorg2l_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dorg2r_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dorgbr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dorghr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dorgl2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dorglq_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dorgql_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dorgqr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dorgr2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dorgrq_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dorgtr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dorm2l_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dorm2r_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dormbr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dormhr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dorml2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dormlq_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dormql_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dormqr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dormr2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dormr3_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dormrq_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dormrz_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dormtr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dpbcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dpbequ_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dpbrfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dpbstf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dpbsv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dpbsvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dpbtf2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dpbtrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dpbtrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dpftrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dpftri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dpftrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dpocon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dpoequ_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dpoequb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dporfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dposv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dposvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dpotf2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dpotrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dpotri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dpotrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dppcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dppequ_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dpprfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dppsv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dppsvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dpptrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dpptri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dpptrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dpstf2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dpstrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dptcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dpteqr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dptrfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dptsv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dptsvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dpttrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dpttrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dptts2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| drscl_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsbev_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsbevd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsbevx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsbgst_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsbgv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsbgvd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsbgvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsbtrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsfrk_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsgesv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dspcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dspev_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dspevd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dspevx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dspgst_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dspgv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dspgvd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dspgvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsposv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsprfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dspsv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dspsvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsptrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsptrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsptri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsptrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dstebz_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dstedc_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dstegr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dstein_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dstemr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsteqr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsterf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dstev_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dstevd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dstevr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dstevx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsycon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsyequb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsyev_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsyevd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsyevr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsyevx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsygs2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsygst_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsygv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsygvd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsygvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsyrfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsysv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsysvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsytd2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsytf2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsytrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsytrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsytri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dsytrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtbcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtbrfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtbtrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtfsm_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtftri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtfttp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtfttr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtgevc_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtgex2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtgexc_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtgsen_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtgsja_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtgsna_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtgsy2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtgsyl_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtpcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtprfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtptri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtptrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtpttf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtpttr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtrcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtrevc_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtrexc_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtrrfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtrsen_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtrsna_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtrsyl_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtrti2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtrtri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtrtrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtrttf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtrttp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtzrqf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dtzrzf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| dzsum1_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| icmax1_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ieeeck_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ilaclc_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ilaclr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| iladiag_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| iladlc_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| iladlr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ilaenv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ilaprec_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ilaslc_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ilaslr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ilatrans_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ilauplo_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ilaver_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ilazlc_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ilazlr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| iparmq_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| izmax1_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| lsamen_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sbdsdc_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sbdsqr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| scsum1_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sdisna_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgbbrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgbcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgbequ_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgbequb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgbrfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgbsv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgbsvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgbtf2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgbtrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgbtrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgebak_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgebal_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgebd2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgebrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgecon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgeequ_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgeequb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgees_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgeesx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgeev_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgeevx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgegs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgegv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgehd2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgehrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgejsv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgelq2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgelqf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgels_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgelsd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgelss_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgelsx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgelsy_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgeql2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgeqlf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgeqp3_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgeqpf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgeqr2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgeqrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgerfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgerq2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgerqf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgesc2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgesdd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgesv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. Crate reaches it via ffi::sgesv_ (raw-ffi), solve_linear_system_f32. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgesvd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgesvj_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgesvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgetc2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgetf2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgetrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. Crate reaches it via ffi::sgetrf_ (raw-ffi), lu_decompose_f32. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgetri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgetrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sggbak_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sggbal_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgges_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sggesx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sggev_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sggevx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sggglm_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgghrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgglse_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sggqrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sggrqf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sggsvd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sggsvp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgsvj0_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgsvj1_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgtcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgtrfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgtsv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgtsvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgttrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgttrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sgtts2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| shgeqz_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| shsein_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| shseqr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sisnan_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slabad_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slabrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slacn2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slacon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slacpy_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sladiv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slae2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaebz_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaed0_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaed1_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaed2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaed3_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaed4_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaed5_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaed6_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaed7_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaed8_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaed9_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaeda_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaein_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaev2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaexc_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slag2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slag2d_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slags2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slagtf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slagtm_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slagts_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slagv2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slahqr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slahr2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slahrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaic1_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaisnan_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaln2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slals0_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slalsa_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slalsd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slamc1_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slamc2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slamc3_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slamc4_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slamc5_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slamch_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slamrg_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaneg_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slangb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slange_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slangt_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slanhs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slansb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slansf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slansp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slanst_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slansy_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slantb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slantp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slantr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slanv2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slapll_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slapmt_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slapy2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slapy3_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaqgb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaqge_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaqp2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaqps_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaqr0_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaqr1_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaqr2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaqr3_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaqr4_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaqr5_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaqsb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaqsp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaqsy_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaqtr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slar1v_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slar2v_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slarf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slarfb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slarfg_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slarfp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slarft_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slarfx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slargv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slarnv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slarra_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slarrb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slarrc_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slarrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slarre_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slarrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slarrj_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slarrk_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slarrr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slarrv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slarscl2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slartg_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slartv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaruv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slarz_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slarzb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slarzt_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slas2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slascl2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slascl_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slasd0_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slasd1_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slasd2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slasd3_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slasd4_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slasd5_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slasd6_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slasd7_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slasd8_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slasda_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slasdq_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slasdt_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaset_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slasq1_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slasq2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slasq3_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slasq4_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slasq5_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slasq6_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slasr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slasrt_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slassq_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slasv2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slaswp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slasy2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slasyf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slatbs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slatdf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slatps_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slatrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slatrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slatrz_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slatzm_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slauu2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| slauum_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| smaxloc_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sopgtr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sopmtr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sorg2l_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sorg2r_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sorgbr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sorghr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sorgl2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sorglq_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sorgql_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sorgqr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sorgr2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sorgrq_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sorgtr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sorm2l_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sorm2r_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sormbr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sormhr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sorml2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sormlq_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sormql_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sormqr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sormr2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sormr3_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sormrq_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sormrz_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sormtr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| spbcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| spbequ_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| spbrfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| spbstf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| spbsv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| spbsvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| spbtf2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| spbtrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| spbtrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| spftrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| spftri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| spftrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| spocon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| spoequ_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| spoequb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sporfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sposv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sposvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| spotf2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| spotrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| spotri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| spotrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sppcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sppequ_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| spprfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sppsv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sppsvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| spptrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| spptri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| spptrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| spstf2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| spstrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sptcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| spteqr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sptrfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sptsv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sptsvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| spttrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| spttrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sptts2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| srscl_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssbev_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssbevd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssbevx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssbgst_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssbgv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssbgvd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssbgvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssbtrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssfrk_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sspcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sspev_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sspevd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sspevx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sspgst_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sspgv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sspgvd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sspgvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssprfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sspsv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sspsvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssptrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssptrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssptri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssptrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sstebz_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sstedc_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sstegr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sstein_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sstemr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssteqr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssterf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sstev_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sstevd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sstevr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| sstevx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssycon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssyequb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssyev_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssyevd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssyevr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssyevx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssygs2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssygst_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssygv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssygvd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssygvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssyrfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssysv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssysvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssytd2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssytf2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssytrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssytrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssytri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ssytrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| stbcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| stbrfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| stbtrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| stfsm_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| stftri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| stfttp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| stfttr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| stgevc_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| stgex2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| stgexc_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| stgsen_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| stgsja_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| stgsna_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| stgsy2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| stgsyl_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| stpcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| stprfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| stptri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| stptrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| stpttf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| stpttr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| strcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| strevc_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| strexc_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| strrfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| strsen_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| strsna_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| strsyl_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| strti2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| strtri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| strtrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| strttf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| strttp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| stzrqf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| stzrzf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zbdsqr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zcgesv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zcposv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zdrscl_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgbbrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgbcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgbequ_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgbequb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgbrfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgbsv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgbsvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgbtf2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgbtrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgbtrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgebak_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgebal_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgebd2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgebrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgecon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgeequ_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgeequb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgees_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgeesx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgeev_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgeevx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgegs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgegv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgehd2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgehrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgelq2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgelqf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgels_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgelsd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgelss_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgelsx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgelsy_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgeql2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgeqlf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgeqp3_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgeqpf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgeqr2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgeqrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgerfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgerq2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgerqf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgesc2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgesdd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgesv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgesvd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgesvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgetc2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgetf2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgetrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgetri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgetrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zggbak_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zggbal_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgges_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zggesx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zggev_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zggevx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zggglm_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgghrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgglse_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zggqrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zggrqf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zggsvd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zggsvp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgtcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgtrfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgtsv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgtsvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgttrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgttrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zgtts2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhbev_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhbevd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhbevx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhbgst_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhbgv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhbgvd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhbgvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhbtrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhecon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zheequb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zheev_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zheevd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zheevr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zheevx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhegs2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhegst_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhegv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhegvd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhegvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zherfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhesv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhesvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhetd2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhetf2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhetrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhetrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhetri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhetrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhfrk_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhgeqz_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhpcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhpev_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhpevd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhpevx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhpgst_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhpgv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhpgvd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhpgvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhprfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhpsv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhpsvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhptrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhptrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhptri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhptrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhsein_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zhseqr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlabrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlacgv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlacn2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlacon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlacp2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlacpy_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlacrm_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlacrt_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zladiv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlaed0_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlaed7_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlaed8_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlaein_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlaesy_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlaev2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlag2c_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlags2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlagtm_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlahef_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlahqr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlahr2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlahrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlaic1_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlals0_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlalsa_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlalsd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlangb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlange_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlangt_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlanhb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlanhe_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlanhf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlanhp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlanhs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlanht_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlansb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlansp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlansy_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlantb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlantp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlantr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlapll_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlapmt_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlaqgb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlaqge_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlaqhb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlaqhe_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlaqhp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlaqp2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlaqps_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlaqr0_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlaqr1_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlaqr2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlaqr3_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlaqr4_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlaqr5_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlaqsb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlaqsp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlaqsy_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlar1v_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlar2v_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlarcm_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlarf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlarfb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlarfg_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlarfp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlarft_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlarfx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlargv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlarnv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlarrv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlarscl2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlartg_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlartv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlarz_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlarzb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlarzt_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlascl2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlascl_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlaset_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlasr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlassq_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlaswp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlasyf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlat2c_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlatbs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlatdf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlatps_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlatrd_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlatrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlatrz_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlatzm_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlauu2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zlauum_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zpbcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zpbequ_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zpbrfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zpbstf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zpbsv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zpbsvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zpbtf2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zpbtrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zpbtrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zpftrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zpftri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zpftrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zpocon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zpoequ_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zpoequb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zporfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zposv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zposvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zpotf2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zpotrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zpotri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zpotrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zppcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zppequ_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zpprfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zppsv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zppsvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zpptrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zpptri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zpptrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zpstf2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zpstrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zptcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zpteqr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zptrfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zptsv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zptsvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zpttrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zpttrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zptts2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zrot_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zspcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zspmv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zspr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zsprfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zspsv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zspsvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zsptrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zsptri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zsptrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zstedc_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zstegr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zstein_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zstemr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zsteqr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zsycon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zsyequb_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zsymv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zsyr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zsyrfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zsysv_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zsysvx_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zsytf2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zsytrf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zsytri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zsytrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztbcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztbrfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztbtrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztfsm_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztftri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztfttp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztfttr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztgevc_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztgex2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztgexc_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztgsen_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztgsja_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztgsna_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztgsy2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztgsyl_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztpcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztprfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztptri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztptrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztpttf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztpttr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztrcon_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztrevc_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztrexc_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztrrfs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztrsen_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztrsna_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztrsyl_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztrti2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztrtri_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztrtrs_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztrttf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztrttp_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztzrqf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| ztzrzf_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zung2l_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zung2r_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zungbr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zunghr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zungl2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zunglq_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zungql_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zungqr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zungr2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zungrq_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zungtr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zunm2l_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zunm2r_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zunmbr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zunmhr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zunml2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zunmlq_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zunmql_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zunmqr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zunmr2_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zunmr3_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zunmrq_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zunmrz_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zunmtr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zupgtr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| zupmtr_ | function | vecLib/clapack.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; The CLAPACK interface is deprecated.  Please compile with -DACCELERATE_NEW_LAPACK to access the new lapack headers. |
| caxpy_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| ccopy_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cdotc_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cdotu_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cgbmv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cgemm_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cgemv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cgerc_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cgeru_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| chbmv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| chemm_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| chemv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cher2_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cher2k_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cher_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cherk_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| chpmv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| chpr2_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| chpr_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| crotg_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cscal_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| csrot_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| csscal_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| cswap_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| csymm_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| csyr2k_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| csyrk_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| ctbmv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| ctbsv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| ctpmv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| ctpsv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| ctrmm_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| ctrmv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| ctrsm_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| ctrsv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| dasum_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| daxpy_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| dcopy_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| ddot_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| dgbmv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| dgemm_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| dgemv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| dger_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| dnrm2_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| drot_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| drotg_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| drotm_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| drotmg_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| dsbmv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| dscal_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| dsdot_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| dspmv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| dspr2_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| dspr_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| dswap_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| dsymm_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| dsymv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| dsyr2_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| dsyr2k_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| dsyr_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| dsyrk_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| dtbmv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| dtbsv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| dtpmv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| dtpsv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| dtrmm_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| dtrmv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| dtrsm_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| dtrsv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| dzasum_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| dznrm2_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| icamax_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| idamax_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| isamax_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| izamax_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| sasum_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| saxpy_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| scasum_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| scnrm2_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| scopy_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| sdot_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| sdsdot_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| sgbmv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| sgemm_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| sgemv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| sger_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| snrm2_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| srot_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| srotg_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| srotm_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| srotmg_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| ssbmv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| sscal_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| sspmv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| sspr2_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| sspr_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| sswap_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| ssymm_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| ssymv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| ssyr2_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| ssyr2k_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| ssyr_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| ssyrk_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| stbmv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| stbsv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| stpmv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| stpsv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| strmm_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| strmv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| strsm_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| strsv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| xerbla_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| zaxpy_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| zcopy_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| zdotc_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| zdotu_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| zdrot_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| zdscal_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| zgbmv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| zgemm_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| zgemv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| zgerc_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| zgeru_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| zhbmv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| zhemm_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| zhemv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| zher2_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| zher2k_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| zher_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| zherk_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| zhpmv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| zhpr2_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| zhpr_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| zrotg_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| zscal_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| zswap_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| zsymm_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| zsyr2k_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| zsyrk_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| ztbmv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| ztbsv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| ztpmv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| ztpsv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| ztrmm_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| ztrmv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| ztrsm_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| ztrsv_ | function | vecLib/fortran_blas.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→13.3; An updated BLAS interface supporting ILP64 is available.  Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support. |
| vDSP_fft3_zop | function | vecLib/vDSP.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→10.11; use DFT routines |
| vDSP_fft3_zopD | function | vecLib/vDSP.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→10.11; use DFT routines |
| vDSP_fft5_zop | function | vecLib/vDSP.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→10.11; use DFT routines |
| vDSP_fft5_zopD | function | vecLib/vDSP.h | Deprecated on macOS; skipped per audit instructions. | macos 10.2→10.11; use DFT routines |
| vIsamax | function | vecLib/vectorOps.h | Deprecated on macOS; skipped per audit instructions. | macos 10.0→10.14; Use cblas_isamax or vDSP_maxmgvi instead |
| vIsamin | function | vecLib/vectorOps.h | Deprecated on macOS; skipped per audit instructions. | macos 10.0→10.14; Use vDSP_minmgvi instead |
| vIsmax | function | vecLib/vectorOps.h | Deprecated on macOS; skipped per audit instructions. | macos 10.0→10.14; Use vDSP_maxvi instead |
| vIsmin | function | vecLib/vectorOps.h | Deprecated on macOS; skipped per audit instructions. | macos 10.0→10.14; Use vDSP_minvi instead |
| vSasum | function | vecLib/vectorOps.h | Deprecated on macOS; skipped per audit instructions. | macos 10.0→10.14; Use cblas_sasum or vDSP_svemg instead |
| vSaxpy | function | vecLib/vectorOps.h | Deprecated on macOS; skipped per audit instructions. | macos 10.0→10.14; Use cblas_saxpy instead |
| vScopy | function | vecLib/vectorOps.h | Deprecated on macOS; skipped per audit instructions. | macos 10.0→10.14; Use cblas_scopy or memcpy instead |
| vSdot | function | vecLib/vectorOps.h | Deprecated on macOS; skipped per audit instructions. | macos 10.0→10.14; Use cblas_sdot or vDSP_dotpr instead |
| vSgeadd | function | vecLib/vectorOps.h | Deprecated on macOS; skipped per audit instructions. | macos 10.0→10.14; Use appleblas_sgeadd instead |
| vSgemm | function | vecLib/vectorOps.h | Deprecated on macOS; skipped per audit instructions. | macos 10.0→10.14; Use cblas_sgemm instead |
| vSgemtx | function | vecLib/vectorOps.h | Deprecated on macOS; skipped per audit instructions. | macos 10.0→10.14; Use cblas_sgemv instead |
| vSgemul | function | vecLib/vectorOps.h | Deprecated on macOS; skipped per audit instructions. | macos 10.0→10.14; Use cblas_sgemm instead |
| vSgemv | function | vecLib/vectorOps.h | Deprecated on macOS; skipped per audit instructions. | macos 10.0→10.14; Use cblas_sgemv instead |
| vSgemx | function | vecLib/vectorOps.h | Deprecated on macOS; skipped per audit instructions. | macos 10.0→10.14; Use cblas_sgemv instead |
| vSgesub | function | vecLib/vectorOps.h | Deprecated on macOS; skipped per audit instructions. | macos 10.0→10.14; Use appleblas_sgeadd instead |
| vSgetmi | function | vecLib/vectorOps.h | Deprecated on macOS; skipped per audit instructions. | macos 10.0→10.14; Use appleblas_sgeadd instead |
| vSgetmo | function | vecLib/vectorOps.h | Deprecated on macOS; skipped per audit instructions. | macos 10.0→10.14; Use appleblas_sgeadd instead |
| vSgevv | function | vecLib/vectorOps.h | Deprecated on macOS; skipped per audit instructions. | macos 10.0→10.14; Use cblas_sger on a zero matrix instead |
| vSnaxpy | function | vecLib/vectorOps.h | Deprecated on macOS; skipped per audit instructions. | macos 10.0→10.14; Use cblas_saxpy in a loop instead |
| vSndot | function | vecLib/vectorOps.h | Deprecated on macOS; skipped per audit instructions. | macos 10.0→10.14; Use cblas_sdot or vDSP_dotpr in a loop instead |
| vSnorm2 | function | vecLib/vectorOps.h | Deprecated on macOS; skipped per audit instructions. | macos 10.0→10.14; Use cblas_snrm2 instead |
| vSnrm2 | function | vecLib/vectorOps.h | Deprecated on macOS; skipped per audit instructions. | macos 10.0→10.14; Use cblas_snrm2 instead |
| vSrot | function | vecLib/vectorOps.h | Deprecated on macOS; skipped per audit instructions. | macos 10.0→10.14; Use cblas_srot instead |
| vSscal | function | vecLib/vectorOps.h | Deprecated on macOS; skipped per audit instructions. | macos 10.0→10.14; Use cblas_sscal instead |
| vSsum | function | vecLib/vectorOps.h | Deprecated on macOS; skipped per audit instructions. | macos 10.0→10.14; Use vDSP_sve instead |
| vSswap | function | vecLib/vectorOps.h | Deprecated on macOS; skipped per audit instructions. | macos 10.0→10.14; Use cblas_sswap instead |
| vSyax | function | vecLib/vectorOps.h | Deprecated on macOS; skipped per audit instructions. | macos 10.0→10.14; Use vDSP_vsmul instead |
| vSzaxpy | function | vecLib/vectorOps.h | Deprecated on macOS; skipped per audit instructions. | macos 10.0→10.14; Use vDSP_vsma instead |
| vfabf | function | vecLib/vfp.h | Deprecated on macOS; skipped per audit instructions. | macos 10.0→12.0; vfabsf |
| vintf | function | vecLib/vfp.h | Deprecated on macOS; skipped per audit instructions. | macos 10.5→10.14; vtruncf |
