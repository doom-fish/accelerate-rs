#![allow(missing_docs, non_camel_case_types, non_upper_case_globals)]

use core::ffi::c_void;

/// Raw FFI type alias for `BNNSFilter`.
pub type BNNSFilter = *mut c_void;
/// Raw FFI type alias for `BNNSGraphOptimizationPreference`.
pub type BNNSGraphOptimizationPreference = u32;

/// Raw FFI struct for `bnns_graph_compile_options_t`.
#[repr(C)]
pub struct bnns_graph_compile_options_t {
    pub data: *mut c_void,
    pub size: usize,
}

#[link(name = "Accelerate", kind = "framework")]
unsafe extern "C" {
    /// Raw FFI declaration for `BNNSFilterCreateLayerConvolution`.
    pub fn BNNSFilterCreateLayerConvolution(
        layer_params: *const c_void,
        filter_params: *const c_void,
    ) -> BNNSFilter;
    /// Raw FFI declaration for `BNNSFilterCreateLayerFullyConnected`.
    pub fn BNNSFilterCreateLayerFullyConnected(
        layer_params: *const c_void,
        filter_params: *const c_void,
    ) -> BNNSFilter;
    /// Raw FFI declaration for `BNNSFilterApply`.
    pub fn BNNSFilterApply(filter: BNNSFilter, input: *const c_void, output: *mut c_void) -> i32;
    /// Raw FFI declaration for `BNNSFilterDestroy`.
    pub fn BNNSFilterDestroy(filter: BNNSFilter);

    /// Raw FFI declaration for `BNNSGraphCompileOptionsMakeDefault`.
    pub fn BNNSGraphCompileOptionsMakeDefault() -> bnns_graph_compile_options_t;
    /// Raw FFI declaration for `BNNSGraphCompileOptionsDestroy`.
    pub fn BNNSGraphCompileOptionsDestroy(options: bnns_graph_compile_options_t);
    /// Raw FFI declaration for `BNNSGraphCompileOptionsSetTargetSingleThread`.
    pub fn BNNSGraphCompileOptionsSetTargetSingleThread(
        options: bnns_graph_compile_options_t,
        value: bool,
    );
    /// Raw FFI declaration for `BNNSGraphCompileOptionsGetTargetSingleThread`.
    pub fn BNNSGraphCompileOptionsGetTargetSingleThread(
        options: bnns_graph_compile_options_t,
    ) -> bool;
    /// Raw FFI declaration for `BNNSGraphCompileOptionsSetGenerateDebugInfo`.
    pub fn BNNSGraphCompileOptionsSetGenerateDebugInfo(
        options: bnns_graph_compile_options_t,
        value: bool,
    );
    /// Raw FFI declaration for `BNNSGraphCompileOptionsGetGenerateDebugInfo`.
    pub fn BNNSGraphCompileOptionsGetGenerateDebugInfo(
        options: bnns_graph_compile_options_t,
    ) -> bool;
    /// Raw FFI declaration for `BNNSGraphCompileOptionsSetOptimizationPreference`.
    pub fn BNNSGraphCompileOptionsSetOptimizationPreference(
        options: bnns_graph_compile_options_t,
        preference: BNNSGraphOptimizationPreference,
    );
    /// Raw FFI declaration for `BNNSGraphCompileOptionsGetOptimizationPreference`.
    pub fn BNNSGraphCompileOptionsGetOptimizationPreference(
        options: bnns_graph_compile_options_t,
    ) -> BNNSGraphOptimizationPreference;
}

#[allow(
    missing_docs,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_imports,
    dead_code,
    improper_ctypes,
    improper_ctypes_definitions,
    unnecessary_transmutes
)]
#[allow(clippy::all, clippy::pedantic, clippy::nursery, clippy::style, clippy::complexity, clippy::correctness, clippy::perf)]
mod generated {
    #![allow(clippy::all, clippy::pedantic, clippy::nursery, clippy::style, clippy::complexity, clippy::correctness, clippy::perf)]
    use super::*;
    include!("generated/bnns_missing.rs");
}

pub use generated::*;
