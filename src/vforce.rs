use crate::bridge;
use crate::error::{Error, Result};

type UnaryVectorOp = unsafe extern "C" fn(*const f32, *mut f32, usize) -> bool;

fn unary_vector_op(values: &[f32], f: UnaryVectorOp) -> Result<Vec<f32>> {
    let mut out = vec![0.0_f32; values.len()];
    if values.is_empty() {
        return Ok(out);
    }

    // SAFETY: The buffers are valid for `values.len()` contiguous `f32` elements.
    let ok = unsafe { f(values.as_ptr(), out.as_mut_ptr(), values.len()) };
    if ok {
        Ok(out)
    } else {
        Err(Error::OperationFailed("vForce operation failed"))
    }
}

pub fn sin_f32(values: &[f32]) -> Result<Vec<f32>> {
    unary_vector_op(values, bridge::acc_vforce_sin_f32)
}

pub fn cos_f32(values: &[f32]) -> Result<Vec<f32>> {
    unary_vector_op(values, bridge::acc_vforce_cos_f32)
}

pub fn exp_f32(values: &[f32]) -> Result<Vec<f32>> {
    unary_vector_op(values, bridge::acc_vforce_exp_f32)
}

pub fn log_f32(values: &[f32]) -> Result<Vec<f32>> {
    unary_vector_op(values, bridge::acc_vforce_log_f32)
}

pub fn sqrt_f32(values: &[f32]) -> Result<Vec<f32>> {
    unary_vector_op(values, bridge::acc_vforce_sqrt_f32)
}
