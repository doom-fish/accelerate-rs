#![allow(missing_docs, non_camel_case_types, non_upper_case_globals)]

use core::ffi::c_void;

pub type BNNSFilter = *mut c_void;
pub type BNNSGraphOptimizationPreference = u32;

#[repr(C)]
pub struct bnns_graph_compile_options_t {
    pub data: *mut c_void,
    pub size: usize,
}

#[link(name = "Accelerate", kind = "framework")]
unsafe extern "C" {
    pub fn BNNSFilterCreateLayerConvolution(
        layer_params: *const c_void,
        filter_params: *const c_void,
    ) -> BNNSFilter;
    pub fn BNNSFilterCreateLayerFullyConnected(
        layer_params: *const c_void,
        filter_params: *const c_void,
    ) -> BNNSFilter;
    pub fn BNNSFilterApply(filter: BNNSFilter, input: *const c_void, output: *mut c_void) -> i32;
    pub fn BNNSFilterDestroy(filter: BNNSFilter);

    pub fn BNNSGraphCompileOptionsMakeDefault() -> bnns_graph_compile_options_t;
    pub fn BNNSGraphCompileOptionsDestroy(options: bnns_graph_compile_options_t);
    pub fn BNNSGraphCompileOptionsSetTargetSingleThread(
        options: bnns_graph_compile_options_t,
        value: bool,
    );
    pub fn BNNSGraphCompileOptionsGetTargetSingleThread(
        options: bnns_graph_compile_options_t,
    ) -> bool;
    pub fn BNNSGraphCompileOptionsSetGenerateDebugInfo(
        options: bnns_graph_compile_options_t,
        value: bool,
    );
    pub fn BNNSGraphCompileOptionsGetGenerateDebugInfo(
        options: bnns_graph_compile_options_t,
    ) -> bool;
    pub fn BNNSGraphCompileOptionsSetOptimizationPreference(
        options: bnns_graph_compile_options_t,
        preference: BNNSGraphOptimizationPreference,
    );
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
