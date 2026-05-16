use crate::bridge;
use crate::error::{Error, Result};
use core::ffi::c_void;
use core::marker::PhantomData;

/// Common `vImage_Flags` values.
pub mod vimage_flags {
    pub const NO_FLAGS: u32 = 0;
    pub const BACKGROUND_COLOR_FILL: u32 = 4;
    pub const EDGE_EXTEND: u32 = 8;
    pub const HIGH_QUALITY_RESAMPLING: u32 = 32;
}

fn vimage_result(status: isize) -> Result<()> {
    if status == 0 {
        Ok(())
    } else {
        Err(Error::VImageError(status))
    }
}

/// Borrowed wrapper around a caller-owned image buffer.
pub struct ImageBuffer<'a> {
    data: *mut u8,
    width: usize,
    height: usize,
    row_bytes: usize,
    _marker: PhantomData<&'a mut [u8]>,
}

impl<'a> ImageBuffer<'a> {
    pub fn from_argb8888(data: &'a mut [u8], width: usize, height: usize) -> Result<Self> {
        let expected = width
            .checked_mul(height)
            .and_then(|pixels| pixels.checked_mul(4))
            .ok_or(Error::OperationFailed("image dimensions overflowed"))?;
        if data.len() < expected {
            return Err(Error::InvalidLength {
                expected,
                actual: data.len(),
            });
        }

        Ok(Self {
            data: data.as_mut_ptr(),
            width,
            height,
            row_bytes: width * 4,
            _marker: PhantomData,
        })
    }

    pub fn from_planar8(data: &'a mut [u8], width: usize, height: usize) -> Result<Self> {
        let expected = width
            .checked_mul(height)
            .ok_or(Error::OperationFailed("image dimensions overflowed"))?;
        if data.len() < expected {
            return Err(Error::InvalidLength {
                expected,
                actual: data.len(),
            });
        }

        Ok(Self {
            data: data.as_mut_ptr(),
            width,
            height,
            row_bytes: width,
            _marker: PhantomData,
        })
    }

    fn data_ptr(&self) -> *mut c_void {
        self.data.cast()
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn row_bytes(&self) -> usize {
        self.row_bytes
    }
}

pub fn rotate_argb8888(
    src: &ImageBuffer<'_>,
    dst: &mut ImageBuffer<'_>,
    angle_radians: f32,
    background_color: [u8; 4],
    flags: u32,
) -> Result<()> {
    // SAFETY: Source and destination buffers remain valid for the duration of the call.
    let status = unsafe {
        bridge::acc_vimage_rotate_argb8888(
            src.data_ptr(),
            src.width(),
            src.height(),
            src.row_bytes(),
            dst.data_ptr(),
            dst.width(),
            dst.height(),
            dst.row_bytes(),
            angle_radians,
            background_color.as_ptr(),
            flags,
        )
    };
    vimage_result(status)
}

pub fn box_convolve_argb8888(
    src: &ImageBuffer<'_>,
    dst: &mut ImageBuffer<'_>,
    kernel_height: u32,
    kernel_width: u32,
    background_color: [u8; 4],
    flags: u32,
) -> Result<()> {
    // SAFETY: Source and destination buffers remain valid for the duration of the call.
    let status = unsafe {
        bridge::acc_vimage_box_convolve_argb8888(
            src.data_ptr(),
            src.width(),
            src.height(),
            src.row_bytes(),
            dst.data_ptr(),
            dst.width(),
            dst.height(),
            dst.row_bytes(),
            kernel_height,
            kernel_width,
            background_color.as_ptr(),
            flags,
        )
    };
    vimage_result(status)
}

pub fn scale_argb8888(src: &ImageBuffer<'_>, dst: &mut ImageBuffer<'_>, flags: u32) -> Result<()> {
    // SAFETY: Source and destination buffers remain valid for the duration of the call.
    let status = unsafe {
        bridge::acc_vimage_scale_argb8888(
            src.data_ptr(),
            src.width(),
            src.height(),
            src.row_bytes(),
            dst.data_ptr(),
            dst.width(),
            dst.height(),
            dst.row_bytes(),
            flags,
        )
    };
    vimage_result(status)
}

pub fn contrast_stretch_planar8(
    src: &ImageBuffer<'_>,
    dst: &mut ImageBuffer<'_>,
    flags: u32,
) -> Result<()> {
    // SAFETY: Source and destination buffers remain valid for the duration of the call.
    let status = unsafe {
        bridge::acc_vimage_contrast_stretch_planar8(
            src.data_ptr(),
            src.width(),
            src.height(),
            src.row_bytes(),
            dst.data_ptr(),
            dst.width(),
            dst.height(),
            dst.row_bytes(),
            flags,
        )
    };
    vimage_result(status)
}
