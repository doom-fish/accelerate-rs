use core::ffi::c_void;

pub type QuadratureCallback =
    Option<unsafe extern "C" fn(*mut c_void, usize, *const f64, *mut f64)>;

unsafe extern "C" {
    pub fn acc_release_handle(handle: *mut c_void);

    pub fn acc_vdsp_fft_setup_create(log2n: usize, radix: i32) -> *mut c_void;
    pub fn acc_vdsp_fft_setup_apply(
        handle: *mut c_void,
        real: *mut f32,
        imag: *mut f32,
        log2n: usize,
        direction: i32,
    ) -> bool;
    pub fn acc_vdsp_biquad_setup_create(coefficients: *const f64, count: usize) -> *mut c_void;
    pub fn acc_vdsp_biquad_setup_apply(
        handle: *mut c_void,
        delay: *mut f32,
        input: *const f32,
        output: *mut f32,
        length: usize,
    ) -> bool;
    pub fn acc_vdsp_add_f32(a: *const f32, b: *const f32, output: *mut f32, length: usize) -> bool;
    pub fn acc_vdsp_sub_f32(a: *const f32, b: *const f32, output: *mut f32, length: usize) -> bool;
    pub fn acc_vdsp_dot_f32(a: *const f32, b: *const f32, output: *mut f32, length: usize) -> bool;
    pub fn acc_vdsp_max_f32(input: *const f32, output: *mut f32, length: usize) -> bool;
    pub fn acc_vdsp_min_f32(input: *const f32, output: *mut f32, length: usize) -> bool;
    pub fn acc_vdsp_mean_f32(input: *const f32, output: *mut f32, length: usize) -> bool;
    pub fn acc_vdsp_sum_f32(input: *const f32, output: *mut f32, length: usize) -> bool;
    pub fn acc_vdsp_hamming_window(output: *mut f32, length: usize, flags: i32) -> bool;
    pub fn acc_vdsp_blackman_window(output: *mut f32, length: usize, flags: i32) -> bool;

    pub fn acc_vforce_sin_f32(input: *const f32, output: *mut f32, length: usize) -> bool;
    pub fn acc_vforce_cos_f32(input: *const f32, output: *mut f32, length: usize) -> bool;
    pub fn acc_vforce_exp_f32(input: *const f32, output: *mut f32, length: usize) -> bool;
    pub fn acc_vforce_log_f32(input: *const f32, output: *mut f32, length: usize) -> bool;
    pub fn acc_vforce_sqrt_f32(input: *const f32, output: *mut f32, length: usize) -> bool;

    pub fn acc_blas_sdot(length: i32, x: *const f32, y: *const f32) -> f32;
    pub fn acc_blas_sgemv_row_major(
        rows: i32,
        columns: i32,
        alpha: f32,
        matrix: *const f32,
        x: *const f32,
        beta: f32,
        y: *mut f32,
    ) -> bool;
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

    pub fn acc_lapack_sgetrf(matrix: *mut f32, dimension: i32, pivots: *mut i32) -> i32;
    pub fn acc_lapack_sgesv(
        matrix: *mut f32,
        dimension: i32,
        rhs: *mut f32,
        rhs_count: i32,
        pivots: *mut i32,
    ) -> i32;

    pub fn acc_bnns_relu_f32(input: *const f32, output: *mut f32, length: usize) -> i32;
    pub fn acc_bnns_sigmoid_f32(input: *const f32, output: *mut f32, length: usize) -> i32;

    pub fn acc_sparse_dot_dense_f32(
        nz: u64,
        values: *const f32,
        indices: *const i64,
        dense: *const f32,
    ) -> f32;
    pub fn acc_sparse_dot_sparse_f32(
        lhs_count: u64,
        lhs_values: *const f32,
        lhs_indices: *const i64,
        rhs_count: u64,
        rhs_values: *const f32,
        rhs_indices: *const i64,
    ) -> f32;
    pub fn acc_sparse_add_to_dense_f32(
        nz: u64,
        alpha: f32,
        values: *const f32,
        indices: *const i64,
        dense: *mut f32,
    ) -> bool;

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

    pub fn acc_simd_add_f32x4(lhs: *const f32, rhs: *const f32, output: *mut f32) -> bool;
    pub fn acc_simd_dot_f32x4(lhs: *const f32, rhs: *const f32, output: *mut f32) -> bool;
    pub fn acc_simd_length_f32x4(input: *const f32, output: *mut f32) -> bool;
    pub fn acc_simd_normalize_f32x4(input: *const f32, output: *mut f32) -> bool;

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
