use crate::bridge;
use crate::error::{Error, Result};
use core::ffi::c_void;
use core::ptr;

/// `FFTDirection` constants.
pub mod fft_direction {
    /// `FFTDirection` value for forward transforms passed to `vDSP_fft_zip`.
    pub const FORWARD: i32 = 1;
    /// `FFTDirection` value for inverse transforms passed to `vDSP_fft_zip`.
    pub const INVERSE: i32 = -1;
}

/// `FFTRadix` constants.
pub mod fft_radix {
    /// `FFTRadix` value for radix-2 setups passed to `vDSP_create_fftsetup`.
    pub const RADIX2: i32 = 0;
    /// `FFTRadix` value for radix-3 setups passed to `vDSP_create_fftsetup`.
    pub const RADIX3: i32 = 1;
    /// `FFTRadix` value for radix-5 setups passed to `vDSP_create_fftsetup`.
    pub const RADIX5: i32 = 2;
}

/// Window-generation flags.
pub mod window_flags {
    /// `vDSP_HALF_WINDOW` flag for `vDSP_hamm_window` and `vDSP_blkman_window`.
    pub const HALF_WINDOW: i32 = 1;
    /// `vDSP_HANN_DENORM` flag for `vDSP_hamm_window` and `vDSP_blkman_window`.
    pub const HANN_DENORM: i32 = 0;
    /// `vDSP_HANN_NORM` flag for `vDSP_hamm_window` and `vDSP_blkman_window`.
    pub const HANN_NORM: i32 = 2;
}

/// Owned `FFTSetup` handle backed by the Swift bridge.
pub struct FftSetup {
    ptr: *mut c_void,
}

unsafe impl Send for FftSetup {}
unsafe impl Sync for FftSetup {}

