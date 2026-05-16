use crate::bridge;
use crate::error::{Error, Result};
use crate::raw_ffi;
use core::ffi::c_void;
use core::ptr;

/// Thin owner for the deprecated-but-still-available BNNS filter APIs.
pub struct Filter {
    ptr: raw_ffi::BNNSFilter,
}

unsafe impl Send for Filter {}
unsafe impl Sync for Filter {}

impl Drop for Filter {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            // SAFETY: `ptr` was returned by a BNNS filter constructor and is owned by this wrapper.
            unsafe { raw_ffi::BNNSFilterDestroy(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl Filter {
    /// Create a BNNS convolution filter from caller-owned raw layer/filter parameter structs.
    ///
    /// # Safety
    ///
    /// The pointers must refer to valid BNNS parameter structs for the duration of the call.
    #[must_use]
    pub unsafe fn from_convolution(
        layer_params: *const c_void,
        filter_params: *const c_void,
    ) -> Option<Self> {
        let ptr = unsafe { raw_ffi::BNNSFilterCreateLayerConvolution(layer_params, filter_params) };
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    /// Create a BNNS fully connected filter from caller-owned raw layer/filter parameter structs.
    ///
    /// # Safety
    ///
    /// The pointers must refer to valid BNNS parameter structs for the duration of the call.
    #[must_use]
    pub unsafe fn from_fully_connected(
        layer_params: *const c_void,
        filter_params: *const c_void,
    ) -> Option<Self> {
        let ptr =
            unsafe { raw_ffi::BNNSFilterCreateLayerFullyConnected(layer_params, filter_params) };
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

    /// Apply the filter to caller-owned raw buffers.
    ///
    /// # Safety
    ///
    /// `input` and `output` must match the layout and lengths described when the filter was created.
    pub unsafe fn apply(&self, input: *const c_void, output: *mut c_void) -> i32 {
        unsafe { raw_ffi::BNNSFilterApply(self.ptr, input, output) }
    }
}

fn activation_result(status: i32) -> Result<()> {
    if status == 0 {
        Ok(())
    } else {
        Err(Error::BnnsStatus(status))
    }
}

fn apply_activation(
    values: &[f32],
    f: unsafe extern "C" fn(*const f32, *mut f32, usize) -> i32,
) -> Result<Vec<f32>> {
    let mut out = vec![0.0_f32; values.len()];
    if values.is_empty() {
        return Ok(out);
    }

    // SAFETY: The buffers are valid for `values.len()` contiguous `f32` elements.
    let status = unsafe { f(values.as_ptr(), out.as_mut_ptr(), values.len()) };
    activation_result(status)?;
    Ok(out)
}

/// Apply BNNS `ReLU` activation to a vector.
pub fn relu_f32(values: &[f32]) -> Result<Vec<f32>> {
    apply_activation(values, bridge::acc_bnns_relu_f32)
}

/// Apply BNNS sigmoid activation to a vector.
pub fn sigmoid_f32(values: &[f32]) -> Result<Vec<f32>> {
    apply_activation(values, bridge::acc_bnns_sigmoid_f32)
}
