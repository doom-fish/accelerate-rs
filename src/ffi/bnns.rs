#![allow(missing_docs, non_camel_case_types, non_upper_case_globals)]

use core::ffi::c_void;

pub type BNNSFilter = *mut c_void;

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
}
