use core::ffi::c_void;

/// Bridge callback type for `quadrature_integrate`.
pub type QuadratureCallback =
    Option<unsafe extern "C" fn(*mut c_void, usize, *const f64, *mut f64)>;

unsafe extern "C" {
    /// Releases an opaque Swift bridge handle owned by a safe wrapper.
    pub fn acc_release_handle(handle: *mut c_void);

    /// Swift bridge wrapper for `vDSP_create_fftsetup`.
    pub fn acc_vdsp_fft_setup_create(log2n: usize, radix: i32) -> *mut c_void;
    /// Swift bridge wrapper for `vDSP_fft_zip`.
    pub fn acc_vdsp_fft_setup_apply(
        handle: *mut c_void,
        real: *mut f32,
        imag: *mut f32,
        log2n: usize,
        direction: i32,
    ) -> bool;
    /// Swift bridge wrapper for `vDSP_biquad_CreateSetup`.
    pub fn acc_vdsp_biquad_setup_create(coefficients: *const f64, count: usize) -> *mut c_void;
    /// Swift bridge wrapper for `vDSP_biquad`.
    pub fn acc_vdsp_biquad_setup_apply(
        handle: *mut c_void,
        delay: *mut f32,
        input: *const f32,
        output: *mut f32,
        length: usize,
    ) -> bool;
    /// Swift bridge wrapper for `vDSP_vadd`.
    pub fn acc_vdsp_add_f32(a: *const f32, b: *const f32, output: *mut f32, length: usize) -> bool;
    /// Swift bridge wrapper for `vDSP_vaddD`.
    pub fn acc_vdsp_add_f64(a: *const f64, b: *const f64, output: *mut f64, length: usize) -> bool;
    /// Swift bridge wrapper for `vDSP_vsub`.
    pub fn acc_vdsp_sub_f32(a: *const f32, b: *const f32, output: *mut f32, length: usize) -> bool;
    /// Swift bridge wrapper for `vDSP_vsubD`.
    pub fn acc_vdsp_sub_f64(a: *const f64, b: *const f64, output: *mut f64, length: usize) -> bool;
    /// Swift bridge wrapper for `vDSP_dotpr`.
    pub fn acc_vdsp_dot_f32(a: *const f32, b: *const f32, output: *mut f32, length: usize) -> bool;
    /// Swift bridge wrapper for `vDSP_dotprD`.
    pub fn acc_vdsp_dot_f64(a: *const f64, b: *const f64, output: *mut f64, length: usize) -> bool;
    /// Swift bridge wrapper for `vDSP_maxv`.
    pub fn acc_vdsp_max_f32(input: *const f32, output: *mut f32, length: usize) -> bool;
    /// Swift bridge wrapper for `vDSP_maxvD`.
    pub fn acc_vdsp_max_f64(input: *const f64, output: *mut f64, length: usize) -> bool;
    /// Swift bridge wrapper for `vDSP_minv`.
    pub fn acc_vdsp_min_f32(input: *const f32, output: *mut f32, length: usize) -> bool;
    /// Swift bridge wrapper for `vDSP_minvD`.
    pub fn acc_vdsp_min_f64(input: *const f64, output: *mut f64, length: usize) -> bool;
    /// Swift bridge wrapper for `vDSP_meanv`.
    pub fn acc_vdsp_mean_f32(input: *const f32, output: *mut f32, length: usize) -> bool;
    /// Swift bridge wrapper for `vDSP_meanvD`.
    pub fn acc_vdsp_mean_f64(input: *const f64, output: *mut f64, length: usize) -> bool;
    /// Swift bridge wrapper for `vDSP_sve`.
    pub fn acc_vdsp_sum_f32(input: *const f32, output: *mut f32, length: usize) -> bool;
    /// Swift bridge wrapper for `vDSP_sveD`.
    pub fn acc_vdsp_sum_f64(input: *const f64, output: *mut f64, length: usize) -> bool;
    /// Swift bridge wrapper for `vDSP_hamm_window`.
    pub fn acc_vdsp_hamming_window(output: *mut f32, length: usize, flags: i32) -> bool;
    /// Swift bridge wrapper for `vDSP_hamm_windowD`.
    pub fn acc_vdsp_hamming_window_f64(output: *mut f64, length: usize, flags: i32) -> bool;
    /// Swift bridge wrapper for `vDSP_blkman_window`.
    pub fn acc_vdsp_blackman_window(output: *mut f32, length: usize, flags: i32) -> bool;
    /// Swift bridge wrapper for `vDSP_blkman_windowD`.
    pub fn acc_vdsp_blackman_window_f64(output: *mut f64, length: usize, flags: i32) -> bool;

    /// Swift bridge wrapper for `vvsinf`.
    pub fn acc_vforce_sin_f32(input: *const f32, output: *mut f32, length: usize) -> bool;
    /// Swift bridge wrapper for `vvcosf`.
    pub fn acc_vforce_cos_f32(input: *const f32, output: *mut f32, length: usize) -> bool;
    /// Swift bridge wrapper for `vvexpf`.
    pub fn acc_vforce_exp_f32(input: *const f32, output: *mut f32, length: usize) -> bool;
    /// Swift bridge wrapper for `vvlogf`.
    pub fn acc_vforce_log_f32(input: *const f32, output: *mut f32, length: usize) -> bool;
    /// Swift bridge wrapper for `vvsqrtf`.
    pub fn acc_vforce_sqrt_f32(input: *const f32, output: *mut f32, length: usize) -> bool;

    /// Swift bridge wrapper for `cblas_sdot`.
    pub fn acc_blas_sdot(length: i32, x: *const f32, y: *const f32) -> f32;
    /// Swift bridge wrapper for `cblas_sgemv`.
    pub fn acc_blas_sgemv_row_major(
        rows: i32,
        columns: i32,
        alpha: f32,
        matrix: *const f32,
        x: *const f32,
        beta: f32,
        y: *mut f32,
    ) -> bool;
    /// Swift bridge wrapper for `cblas_sgemm`.
    pub fn acc_blas_sgemm_row_major(
        rows: i32,
        columns: i32,
        inner_dimension: i32,
        alpha: f32,
        lhs: *const f32,
        rhs: *const f32,
        beta: f32,
        output: *mut f32,
    ) -> bool;

    /// Swift bridge wrapper for `sgetrf_`.
    pub fn acc_lapack_sgetrf(matrix: *mut f32, dimension: i32, pivots: *mut i32) -> i32;
    /// Swift bridge wrapper for `sgesv_`.
    pub fn acc_lapack_sgesv(
        matrix: *mut f32,
        dimension: i32,
        rhs: *mut f32,
        rhs_count: i32,
        pivots: *mut i32,
    ) -> i32;

    /// Swift bridge wrapper for BNNS `ReLU` activation.
    pub fn acc_bnns_relu_f32(input: *const f32, output: *mut f32, length: usize) -> i32;
    /// Swift bridge wrapper for BNNS sigmoid activation.
    pub fn acc_bnns_sigmoid_f32(input: *const f32, output: *mut f32, length: usize) -> i32;
    /// Swift bridge constructor for BNNS graph compile options.
    pub fn acc_bnns_graph_compile_options_create() -> *mut c_void;
    /// Swift bridge setter for the BNNS graph single-thread option.
    pub fn acc_bnns_graph_compile_options_set_target_single_thread(
        handle: *mut c_void,
        value: bool,
    ) -> bool;
    /// Swift bridge getter for the BNNS graph single-thread option.
    pub fn acc_bnns_graph_compile_options_get_target_single_thread(handle: *mut c_void) -> bool;
    /// Swift bridge setter for the BNNS graph debug-info option.
    pub fn acc_bnns_graph_compile_options_set_generate_debug_info(
        handle: *mut c_void,
        value: bool,
    ) -> bool;
    /// Swift bridge getter for the BNNS graph debug-info option.
    pub fn acc_bnns_graph_compile_options_get_generate_debug_info(handle: *mut c_void) -> bool;
    /// Swift bridge setter for the BNNS graph optimization preference.
    pub fn acc_bnns_graph_compile_options_set_optimization_preference(
        handle: *mut c_void,
        preference: u32,
    ) -> bool;
    /// Swift bridge getter for the BNNS graph optimization preference.
    pub fn acc_bnns_graph_compile_options_get_optimization_preference(handle: *mut c_void) -> u32;

    /// Swift bridge wrapper for `sparse_inner_product_dense_float`.
    pub fn acc_sparse_dot_dense_f32(
        nz: u64,
        values: *const f32,
        indices: *const i64,
        dense: *const f32,
    ) -> f32;
    /// Swift bridge wrapper for `sparse_inner_product_sparse_float`.
    pub fn acc_sparse_dot_sparse_f32(
        lhs_count: u64,
        lhs_values: *const f32,
        lhs_indices: *const i64,
        rhs_count: u64,
        rhs_values: *const f32,
        rhs_indices: *const i64,
    ) -> f32;
    /// Swift bridge wrapper for `sparse_vector_add_with_scale_dense_float`.
    pub fn acc_sparse_add_to_dense_f32(
        nz: u64,
        alpha: f32,
        values: *const f32,
        indices: *const i64,
        dense: *mut f32,
    ) -> bool;
    /// Swift bridge wrapper for `sparse_matrix_create_float`.
    pub fn acc_sparse_matrix_f32_create(rows: u64, columns: u64) -> *mut c_void;
    /// Swift bridge wrapper for `sparse_set_matrix_property`.
    pub fn acc_sparse_matrix_f32_set_property(handle: *mut c_void, property: i32) -> i32;
    /// Swift bridge wrapper for `sparse_insert_entry_float`.
    pub fn acc_sparse_matrix_f32_insert_entry(
        handle: *mut c_void,
        value: f32,
        row: i64,
        column: i64,
    ) -> i32;
    /// Swift bridge wrapper for `sparse_commit`.
    pub fn acc_sparse_matrix_f32_commit(handle: *mut c_void) -> i32;
    /// Swift bridge wrapper for `sparse_get_matrix_number_of_rows`.
    pub fn acc_sparse_matrix_f32_rows(handle: *mut c_void) -> u64;
    /// Swift bridge wrapper for `sparse_get_matrix_number_of_columns`.
    pub fn acc_sparse_matrix_f32_columns(handle: *mut c_void) -> u64;
    /// Swift bridge wrapper for `sparse_get_matrix_nonzero_count`.
    pub fn acc_sparse_matrix_f32_nonzero_count(handle: *mut c_void) -> i64;
    /// Swift bridge wrapper for `sparse_vector_triangular_solve_dense_float`.
    pub fn acc_sparse_matrix_f32_triangular_solve_vector(
        handle: *mut c_void,
        transpose: i32,
        alpha: f32,
        values: *mut f32,
        length: u64,
    ) -> i32;
    /// Swift bridge wrapper for `sparse_matrix_triangular_solve_dense_float`.
    pub fn acc_sparse_matrix_f32_triangular_solve_matrix(
        handle: *mut c_void,
        order: i32,
        transpose: i32,
        rhs_count: u64,
        alpha: f32,
        values: *mut f32,
        ldb: u64,
    ) -> i32;

    /// Swift bridge wrapper for `vImageRotate_ARGB8888`.
    pub fn acc_vimage_rotate_argb8888(
        src_data: *mut c_void,
        src_width: usize,
        src_height: usize,
        src_row_bytes: usize,
        dst_data: *mut c_void,
        dst_width: usize,
        dst_height: usize,
        dst_row_bytes: usize,
        angle_radians: f32,
        background_color: *const u8,
        flags: u32,
    ) -> isize;
    /// Swift bridge wrapper for `vImageBoxConvolve_ARGB8888`.
    pub fn acc_vimage_box_convolve_argb8888(
        src_data: *mut c_void,
        src_width: usize,
        src_height: usize,
        src_row_bytes: usize,
        dst_data: *mut c_void,
        dst_width: usize,
        dst_height: usize,
        dst_row_bytes: usize,
        kernel_height: u32,
        kernel_width: u32,
        background_color: *const u8,
        flags: u32,
    ) -> isize;
    /// Swift bridge wrapper for `vImageScale_ARGB8888`.
    pub fn acc_vimage_scale_argb8888(
        src_data: *mut c_void,
        src_width: usize,
        src_height: usize,
        src_row_bytes: usize,
        dst_data: *mut c_void,
        dst_width: usize,
        dst_height: usize,
        dst_row_bytes: usize,
        flags: u32,
    ) -> isize;
    /// Swift bridge wrapper for `vImageContrastStretch_Planar8`.
    pub fn acc_vimage_contrast_stretch_planar8(
        src_data: *mut c_void,
        src_width: usize,
        src_height: usize,
        src_row_bytes: usize,
        dst_data: *mut c_void,
        dst_width: usize,
        dst_height: usize,
        dst_row_bytes: usize,
        flags: u32,
    ) -> isize;
    /// Swift bridge wrapper for `vImageAlphaBlend_ARGB8888`.
    pub fn acc_vimage_alpha_blend_argb8888(
        src_top_data: *mut c_void,
        src_top_width: usize,
        src_top_height: usize,
        src_top_row_bytes: usize,
        src_bottom_data: *mut c_void,
        src_bottom_width: usize,
        src_bottom_height: usize,
        src_bottom_row_bytes: usize,
        dst_data: *mut c_void,
        dst_width: usize,
        dst_height: usize,
        dst_row_bytes: usize,
        flags: u32,
    ) -> isize;
    /// Swift bridge wrapper for `vImageClipToAlpha_ARGB8888`.
    pub fn acc_vimage_clip_to_alpha_argb8888(
        src_data: *mut c_void,
        src_width: usize,
        src_height: usize,
        src_row_bytes: usize,
        dst_data: *mut c_void,
        dst_width: usize,
        dst_height: usize,
        dst_row_bytes: usize,
        flags: u32,
    ) -> isize;
    /// Swift bridge wrapper for `vImagePremultiplyData_ARGB8888`.
    pub fn acc_vimage_premultiply_argb8888(
        src_data: *mut c_void,
        src_width: usize,
        src_height: usize,
        src_row_bytes: usize,
        dst_data: *mut c_void,
        dst_width: usize,
        dst_height: usize,
        dst_row_bytes: usize,
        flags: u32,
    ) -> isize;
    /// Swift bridge wrapper for `vImageUnpremultiplyData_ARGB8888`.
    pub fn acc_vimage_unpremultiply_argb8888(
        src_data: *mut c_void,
        src_width: usize,
        src_height: usize,
        src_row_bytes: usize,
        dst_data: *mut c_void,
        dst_width: usize,
        dst_height: usize,
        dst_row_bytes: usize,
        flags: u32,
    ) -> isize;
    /// Swift bridge wrapper for `vImageConvert_Planar8toARGB8888`.
    pub fn acc_vimage_convert_planar8_to_argb8888(
        src_alpha_data: *mut c_void,
        src_alpha_width: usize,
        src_alpha_height: usize,
        src_alpha_row_bytes: usize,
        src_red_data: *mut c_void,
        src_red_width: usize,
        src_red_height: usize,
        src_red_row_bytes: usize,
        src_green_data: *mut c_void,
        src_green_width: usize,
        src_green_height: usize,
        src_green_row_bytes: usize,
        src_blue_data: *mut c_void,
        src_blue_width: usize,
        src_blue_height: usize,
        src_blue_row_bytes: usize,
        dst_data: *mut c_void,
        dst_width: usize,
        dst_height: usize,
        dst_row_bytes: usize,
        flags: u32,
    ) -> isize;
    /// Swift bridge wrapper for `vImageConvert_ARGB8888toPlanar8`.
    pub fn acc_vimage_convert_argb8888_to_planar8(
        src_data: *mut c_void,
        src_width: usize,
        src_height: usize,
        src_row_bytes: usize,
        dst_alpha_data: *mut c_void,
        dst_alpha_width: usize,
        dst_alpha_height: usize,
        dst_alpha_row_bytes: usize,
        dst_red_data: *mut c_void,
        dst_red_width: usize,
        dst_red_height: usize,
        dst_red_row_bytes: usize,
        dst_green_data: *mut c_void,
        dst_green_width: usize,
        dst_green_height: usize,
        dst_green_row_bytes: usize,
        dst_blue_data: *mut c_void,
        dst_blue_width: usize,
        dst_blue_height: usize,
        dst_blue_row_bytes: usize,
        flags: u32,
    ) -> isize;

    /// Swift bridge wrapper for four-lane `simd_float4` addition.
    pub fn acc_simd_add_f32x4(lhs: *const f32, rhs: *const f32, output: *mut f32) -> bool;
    /// Swift bridge wrapper for four-lane `simd_float4` dot-product evaluation.
    pub fn acc_simd_dot_f32x4(lhs: *const f32, rhs: *const f32, output: *mut f32) -> bool;
    /// Swift bridge wrapper for four-lane `simd_float4` length computation.
    pub fn acc_simd_length_f32x4(input: *const f32, output: *mut f32) -> bool;
    /// Swift bridge wrapper for four-lane `simd_float4` normalization.
    pub fn acc_simd_normalize_f32x4(input: *const f32, output: *mut f32) -> bool;

    /// Swift bridge wrapper for `quadrature_integrate`.
    pub fn acc_quadrature_integrate(
        callback: QuadratureCallback,
        context: *mut c_void,
        a: f64,
        b: f64,
        integrator: i32,
        abs_tolerance: f64,
        rel_tolerance: f64,
        qag_points_per_interval: usize,
        max_intervals: usize,
        status_out: *mut i32,
        abs_error_out: *mut f64,
    ) -> f64;
}