impl Drop for FftSetup {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            // SAFETY: `ptr` is an opaque Swift object retained by the bridge.
            unsafe { bridge::acc_release_handle(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl FftSetup {
    /// Creates an `FFTSetup` with `vDSP_create_fftsetup`.
    #[must_use]
    pub fn new(log2n: usize, radix: i32) -> Option<Self> {
        // SAFETY: Pure constructor over scalar inputs.
        let ptr = unsafe { bridge::acc_vdsp_fft_setup_create(log2n, radix) };
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    /// Wraps `vDSP_fft_zip` for split-complex single-precision buffers.
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

        // SAFETY: Buffers are valid for `expected` elements and `self.ptr` is a live bridge handle.
        let ok = unsafe {
            bridge::acc_vdsp_fft_setup_apply(
                self.ptr,
                real.as_mut_ptr(),
                imag.as_mut_ptr(),
                log2n,
                direction,
            )
        };
        if ok {
            Ok(())
        } else {
            Err(Error::OperationFailed("vDSP FFT operation failed"))
        }
    }
}

/// Owned `vDSP_biquad_Setup` handle backed by the Swift bridge.
pub struct BiquadSetup {
    ptr: *mut c_void,
}

unsafe impl Send for BiquadSetup {}
unsafe impl Sync for BiquadSetup {}

impl Drop for BiquadSetup {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            // SAFETY: `ptr` is an opaque Swift object retained by the bridge.
            unsafe { bridge::acc_release_handle(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl BiquadSetup {
    /// Creates a `vDSP_biquad_Setup` with `vDSP_biquad_CreateSetup`.
    #[must_use]
    pub fn new(coefficients: &[f64]) -> Option<Self> {
        if coefficients.is_empty() || coefficients.len() % 5 != 0 {
            return None;
        }

        // SAFETY: `coefficients` is valid for `count` contiguous `f64` values.
        let ptr = unsafe {
            bridge::acc_vdsp_biquad_setup_create(coefficients.as_ptr(), coefficients.len())
        };
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    /// Wraps `vDSP_biquad` for single-precision input and output buffers.
    pub fn apply(&self, delay: &mut [f32], input: &[f32], output: &mut [f32]) -> Result<()> {
        if delay.is_empty() {
            return Err(Error::InvalidLength {
                expected: 1,
                actual: 0,
            });
        }
        if input.len() != output.len() {
            return Err(Error::InvalidLength {
                expected: input.len(),
                actual: output.len(),
            });
        }

        // SAFETY: Buffers are valid and `self.ptr` is a live bridge handle.
        let ok = unsafe {
            bridge::acc_vdsp_biquad_setup_apply(
                self.ptr,
                delay.as_mut_ptr(),
                input.as_ptr(),
                output.as_mut_ptr(),
                input.len(),
            )
        };
        if ok {
            Ok(())
        } else {
            Err(Error::OperationFailed("vDSP biquad operation failed"))
        }
    }
}

type BinaryVectorOpF32 = unsafe extern "C" fn(*const f32, *const f32, *mut f32, usize) -> bool;
type BinaryVectorOpF64 = unsafe extern "C" fn(*const f64, *const f64, *mut f64, usize) -> bool;
type ReduceOpF32 = unsafe extern "C" fn(*const f32, *mut f32, usize) -> bool;
type ReduceOpF64 = unsafe extern "C" fn(*const f64, *mut f64, usize) -> bool;
type WindowOpF32 = unsafe extern "C" fn(*mut f32, usize, i32) -> bool;
type WindowOpF64 = unsafe extern "C" fn(*mut f64, usize, i32) -> bool;

fn binary_vector_op_f32(a: &[f32], b: &[f32], f: BinaryVectorOpF32) -> Result<Vec<f32>> {
    if a.len() != b.len() {
        return Err(Error::InvalidLength {
            expected: a.len(),
            actual: b.len(),
        });
    }

    let mut out = vec![0.0_f32; a.len()];
    // SAFETY: All slices are valid for `a.len()` contiguous `f32` elements.
    let ok = unsafe { f(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len()) };
    if ok {
        Ok(out)
    } else {
        Err(Error::OperationFailed("vDSP vector operation failed"))
    }
}

fn binary_vector_op_f64(a: &[f64], b: &[f64], f: BinaryVectorOpF64) -> Result<Vec<f64>> {
    if a.len() != b.len() {
        return Err(Error::InvalidLength {
            expected: a.len(),
            actual: b.len(),
        });
    }

    let mut out = vec![0.0_f64; a.len()];
    // SAFETY: All slices are valid for `a.len()` contiguous `f64` elements.
    let ok = unsafe { f(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), a.len()) };
    if ok {
        Ok(out)
    } else {
        Err(Error::OperationFailed("vDSP vector operation failed"))
    }
}

fn reduce_f32(values: &[f32], f: ReduceOpF32) -> Result<f32> {
    if values.is_empty() {
        return Err(Error::InvalidLength {
            expected: 1,
            actual: 0,
        });
    }

    let mut out = 0.0_f32;
    // SAFETY: The slice is valid for `values.len()` contiguous `f32` elements.
    let ok = unsafe { f(values.as_ptr(), &mut out, values.len()) };
    if ok {
        Ok(out)
    } else {
        Err(Error::OperationFailed("vDSP reduction failed"))
    }
}

fn reduce_f64(values: &[f64], f: ReduceOpF64) -> Result<f64> {
    if values.is_empty() {
        return Err(Error::InvalidLength {
            expected: 1,
            actual: 0,
        });
    }

    let mut out = 0.0_f64;
    // SAFETY: The slice is valid for `values.len()` contiguous `f64` elements.
    let ok = unsafe { f(values.as_ptr(), &mut out, values.len()) };
    if ok {
        Ok(out)
    } else {
        Err(Error::OperationFailed("vDSP reduction failed"))
    }
}

#[must_use]
fn window_f32(length: usize, flags: i32, f: WindowOpF32) -> Vec<f32> {
    let mut out = vec![0.0_f32; length];
    if length == 0 {
        return out;
    }

    // SAFETY: `out` is valid for `length` contiguous `f32` values.
    let _ = unsafe { f(out.as_mut_ptr(), length, flags) };
    out
}

#[must_use]
fn window_f64(length: usize, flags: i32, f: WindowOpF64) -> Vec<f64> {
    let mut out = vec![0.0_f64; length];
    if length == 0 {
        return out;
    }

    // SAFETY: `out` is valid for `length` contiguous `f64` values.
    let _ = unsafe { f(out.as_mut_ptr(), length, flags) };
    out
}

/// Wraps `vDSP_vadd`.
pub fn add_f32(a: &[f32], b: &[f32]) -> Result<Vec<f32>> {
    binary_vector_op_f32(a, b, bridge::acc_vdsp_add_f32)
}

/// Wraps `vDSP_vaddD`.
pub fn add_f64(a: &[f64], b: &[f64]) -> Result<Vec<f64>> {
    binary_vector_op_f64(a, b, bridge::acc_vdsp_add_f64)
}

/// Wraps `vDSP_vsub`.
pub fn sub_f32(a: &[f32], b: &[f32]) -> Result<Vec<f32>> {
    binary_vector_op_f32(a, b, bridge::acc_vdsp_sub_f32)
}

/// Wraps `vDSP_vsubD`.
pub fn sub_f64(a: &[f64], b: &[f64]) -> Result<Vec<f64>> {
    binary_vector_op_f64(a, b, bridge::acc_vdsp_sub_f64)
}

/// Wraps `vDSP_dotpr`.
pub fn dot_f32(a: &[f32], b: &[f32]) -> Result<f32> {
    if a.len() != b.len() {
        return Err(Error::InvalidLength {
            expected: a.len(),
            actual: b.len(),
        });
    }

    let mut out = 0.0_f32;
    // SAFETY: The slices are valid for `a.len()` contiguous `f32` elements.
    let ok = unsafe { bridge::acc_vdsp_dot_f32(a.as_ptr(), b.as_ptr(), &mut out, a.len()) };
    if ok {
        Ok(out)
    } else {
        Err(Error::OperationFailed("vDSP dot-product failed"))
    }
}

/// Wraps `vDSP_dotprD`.
pub fn dot_f64(a: &[f64], b: &[f64]) -> Result<f64> {
    if a.len() != b.len() {
        return Err(Error::InvalidLength {
            expected: a.len(),
            actual: b.len(),
        });
    }

    let mut out = 0.0_f64;
    // SAFETY: The slices are valid for `a.len()` contiguous `f64` elements.
    let ok = unsafe { bridge::acc_vdsp_dot_f64(a.as_ptr(), b.as_ptr(), &mut out, a.len()) };
    if ok {
        Ok(out)
    } else {
        Err(Error::OperationFailed("vDSP dot-product failed"))
    }
}

/// Wraps `vDSP_maxv`.
pub fn max_f32(values: &[f32]) -> Result<f32> {
    reduce_f32(values, bridge::acc_vdsp_max_f32)
}

/// Wraps `vDSP_maxvD`.
pub fn max_f64(values: &[f64]) -> Result<f64> {
    reduce_f64(values, bridge::acc_vdsp_max_f64)
}

/// Wraps `vDSP_minv`.
pub fn min_f32(values: &[f32]) -> Result<f32> {
    reduce_f32(values, bridge::acc_vdsp_min_f32)
}

/// Wraps `vDSP_minvD`.
pub fn min_f64(values: &[f64]) -> Result<f64> {
    reduce_f64(values, bridge::acc_vdsp_min_f64)
}

/// Wraps `vDSP_meanv`.
pub fn mean_f32(values: &[f32]) -> Result<f32> {
    reduce_f32(values, bridge::acc_vdsp_mean_f32)
}

/// Wraps `vDSP_meanvD`.
pub fn mean_f64(values: &[f64]) -> Result<f64> {
    reduce_f64(values, bridge::acc_vdsp_mean_f64)
}

/// Wraps `vDSP_sve`.
pub fn sum_f32(values: &[f32]) -> Result<f32> {
    reduce_f32(values, bridge::acc_vdsp_sum_f32)
}

/// Wraps `vDSP_sveD`.
pub fn sum_f64(values: &[f64]) -> Result<f64> {
    reduce_f64(values, bridge::acc_vdsp_sum_f64)
}

/// Wraps `vDSP_hamm_window`.
#[must_use]
pub fn hamming_window(length: usize, flags: i32) -> Vec<f32> {
    window_f32(length, flags, bridge::acc_vdsp_hamming_window)
}

/// Wraps `vDSP_hamm_windowD`.
#[must_use]
pub fn hamming_window_f64(length: usize, flags: i32) -> Vec<f64> {
    window_f64(length, flags, bridge::acc_vdsp_hamming_window_f64)
}

/// Wraps `vDSP_blkman_window`.
#[must_use]
pub fn blackman_window(length: usize, flags: i32) -> Vec<f32> {
    window_f32(length, flags, bridge::acc_vdsp_blackman_window)
}

/// Wraps `vDSP_blkman_windowD`.
#[must_use]
pub fn blackman_window_f64(length: usize, flags: i32) -> Vec<f64> {
    window_f64(length, flags, bridge::acc_vdsp_blackman_window_f64)
}
