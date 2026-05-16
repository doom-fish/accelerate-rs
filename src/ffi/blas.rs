#![allow(missing_docs, non_camel_case_types, non_upper_case_globals)]

pub type CBLAS_ORDER = i32;
pub type CBLAS_TRANSPOSE = i32;

#[link(name = "Accelerate", kind = "framework")]
unsafe extern "C" {
    pub fn cblas_sdot(n: i32, x: *const f32, inc_x: i32, y: *const f32, inc_y: i32) -> f32;
    pub fn cblas_sgemv(
        order: CBLAS_ORDER,
        transpose: CBLAS_TRANSPOSE,
        m: i32,
        n: i32,
        alpha: f32,
        a: *const f32,
        lda: i32,
        x: *const f32,
        inc_x: i32,
        beta: f32,
        y: *mut f32,
        inc_y: i32,
    );
    pub fn cblas_sgemm(
        order: CBLAS_ORDER,
        transpose_a: CBLAS_TRANSPOSE,
        transpose_b: CBLAS_TRANSPOSE,
        m: i32,
        n: i32,
        k: i32,
        alpha: f32,
        a: *const f32,
        lda: i32,
        b: *const f32,
        ldb: i32,
        beta: f32,
        c: *mut f32,
        ldc: i32,
    );
}
