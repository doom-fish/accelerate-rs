use crate::error::{Error, Result};
use crate::ffi;
use core::ptr;

/// `FFTDirection` constants.
pub mod fft_direction {
    pub const FORWARD: i32 = 1;
    pub const INVERSE: i32 = -1;
}

/// `FFTRadix` constants.
pub mod fft_radix {
    pub const RADIX2: i32 = 0;
    pub const RADIX3: i32 = 1;
    pub const RADIX5: i32 = 2;
}

/// Window-generation flags.
pub mod window_flags {
    pub const HALF_WINDOW: i32 = 1;
    pub const HANN_DENORM: i32 = 0;
    pub const HANN_NORM: i32 = 2;
}

/// Owned `FFTSetup` handle.
pub struct FftSetup {
    ptr: ffi::FFTSetup,
}

unsafe impl Send for FftSetup {}
unsafe impl Sync for FftSetup {}

impl Drop for FftSetup {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            // SAFETY: `ptr` was returned by `vDSP_create_fftsetup` and is owned by this wrapper.
            unsafe { ffi::vDSP_destroy_fftsetup(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl FftSetup {
    #[must_use]
    pub fn new(log2n: usize, radix: i32) -> Option<Self> {
        // SAFETY: Pure constructor over scalar inputs.
        let ptr = unsafe { ffi::vDSP_create_fftsetup(log2n, radix) };
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    pub fn fft_zip(
        &self,
        real: &mut [f32],
        imag: &mut [f32],
        log2n: usize,
        direction: i32,
    ) -> Result<()> {
        let shift = u32::try_from(log2n)
            .map_err(|_| Error::OperationFailed("FFT log2 length exceeds u32"))?;
        let expected = 1_usize
            .checked_shl(shift)
            .ok_or(Error::OperationFailed("FFT length overflowed"))?;
        if real.len() != expected {
            return Err(Error::InvalidLength {
                expected,
                actual: real.len(),
            });
        }
        if imag.len() != expected {
            return Err(Error::InvalidLength {
                expected,
                actual: imag.len(),
            });
        }

        let split = ffi::DSPSplitComplex {
            realp: real.as_mut_ptr(),
            imagp: imag.as_mut_ptr(),
        };
        // SAFETY: The split-complex buffers are valid for `expected` elements.
        unsafe { ffi::vDSP_fft_zip(self.ptr, &split, 1, log2n, direction) };
        Ok(())
    }
}

/// Owned `vDSP_biquad_Setup` handle.
pub struct BiquadSetup {
    ptr: ffi::vDSP_biquad_Setup,
}

unsafe impl Send for BiquadSetup {}
unsafe impl Sync for BiquadSetup {}

impl Drop for BiquadSetup {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            // SAFETY: `ptr` was returned by `vDSP_biquad_CreateSetup` and is owned by this wrapper.
            unsafe { ffi::vDSP_biquad_DestroySetup(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl BiquadSetup {
    #[must_use]
    pub fn new(coefficients: &[f64]) -> Option<Self> {
        if coefficients.is_empty() || coefficients.len() % 5 != 0 {
            return None;
        }

        let sections = coefficients.len() / 5;
        // SAFETY: `coefficients` is valid for `sections * 5` entries.
        let ptr = unsafe { ffi::vDSP_biquad_CreateSetup(coefficients.as_ptr(), sections) };
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    pub fn apply(&self, delay: &mut [f32], input: &[f32], output: &mut [f32]) -> Result<()> {
        if input.len() != output.len() {
            return Err(Error::InvalidLength {
                expected: input.len(),
                actual: output.len(),
            });
        }

        // SAFETY: The slices are valid for the provided strides and element count.
        unsafe {
            ffi::vDSP_biquad(
                self.ptr,
                delay.as_mut_ptr(),
                input.as_ptr(),
                1,
                output.as_mut_ptr(),
                1,
                input.len(),
            );
        }
        Ok(())
    }
}

fn binary_vector_op(
    a: &[f32],
    b: &[f32],
    f: unsafe extern "C" fn(*const f32, isize, *const f32, isize, *mut f32, isize, usize),
) -> Result<Vec<f32>> {
    if a.len() != b.len() {
        return Err(Error::InvalidLength {
            expected: a.len(),
            actual: b.len(),
        });
    }

    let mut out = vec![0.0_f32; a.len()];
    // SAFETY: All slices are valid for `a.len()` contiguous `f32` elements.
    unsafe { f(a.as_ptr(), 1, b.as_ptr(), 1, out.as_mut_ptr(), 1, a.len()) };
    Ok(out)
}

pub fn add_f32(a: &[f32], b: &[f32]) -> Result<Vec<f32>> {
    binary_vector_op(a, b, ffi::vDSP_vadd)
}

pub fn sub_f32(a: &[f32], b: &[f32]) -> Result<Vec<f32>> {
    if a.len() != b.len() {
        return Err(Error::InvalidLength {
            expected: a.len(),
            actual: b.len(),
        });
    }

    let mut out = vec![0.0_f32; a.len()];
    // SAFETY: `vDSP_vsub` computes `a - b` with `b` and `a` intentionally swapped in the argument list.
    unsafe { ffi::vDSP_vsub(b.as_ptr(), 1, a.as_ptr(), 1, out.as_mut_ptr(), 1, a.len()) };
    Ok(out)
}

pub fn dot_f32(a: &[f32], b: &[f32]) -> Result<f32> {
    if a.len() != b.len() {
        return Err(Error::InvalidLength {
            expected: a.len(),
            actual: b.len(),
        });
    }

    let mut out = 0.0_f32;
    // SAFETY: The slices are valid for `a.len()` contiguous `f32` elements.
    unsafe { ffi::vDSP_dotpr(a.as_ptr(), 1, b.as_ptr(), 1, &mut out, a.len()) };
    Ok(out)
}

fn reduce_f32(
    values: &[f32],
    f: unsafe extern "C" fn(*const f32, isize, *mut f32, usize),
) -> Result<f32> {
    if values.is_empty() {
        return Err(Error::InvalidLength {
            expected: 1,
            actual: 0,
        });
    }

    let mut out = 0.0_f32;
    // SAFETY: The slice is valid for `values.len()` contiguous `f32` elements.
    unsafe { f(values.as_ptr(), 1, &mut out, values.len()) };
    Ok(out)
}

pub fn max_f32(values: &[f32]) -> Result<f32> {
    reduce_f32(values, ffi::vDSP_maxv)
}

pub fn min_f32(values: &[f32]) -> Result<f32> {
    reduce_f32(values, ffi::vDSP_minv)
}

pub fn mean_f32(values: &[f32]) -> Result<f32> {
    reduce_f32(values, ffi::vDSP_meanv)
}

pub fn sum_f32(values: &[f32]) -> Result<f32> {
    reduce_f32(values, ffi::vDSP_sve)
}

#[must_use]
pub fn hamming_window(length: usize, flags: i32) -> Vec<f32> {
    let mut out = vec![0.0_f32; length];
    if length > 0 {
        // SAFETY: `out` is valid for `length` contiguous `f32` values.
        unsafe { ffi::vDSP_hamm_window(out.as_mut_ptr(), length, flags) };
    }
    out
}

#[must_use]
pub fn blackman_window(length: usize, flags: i32) -> Vec<f32> {
    let mut out = vec![0.0_f32; length];
    if length > 0 {
        // SAFETY: `out` is valid for `length` contiguous `f32` values.
        unsafe { ffi::vDSP_blkman_window(out.as_mut_ptr(), length, flags) };
    }
    out
}
