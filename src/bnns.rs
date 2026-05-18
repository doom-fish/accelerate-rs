use crate::bridge;
use crate::error::{Error, Result};
use crate::raw_ffi;
use core::ffi::c_void;
use core::ptr;

/// `BNNSGraphOptimizationPreference` constants.
pub mod graph_optimization_preference {
    /// `BNNSGraphOptimizationPreference` value that favors runtime performance.
    pub const PERFORMANCE: u32 = 0;
    /// `BNNSGraphOptimizationPreference` value that favors smaller BNNS IR size.
    pub const IR_SIZE: u32 = 1;
}

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
        // SAFETY: Caller has verified that `layer_params` and `filter_params` are valid BNNS structs.
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
        // SAFETY: Caller has verified that `layer_params` and `filter_params` are valid BNNS structs.
        let ptr =
            unsafe { raw_ffi::BNNSFilterCreateLayerFullyConnected(layer_params, filter_params) };
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    /// Returns the raw `BNNSFilter` pointer wrapped by this owner.
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
        // SAFETY: Caller has verified that `input` and `output` satisfy the filter's documented preconditions.
        unsafe { raw_ffi::BNNSFilterApply(self.ptr, input, output) }
    }
}

/// Owned BNNS Graph compile-options handle backed by the Swift bridge.
pub struct GraphCompileOptions {
    ptr: *mut c_void,
}

unsafe impl Send for GraphCompileOptions {}
unsafe impl Sync for GraphCompileOptions {}

impl Drop for GraphCompileOptions {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            // SAFETY: `ptr` is an opaque Swift object retained by the bridge.
            unsafe { bridge::acc_release_handle(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

fn activation_result(status: i32) -> Result<()> {
    if status == 0 {
        Ok(())
    } else {
        Err(Error::BnnsStatus(status))
    }
}

fn graph_result(ok: bool) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(Error::OperationFailed(
            "BNNS Graph compile options are unavailable on this macOS version",
        ))
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

impl GraphCompileOptions {
    /// Creates BNNS graph compile options backed by the BNNS graph compile-options API.
    #[must_use]
    pub fn new() -> Option<Self> {
        // SAFETY: Pure constructor over the current runtime environment.
        let ptr = unsafe { bridge::acc_bnns_graph_compile_options_create() };
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    /// Sets the BNNS graph compile option that targets single-thread execution.
    pub fn set_target_single_thread(&mut self, value: bool) -> Result<()> {
        // SAFETY: `self.ptr` is a live bridge handle.
        graph_result(unsafe {
            bridge::acc_bnns_graph_compile_options_set_target_single_thread(self.ptr, value)
        })
    }

    /// Returns whether BNNS graph compilation targets single-thread execution.
    #[must_use]
    pub fn target_single_thread(&self) -> bool {
        // SAFETY: `self.ptr` is a live bridge handle.
        unsafe { bridge::acc_bnns_graph_compile_options_get_target_single_thread(self.ptr) }
    }

    /// Sets the BNNS graph compile option that emits debug information.
    pub fn set_generate_debug_info(&mut self, value: bool) -> Result<()> {
        // SAFETY: `self.ptr` is a live bridge handle.
        graph_result(unsafe {
            bridge::acc_bnns_graph_compile_options_set_generate_debug_info(self.ptr, value)
        })
    }

    /// Returns whether the BNNS graph compile option emits debug information.
    #[must_use]
    pub fn generate_debug_info(&self) -> bool {
        // SAFETY: `self.ptr` is a live bridge handle.
        unsafe { bridge::acc_bnns_graph_compile_options_get_generate_debug_info(self.ptr) }
    }

    /// Sets the `BNNSGraphOptimizationPreference` used by BNNS graph compilation.
    pub fn set_optimization_preference(&mut self, preference: u32) -> Result<()> {
        // SAFETY: `self.ptr` is a live bridge handle.
        graph_result(unsafe {
            bridge::acc_bnns_graph_compile_options_set_optimization_preference(self.ptr, preference)
        })
    }

    /// Returns the `BNNSGraphOptimizationPreference` used by BNNS graph compilation.
    #[must_use]
    pub fn optimization_preference(&self) -> u32 {
        // SAFETY: `self.ptr` is a live bridge handle.
        unsafe { bridge::acc_bnns_graph_compile_options_get_optimization_preference(self.ptr) }
    }
}

/// Apply BNNS `ReLU` activation to a vector.
pub fn relu_f32(values: &[f32]) -> Result<Vec<f32>> {
    apply_activation(values, bridge::acc_bnns_relu_f32)
}

/// Apply BNNS sigmoid activation to a vector.
pub fn sigmoid_f32(values: &[f32]) -> Result<Vec<f32>> {
    apply_activation(values, bridge::acc_bnns_sigmoid_f32)
}
