use crate::bridge;
use crate::error::{Error, Result};

type UnaryVectorOp = unsafe extern "C" fn(*const f32, *mut f32, usize) -> bool;

const MAX_CHUNK_LEN: usize = 0x7FFF_FFFF;

fn unary_vector_op(values: &[f32], f: UnaryVectorOp, chunk_len: usize) -> Result<Vec<f32>> {
    let mut out = vec![0.0_f32; values.len()];
    for (input, output) in values.chunks(chunk_len).zip(out.chunks_mut(chunk_len)) {
        // SAFETY: The buffers are valid for `input.len()` contiguous `f32` elements.
        let ok = unsafe { f(input.as_ptr(), output.as_mut_ptr(), input.len()) };
        if !ok {
            return Err(Error::OperationFailed("vForce operation failed"));
        }
    }
    Ok(out)
}

/// Wraps `vvsinf`.
pub fn sin_f32(values: &[f32]) -> Result<Vec<f32>> {
    unary_vector_op(values, bridge::acc_vforce_sin_f32, MAX_CHUNK_LEN)
}

/// Wraps `vvcosf`.
pub fn cos_f32(values: &[f32]) -> Result<Vec<f32>> {
    unary_vector_op(values, bridge::acc_vforce_cos_f32, MAX_CHUNK_LEN)
}

/// Wraps `vvexpf`.
pub fn exp_f32(values: &[f32]) -> Result<Vec<f32>> {
    unary_vector_op(values, bridge::acc_vforce_exp_f32, MAX_CHUNK_LEN)
}

/// Wraps `vvlogf`.
pub fn log_f32(values: &[f32]) -> Result<Vec<f32>> {
    unary_vector_op(values, bridge::acc_vforce_log_f32, MAX_CHUNK_LEN)
}

/// Wraps `vvsqrtf`.
pub fn sqrt_f32(values: &[f32]) -> Result<Vec<f32>> {
    unary_vector_op(values, bridge::acc_vforce_sqrt_f32, MAX_CHUNK_LEN)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunked_calls_cover_every_element() {
        let values: Vec<f32> = (0_u8..10).map(|value| f32::from(value) * 0.5).collect();
        let whole = unary_vector_op(&values, bridge::acc_vforce_sqrt_f32, MAX_CHUNK_LEN)
            .expect("single call");
        for chunk_len in [1, 3, 4, 9, 10, 11] {
            let chunked = unary_vector_op(&values, bridge::acc_vforce_sqrt_f32, chunk_len)
                .expect("chunked calls");
            assert_eq!(chunked, whole, "chunk length {chunk_len}");
        }
        assert!(whole
            .iter()
            .zip(&values)
            .all(|(root, value)| (root * root - value).abs() < 1.0e-5));
        assert!(unary_vector_op(&[], bridge::acc_vforce_sqrt_f32, 3)
            .expect("empty")
            .is_empty());
    }
}
