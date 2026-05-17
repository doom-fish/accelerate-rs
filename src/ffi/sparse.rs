#![allow(missing_docs, non_camel_case_types, non_upper_case_globals)]

use core::ffi::c_void;

pub enum sparse_m_float {}
pub type sparse_matrix_float = *mut sparse_m_float;
pub type sparse_dimension = u64;
pub type sparse_stride = i64;
pub type sparse_index = i64;
pub type sparse_status = i32;
pub type sparse_matrix_property = i32;

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
    pub fn sparse_matrix_create_float(
        m: sparse_dimension,
        n: sparse_dimension,
    ) -> sparse_matrix_float;
    pub fn sparse_insert_entry_float(
        a: sparse_matrix_float,
        val: f32,
        i: sparse_index,
        j: sparse_index,
    ) -> sparse_status;
    pub fn sparse_vector_triangular_solve_dense_float(
        transt: i32,
        alpha: f32,
        t: sparse_matrix_float,
        x: *mut f32,
        incx: sparse_stride,
    ) -> sparse_status;
    pub fn sparse_matrix_triangular_solve_dense_float(
        order: i32,
        transt: i32,
        nrhs: sparse_dimension,
        alpha: f32,
        t: sparse_matrix_float,
        b: *mut f32,
        ldb: sparse_dimension,
    ) -> sparse_status;
    pub fn sparse_commit(a: *mut c_void) -> sparse_status;
    pub fn sparse_set_matrix_property(
        a: *mut c_void,
        pname: sparse_matrix_property,
    ) -> sparse_status;
    pub fn sparse_get_matrix_number_of_rows(a: *mut c_void) -> sparse_dimension;
    pub fn sparse_get_matrix_number_of_columns(a: *mut c_void) -> sparse_dimension;
    pub fn sparse_get_matrix_nonzero_count(a: *mut c_void) -> i64;
    pub fn sparse_matrix_destroy(a: *mut c_void) -> sparse_status;
}

#[allow(
    missing_docs,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_imports,
    dead_code,
    improper_ctypes,
    improper_ctypes_definitions,
    unnecessary_transmutes
)]
mod generated {
    use super::*;
    include!("generated/sparse_missing.rs");
}

pub use generated::*;
