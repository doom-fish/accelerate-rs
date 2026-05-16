use crate::ffi;
use core::ffi::c_void;
use core::ptr;

/// Thin owner for the deprecated-but-still-available BNNS filter APIs.
pub struct Filter {
    ptr: ffi::BNNSFilter,
}

unsafe impl Send for Filter {}
unsafe impl Sync for Filter {}

impl Drop for Filter {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            // SAFETY: `ptr` was returned by a BNNS filter constructor and is owned by this wrapper.
            unsafe { ffi::BNNSFilterDestroy(self.ptr) };
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
        let ptr = ffi::BNNSFilterCreateLayerConvolution(layer_params, filter_params);
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
        let ptr = ffi::BNNSFilterCreateLayerFullyConnected(layer_params, filter_params);
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
        ffi::BNNSFilterApply(self.ptr, input, output)
    }
}
