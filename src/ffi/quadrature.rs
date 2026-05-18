#![allow(missing_docs, non_camel_case_types, non_upper_case_globals)]

use core::ffi::c_void;

/// Raw FFI type alias for `quadrature_function_array`.
pub type quadrature_function_array =
    Option<unsafe extern "C" fn(*mut c_void, usize, *const f64, *mut f64)>;

/// Raw FFI enum for `quadrature_status`.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum quadrature_status {
    QUADRATURE_SUCCESS = 0,
    QUADRATURE_ERROR = -1,
    QUADRATURE_INVALID_ARG_ERROR = -2,
    QUADRATURE_ALLOC_ERROR = -3,
    QUADRATURE_INTERNAL_ERROR = -99,
    QUADRATURE_INTEGRATE_MAX_EVAL_ERROR = -101,
    QUADRATURE_INTEGRATE_BAD_BEHAVIOUR_ERROR = -102,
}

/// Raw FFI enum for `quadrature_integrator`.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum quadrature_integrator {
    QUADRATURE_INTEGRATE_QNG = 0,
    QUADRATURE_INTEGRATE_QAG = 1,
    QUADRATURE_INTEGRATE_QAGS = 2,
}

/// Raw FFI struct for `quadrature_integrate_function`.
#[repr(C)]
pub struct quadrature_integrate_function {
    pub fun: quadrature_function_array,
    pub fun_arg: *mut c_void,
}

/// Raw FFI struct for `quadrature_integrate_options`.
#[repr(C)]
pub struct quadrature_integrate_options {
    pub integrator: quadrature_integrator,
    pub abs_tolerance: f64,
    pub rel_tolerance: f64,
    pub qag_points_per_interval: usize,
    pub max_intervals: usize,
}

#[link(name = "Accelerate", kind = "framework")]
unsafe extern "C" {
    /// Raw FFI declaration for `quadrature_integrate`.
    pub fn quadrature_integrate(
        f: *const quadrature_integrate_function,
        a: f64,
        b: f64,
        options: *const quadrature_integrate_options,
        status: *mut quadrature_status,
        abs_error: *mut f64,
        workspace_size: usize,
        workspace: *mut c_void,
    ) -> f64;
}
