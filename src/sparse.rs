use crate::blas::{blas_order, blas_transpose};
use crate::bridge;
use crate::error::{Error, Result};
use core::ffi::c_void;
use core::ptr;

/// Sparse index type used by the `sparse_*_float` routines.
pub type SparseIndex = i64;

/// `sparse_matrix_property` constants.
pub mod sparse_matrix_property {
    /// `sparse_matrix_property` flag for upper-triangular matrices.
    pub const UPPER_TRIANGULAR: i32 = 1;
    /// `sparse_matrix_property` flag for lower-triangular matrices.
    pub const LOWER_TRIANGULAR: i32 = 2;
    /// `sparse_matrix_property` flag for upper-symmetric matrices.
    pub const UPPER_SYMMETRIC: i32 = 4;
    /// `sparse_matrix_property` flag for lower-symmetric matrices.
    pub const LOWER_SYMMETRIC: i32 = 8;
}

/// `sparse_status` constants.
pub mod sparse_status {
    /// `sparse_status` value returned for successful sparse operations.
    pub const SUCCESS: i32 = 0;
    /// `sparse_status` value returned when a sparse argument is invalid.
    pub const ILLEGAL_PARAMETER: i32 = -1000;
    /// `sparse_status` value returned when `sparse_set_matrix_property` rejects a property.
    pub const CANNOT_SET_PROPERTY: i32 = -1001;
    /// `sparse_status` value returned when the sparse runtime reports a system failure.
    pub const SYSTEM_ERROR: i32 = -1002;
}

fn u64_len(value: usize) -> Result<u64> {
    u64::try_from(value).map_err(|_| Error::OperationFailed("sparse dimension overflowed"))
}

fn i64_index(value: usize) -> Result<i64> {
    i64::try_from(value).map_err(|_| Error::OperationFailed("sparse index overflowed"))
}

fn usize_dimension(value: u64) -> Result<usize> {
    usize::try_from(value).map_err(|_| Error::OperationFailed("sparse dimension exceeds usize"))
}

fn usize_count(value: i64) -> Result<usize> {
    if value < 0 {
        return Err(Error::SparseStatus(
            i32::try_from(value).unwrap_or(sparse_status::SYSTEM_ERROR),
        ));
    }
    usize::try_from(value).map_err(|_| Error::OperationFailed("sparse count exceeds usize"))
}

fn sparse_result(status: i32) -> Result<()> {
    if status == sparse_status::SUCCESS {
        Ok(())
    } else {
        Err(Error::SparseStatus(status))
    }
}

fn validate_sparse_entries(values: &[f32], indices: &[SparseIndex]) -> Result<()> {
    if values.len() != indices.len() {
        return Err(Error::InvalidLength {
            expected: values.len(),
            actual: indices.len(),
        });
    }
    for window in indices.windows(2) {
        if window[0] >= window[1] {
            return Err(Error::InvalidValue(
                "sparse indices must be strictly increasing and unique",
            ));
        }
    }
    Ok(())
}

fn validate_dense_coverage(indices: &[SparseIndex], dense_len: usize) -> Result<()> {
    if let Some(&max_index) = indices.last() {
        let max_index = usize::try_from(max_index)
            .map_err(|_| Error::InvalidValue("sparse indices must be non-negative"))?;
        if max_index >= dense_len {
            return Err(Error::InvalidLength {
                expected: max_index + 1,
                actual: dense_len,
            });
        }
    }
    Ok(())
}

/// Owned sparse single-precision matrix handle backed by the Swift bridge.
pub struct SparseMatrixF32 {
    ptr: *mut c_void,
}

unsafe impl Send for SparseMatrixF32 {}
unsafe impl Sync for SparseMatrixF32 {}

