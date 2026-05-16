use crate::bridge;
use crate::error::{Error, Result};

pub type Float4 = [f32; 4];

fn unary_simd_op(
    input: Float4,
    f: unsafe extern "C" fn(*const f32, *mut f32) -> bool,
) -> Result<Float4> {
    let mut output = [0.0_f32; 4];
    // SAFETY: Both arrays are valid for exactly four `f32` values.
    let ok = unsafe { f(input.as_ptr(), output.as_mut_ptr()) };
    if ok {
        Ok(output)
    } else {
        Err(Error::OperationFailed("SIMD bridge operation failed"))
    }
}

pub fn add_f32x4(lhs: Float4, rhs: Float4) -> Result<Float4> {
    let mut output = [0.0_f32; 4];
    // SAFETY: All arrays are valid for exactly four `f32` values.
    let ok = unsafe { bridge::acc_simd_add_f32x4(lhs.as_ptr(), rhs.as_ptr(), output.as_mut_ptr()) };
    if ok {
        Ok(output)
    } else {
        Err(Error::OperationFailed("SIMD add failed"))
    }
}

pub fn dot_f32x4(lhs: Float4, rhs: Float4) -> Result<f32> {
    let mut output = 0.0_f32;
    // SAFETY: All arrays are valid for exactly four `f32` values.
    let ok = unsafe { bridge::acc_simd_dot_f32x4(lhs.as_ptr(), rhs.as_ptr(), &mut output) };
    if ok {
        Ok(output)
    } else {
        Err(Error::OperationFailed("SIMD dot product failed"))
    }
}

pub fn length_f32x4(input: Float4) -> Result<f32> {
    let mut output = 0.0_f32;
    // SAFETY: Both arrays are valid for exactly four `f32` values.
    let ok = unsafe { bridge::acc_simd_length_f32x4(input.as_ptr(), &mut output) };
    if ok {
        Ok(output)
    } else {
        Err(Error::OperationFailed("SIMD length failed"))
    }
}

pub fn normalize_f32x4(input: Float4) -> Result<Float4> {
    if input.iter().all(|value| *value == 0.0) {
        return Err(Error::InvalidValue("cannot normalize the zero vector"));
    }
    unary_simd_op(input, bridge::acc_simd_normalize_f32x4)
}
