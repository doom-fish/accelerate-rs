#![allow(missing_docs, non_camel_case_types, non_upper_case_globals)]

#[link(name = "Accelerate", kind = "framework")]
unsafe extern "C" {
    pub fn vvsinf(y: *mut f32, x: *const f32, n: *const i32);
    pub fn vvcosf(y: *mut f32, x: *const f32, n: *const i32);
    pub fn vvexpf(y: *mut f32, x: *const f32, n: *const i32);
    pub fn vvlogf(y: *mut f32, x: *const f32, n: *const i32);
    pub fn vvsqrtf(y: *mut f32, x: *const f32, n: *const i32);
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
mod generated {
    use super::*;
    include!("generated/vforce_missing.rs");
}

pub use generated::*;