impl Drop for SparseMatrixF32 {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            // SAFETY: `ptr` is an opaque Swift object retained by the bridge.
            unsafe { bridge::acc_release_handle(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl SparseMatrixF32 {
    /// Creates a sparse single-precision matrix with `sparse_matrix_create_float`.
    #[must_use]
    pub fn new(rows: usize, columns: usize) -> Option<Self> {
        if rows == 0 || columns == 0 {
            return None;
        }
        let rows = u64::try_from(rows).ok()?;
        let columns = u64::try_from(columns).ok()?;

        // SAFETY: Pure constructor over scalar dimensions.
        let ptr = unsafe { bridge::acc_sparse_matrix_f32_create(rows, columns) };
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    /// Sets a `sparse_matrix_property` on the matrix with `sparse_set_matrix_property`.
    pub fn set_property(&mut self, property: i32) -> Result<()> {
        // SAFETY: `self.ptr` is a live bridge handle.
        sparse_result(unsafe { bridge::acc_sparse_matrix_f32_set_property(self.ptr, property) })
    }

    /// Inserts one matrix entry with `sparse_insert_entry_float`.
    pub fn insert_entry(&mut self, row: usize, column: usize, value: f32) -> Result<()> {
        let rows = self.rows()?;
        let columns = self.columns()?;
        if row >= rows || column >= columns {
            return Err(Error::InvalidValue(
                "sparse entry coordinates must be within matrix bounds",
            ));
        }

        let row = i64_index(row)?;
        let column = i64_index(column)?;
        // SAFETY: Bounds were validated and `self.ptr` is a live bridge handle.
        sparse_result(unsafe {
            bridge::acc_sparse_matrix_f32_insert_entry(self.ptr, value, row, column)
        })
    }

    /// Finalizes pending sparse edits with `sparse_commit`.
    pub fn commit(&mut self) -> Result<()> {
        // SAFETY: `self.ptr` is a live bridge handle.
        sparse_result(unsafe { bridge::acc_sparse_matrix_f32_commit(self.ptr) })
    }

    /// Returns the row count reported by `sparse_get_matrix_number_of_rows`.
    pub fn rows(&self) -> Result<usize> {
        // SAFETY: `self.ptr` is a live bridge handle.
        usize_dimension(unsafe { bridge::acc_sparse_matrix_f32_rows(self.ptr) })
    }

    /// Returns the column count reported by `sparse_get_matrix_number_of_columns`.
    pub fn columns(&self) -> Result<usize> {
        // SAFETY: `self.ptr` is a live bridge handle.
        usize_dimension(unsafe { bridge::acc_sparse_matrix_f32_columns(self.ptr) })
    }

    /// Returns the nonzero count reported by `sparse_get_matrix_nonzero_count`.
    pub fn nonzero_count(&self) -> Result<usize> {
        // SAFETY: `self.ptr` is a live bridge handle.
        usize_count(unsafe { bridge::acc_sparse_matrix_f32_nonzero_count(self.ptr) })
    }

    /// Solves a sparse triangular system against a dense vector with `sparse_vector_triangular_solve_dense_float`.
    pub fn triangular_solve_vector(
        &self,
        transpose: i32,
        alpha: f32,
        values: &mut [f32],
    ) -> Result<()> {
        let rows = self.rows()?;
        let columns = self.columns()?;
        if rows != columns {
            return Err(Error::InvalidValue(
                "sparse triangular solve requires a square matrix",
            ));
        }
        if values.len() != rows {
            return Err(Error::InvalidLength {
                expected: rows,
                actual: values.len(),
            });
        }

        let len = u64_len(values.len())?;
        // SAFETY: The matrix and dense vector satisfy the API preconditions.
        sparse_result(unsafe {
            bridge::acc_sparse_matrix_f32_triangular_solve_vector(
                self.ptr,
                transpose,
                alpha,
                values.as_mut_ptr(),
                len,
            )
        })
    }

    /// Solves a sparse triangular system against a row-major dense matrix with `sparse_matrix_triangular_solve_dense_float`.
    pub fn triangular_solve_matrix_row_major(
        &self,
        transpose: i32,
        rhs_columns: usize,
        alpha: f32,
        values: &mut [f32],
    ) -> Result<()> {
        let rows = self.rows()?;
        let columns = self.columns()?;
        if rows != columns {
            return Err(Error::InvalidValue(
                "sparse triangular solve requires a square matrix",
            ));
        }
        let expected = rows
            .checked_mul(rhs_columns)
            .ok_or(Error::OperationFailed("sparse rhs dimensions overflowed"))?;
        if values.len() != expected {
            return Err(Error::InvalidLength {
                expected,
                actual: values.len(),
            });
        }
        if rhs_columns == 0 {
            return Ok(());
        }

        let rhs_count = u64_len(rhs_columns)?;
        let ldb = u64_len(rhs_columns)?;
        // SAFETY: The matrix and dense matrix satisfy the API preconditions.
        sparse_result(unsafe {
            bridge::acc_sparse_matrix_f32_triangular_solve_matrix(
                self.ptr,
                blas_order::ROW_MAJOR,
                transpose,
                rhs_count,
                alpha,
                values.as_mut_ptr(),
                ldb,
            )
        })
    }
}

/// Wraps `sparse_inner_product_dense_float`.
pub fn dot_dense_f32(values: &[f32], indices: &[SparseIndex], dense: &[f32]) -> Result<f32> {
    validate_sparse_entries(values, indices)?;
    validate_dense_coverage(indices, dense.len())?;
    if values.is_empty() {
        return Ok(0.0);
    }

    let nz = u64_len(values.len())?;
    // SAFETY: Inputs are validated for length, ordering, and dense coverage.
    Ok(unsafe {
        bridge::acc_sparse_dot_dense_f32(nz, values.as_ptr(), indices.as_ptr(), dense.as_ptr())
    })
}

/// Wraps `sparse_inner_product_sparse_float`.
pub fn dot_sparse_f32(
    lhs_values: &[f32],
    lhs_indices: &[SparseIndex],
    rhs_values: &[f32],
    rhs_indices: &[SparseIndex],
) -> Result<f32> {
    validate_sparse_entries(lhs_values, lhs_indices)?;
    validate_sparse_entries(rhs_values, rhs_indices)?;
    if lhs_values.is_empty() || rhs_values.is_empty() {
        return Ok(0.0);
    }

    let lhs_count = u64_len(lhs_values.len())?;
    let rhs_count = u64_len(rhs_values.len())?;
    // SAFETY: Inputs are validated for length and monotonic indices.
    Ok(unsafe {
        bridge::acc_sparse_dot_sparse_f32(
            lhs_count,
            lhs_values.as_ptr(),
            lhs_indices.as_ptr(),
            rhs_count,
            rhs_values.as_ptr(),
            rhs_indices.as_ptr(),
        )
    })
}

/// Wraps `sparse_vector_add_with_scale_dense_float`.
pub fn add_to_dense_f32(
    values: &[f32],
    indices: &[SparseIndex],
    alpha: f32,
    dense: &mut [f32],
) -> Result<()> {
    validate_sparse_entries(values, indices)?;
    validate_dense_coverage(indices, dense.len())?;
    if values.is_empty() {
        return Ok(());
    }

    let nz = u64_len(values.len())?;
    // SAFETY: Inputs are validated for length, ordering, and dense coverage.
    let ok = unsafe {
        bridge::acc_sparse_add_to_dense_f32(
            nz,
            alpha,
            values.as_ptr(),
            indices.as_ptr(),
            dense.as_mut_ptr(),
        )
    };
    if ok {
        Ok(())
    } else {
        Err(Error::SparseStatus(-1))
    }
}

#[allow(dead_code)]
const _: i32 = blas_transpose::NO_TRANS;
