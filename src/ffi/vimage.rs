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
}
