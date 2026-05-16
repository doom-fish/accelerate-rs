use crate::bridge;
use crate::error::{Error, Result};

pub type SparseIndex = i64;

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

pub fn dot_dense_f32(values: &[f32], indices: &[SparseIndex], dense: &[f32]) -> Result<f32> {
    validate_sparse_entries(values, indices)?;
    validate_dense_coverage(indices, dense.len())?;
    if values.is_empty() {
        return Ok(0.0);
    }

    // SAFETY: Inputs are validated for length, ordering, and dense coverage.
    Ok(unsafe {
        bridge::acc_sparse_dot_dense_f32(
            values.len() as u64,
            values.as_ptr(),
            indices.as_ptr(),
            dense.as_ptr(),
        )
    })
}

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

    // SAFETY: Inputs are validated for length and monotonic indices.
    Ok(unsafe {
        bridge::acc_sparse_dot_sparse_f32(
            lhs_values.len() as u64,
            lhs_values.as_ptr(),
            lhs_indices.as_ptr(),
            rhs_values.len() as u64,
            rhs_values.as_ptr(),
            rhs_indices.as_ptr(),
        )
    })
}

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

    // SAFETY: Inputs are validated for length, ordering, and dense coverage.
    let ok = unsafe {
        bridge::acc_sparse_add_to_dense_f32(
            values.len() as u64,
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
