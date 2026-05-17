#![allow(missing_docs, non_camel_case_types, non_upper_case_globals)]

use core::ffi::c_void;

pub type vImagePixelCount = u64;
pub type vImage_Error = isize;
pub type vImage_Flags = u32;

#[repr(C)]
pub struct vImage_Buffer {
    pub data: *mut c_void,
    pub height: vImagePixelCount,
    pub width: vImagePixelCount,
    pub row_bytes: usize,
}

#[link(name = "Accelerate", kind = "framework")]
unsafe extern "C" {
    pub fn vImageRotate_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        temp_buffer: *mut c_void,
        angle_in_radians: f32,
        background_color: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
    pub fn vImageBoxConvolve_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        temp_buffer: *mut c_void,
        src_offset_to_roi_x: vImagePixelCount,
        src_offset_to_roi_y: vImagePixelCount,
        kernel_height: u32,
        kernel_width: u32,
        background_color: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
    pub fn vImageScale_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        temp_buffer: *mut c_void,
        flags: vImage_Flags,
    ) -> vImage_Error;
    pub fn vImageContrastStretch_Planar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
    pub fn vImageAlphaBlend_ARGB8888(
        src_top: *const vImage_Buffer,
        src_bottom: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
    pub fn vImageClipToAlpha_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
    pub fn vImagePremultiplyData_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
    pub fn vImageUnpremultiplyData_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
    pub fn vImageConvert_Planar8toARGB8888(
        src_a: *const vImage_Buffer,
        src_r: *const vImage_Buffer,
        src_g: *const vImage_Buffer,
        src_b: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
    pub fn vImageConvert_ARGB8888toPlanar8(
        src_argb: *const vImage_Buffer,
        dest_a: *const vImage_Buffer,
        dest_r: *const vImage_Buffer,
        dest_g: *const vImage_Buffer,
        dest_b: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
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
    include!("generated/vimage_missing.rs");
}

pub use generated::*;
