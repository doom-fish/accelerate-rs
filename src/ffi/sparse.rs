#![allow(missing_docs, non_camel_case_types, non_upper_case_globals)]

pub type sparse_dimension = u64;
pub type sparse_stride = i64;
pub type sparse_index = i64;

#[link(name = "Accelerate", kind = "framework")]
unsafe extern "C" {
    pub fn sparse_inner_product_dense_float(
        nz: sparse_dimension,
        x: *const f32,
        indx: *const sparse_index,
        y: *const f32,
        incy: sparse_stride,
    ) -> f32;
    pub fn sparse_inner_product_sparse_float(
        nzx: sparse_dimension,
        nzy: sparse_dimension,
        x: *const f32,
        indx: *const sparse_index,
        y: *const f32,
        indy: *const sparse_index,
    ) -> f32;
    pub fn sparse_vector_add_with_scale_dense_float(
        nz: sparse_dimension,
        alpha: f32,
        x: *const f32,
        indx: *const sparse_index,
        y: *mut f32,
        incy: sparse_stride,
    );
}
