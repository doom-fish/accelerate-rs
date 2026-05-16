use crate::error::{Error, Result};
use crate::ffi;

/// `CBLAS_ORDER` constants.
pub mod blas_order {
    pub const ROW_MAJOR: i32 = 101;
    pub const COL_MAJOR: i32 = 102;
}

/// `CBLAS_TRANSPOSE` constants.
pub mod blas_transpose {
    pub const NO_TRANS: i32 = 111;
    pub const TRANS: i32 = 112;
    pub const CONJ_TRANS: i32 = 113;
}

fn i32_len(value: usize) -> Result<i32> {
    i32::try_from(value).map_err(|_| Error::OperationFailed("dimension exceeds i32"))
}

/// Compute the single-precision dot product of two vectors.
pub fn sdot(x: &[f32], y: &[f32]) -> Result<f32> {
    if x.len() != y.len() {
        return Err(Error::InvalidLength {
            expected: x.len(),
            actual: y.len(),
        });
    }

    let n = i32_len(x.len())?;
    // SAFETY: The slices are valid for `n` contiguous `f32` values.
    Ok(unsafe { ffi::cblas_sdot(n, x.as_ptr(), 1, y.as_ptr(), 1) })
}

/// Multiply a row-major matrix by a vector with `sgemv` and write the result into `y`.
pub fn sgemv_row_major(
    rows: usize,
    columns: usize,
    alpha: f32,
    matrix: &[f32],
    x: &[f32],
    beta: f32,
    y: &mut [f32],
) -> Result<()> {
    let expected_matrix = rows
        .checked_mul(columns)
        .ok_or(Error::OperationFailed("matrix dimensions overflowed"))?;
    if matrix.len() != expected_matrix {
        return Err(Error::InvalidLength {
            expected: expected_matrix,
            actual: matrix.len(),
        });
    }
    if x.len() != columns {
        return Err(Error::InvalidLength {
            expected: columns,
            actual: x.len(),
        });
    }
    if y.len() != rows {
        return Err(Error::InvalidLength {
            expected: rows,
            actual: y.len(),
        });
    }

    let rows_i32 = i32_len(rows)?;
    let columns_i32 = i32_len(columns)?;
    // SAFETY: The matrix and vector buffers are valid for the provided dimensions and strides.
    unsafe {
        ffi::cblas_sgemv(
            blas_order::ROW_MAJOR,
            blas_transpose::NO_TRANS,
            rows_i32,
            columns_i32,
            alpha,
            matrix.as_ptr(),
            columns_i32,
            x.as_ptr(),
            1,
            beta,
            y.as_mut_ptr(),
            1,
        );
    }
    Ok(())
}

/// Multiply two row-major matrices with `sgemm` and write the result into `output`.
#[allow(clippy::too_many_arguments)]
pub fn sgemm_row_major(
    rows: usize,
    columns: usize,
    inner_dimension: usize,
    alpha: f32,
    lhs: &[f32],
    rhs: &[f32],
    beta: f32,
    output: &mut [f32],
) -> Result<()> {
    let expected_lhs = rows
        .checked_mul(inner_dimension)
        .ok_or(Error::OperationFailed("matrix dimensions overflowed"))?;
    let expected_rhs = inner_dimension
        .checked_mul(columns)
        .ok_or(Error::OperationFailed("matrix dimensions overflowed"))?;
    let expected_output = rows
        .checked_mul(columns)
        .ok_or(Error::OperationFailed("matrix dimensions overflowed"))?;
    if lhs.len() != expected_lhs {
        return Err(Error::InvalidLength {
            expected: expected_lhs,
            actual: lhs.len(),
        });
    }
    if rhs.len() != expected_rhs {
        return Err(Error::InvalidLength {
            expected: expected_rhs,
            actual: rhs.len(),
        });
    }
    if output.len() != expected_output {
        return Err(Error::InvalidLength {
            expected: expected_output,
            actual: output.len(),
        });
    }

    let rows_i32 = i32_len(rows)?;
    let columns_i32 = i32_len(columns)?;
    let inner_dimension_i32 = i32_len(inner_dimension)?;
    // SAFETY: The matrix buffers are valid for the provided dimensions and leading dimensions.
    unsafe {
        ffi::cblas_sgemm(
            blas_order::ROW_MAJOR,
            blas_transpose::NO_TRANS,
            blas_transpose::NO_TRANS,
            rows_i32,
            columns_i32,
            inner_dimension_i32,
            alpha,
            lhs.as_ptr(),
            inner_dimension_i32,
            rhs.as_ptr(),
            columns_i32,
            beta,
            output.as_mut_ptr(),
            columns_i32,
        );
    }
    Ok(())
}
