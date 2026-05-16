#![allow(missing_docs, non_camel_case_types, non_upper_case_globals)]

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct simd_float4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
