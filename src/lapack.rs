use crate::bridge;
use crate::error::{Error, Result};

fn checked_square_length(dimension: usize, actual: usize) -> Result<()> {
    let expected = dimension
        .checked_mul(dimension)
        .ok_or(Error::OperationFailed("matrix dimensions overflowed"))?;
    if actual == expected {
        Ok(())
    } else {
        Err(Error::InvalidLength { expected, actual })
    }
}

fn i32_len(value: usize) -> Result<i32> {
    i32::try_from(value).map_err(|_| Error::OperationFailed("dimension exceeds i32"))
}

fn lapack_result(info: i32) -> Result<()> {
    if info == 0 {
        Ok(())
    } else {
        Err(Error::LapackInfo(info))
    }
}

/// Compact LU factorization in column-major layout.
#[derive(Debug, Clone, PartialEq)]
pub struct LuDecompositionF32 {
    factors: Vec<f32>,
    pivots: Vec<i32>,
    dimension: usize,
    info: i32,
}

impl LuDecompositionF32 {
    /// Returns the packed LU factors produced by `sgetrf_`.
    #[must_use]
    pub fn factors(&self) -> &[f32] {
        &self.factors
    }

    /// Returns the pivot indices produced by `sgetrf_`.
    #[must_use]
    pub fn pivots(&self) -> &[i32] {
        &self.pivots
    }

    /// Returns the matrix dimension used with `sgetrf_`.
    #[must_use]
    pub const fn dimension(&self) -> usize {
        self.dimension
    }

    pub const fn info(&self) -> i32 {
        self.info
    }

    pub const fn is_singular(&self) -> bool {
        self.info > 0
    }
}

/// Compute an LU factorization of a column-major square matrix.
pub fn lu_decompose_f32(matrix: &[f32], dimension: usize) -> Result<LuDecompositionF32> {
    checked_square_length(dimension, matrix.len())?;
    let dimension_i32 = i32_len(dimension)?;
    if dimension == 0 {
        return Ok(LuDecompositionF32 {
            factors: Vec::new(),
            pivots: Vec::new(),
            dimension,
            info: 0,
        });
    }

    let mut factors = matrix.to_vec();
    let mut pivots = vec![0_i32; dimension];
    // SAFETY: Buffers are valid for `dimension * dimension` matrix entries and `dimension` pivots.
    let info = unsafe {
        bridge::acc_lapack_sgetrf(factors.as_mut_ptr(), dimension_i32, pivots.as_mut_ptr())
    };
    if info < 0 {
        return Err(Error::LapackInfo(info));
    }

    Ok(LuDecompositionF32 {
        factors,
        pivots,
        dimension,
        info,
    })
}

/// Solve `A * X = B` for a column-major square matrix `A` and one-or-more right-hand sides `B`.
pub fn solve_linear_system_f32(matrix: &[f32], dimension: usize, rhs: &[f32]) -> Result<Vec<f32>> {
    checked_square_length(dimension, matrix.len())?;
    if dimension == 0 {
        return if rhs.is_empty() {
            Ok(Vec::new())
        } else {
            Err(Error::InvalidLength {
                expected: 0,
                actual: rhs.len(),
            })
        };
    }
    if rhs.is_empty() {
        return Err(Error::InvalidLength {
            expected: dimension,
            actual: 0,
        });
    }
    if rhs.len() % dimension != 0 {
        return Err(Error::InvalidLength {
            expected: dimension,
            actual: rhs.len(),
        });
    }

    let dimension_i32 = i32_len(dimension)?;
    let rhs_count = i32_len(rhs.len() / dimension)?;
    let mut factors = matrix.to_vec();
    let mut solution = rhs.to_vec();
    let mut pivots = vec![0_i32; dimension];

    // SAFETY: Buffers are valid for LAPACK's in-place solve routine.
    let info = unsafe {
        bridge::acc_lapack_sgesv(
            factors.as_mut_ptr(),
            dimension_i32,
            solution.as_mut_ptr(),
            rhs_count,
            pivots.as_mut_ptr(),
        )
    };
    lapack_result(info)?;
    Ok(solution)
}
