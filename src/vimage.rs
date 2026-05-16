use crate::error::{Error, Result};
use crate::ffi;
use core::marker::PhantomData;
use core::ptr;

/// Common `vImage_Flags` values.
pub mod vimage_flags {
    pub const NO_FLAGS: u32 = 0;
    pub const BACKGROUND_COLOR_FILL: u32 = 4;
    pub const EDGE_EXTEND: u32 = 8;
    pub const HIGH_QUALITY_RESAMPLING: u32 = 32;
}

fn pixel_count(value: usize) -> Result<u64> {
    u64::try_from(value).map_err(|_| Error::OperationFailed("image dimension exceeds u64"))
}

fn vimage_result(status: ffi::vImage_Error) -> Result<()> {
    if status == 0 {
        Ok(())
    } else {
        Err(Error::VImageError(status))
    }
}

/// Borrowed wrapper around a caller-owned `vImage_Buffer`.
pub struct ImageBuffer<'a> {
    inner: ffi::vImage_Buffer,
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
            inner: ffi::vImage_Buffer {
                data: data.as_mut_ptr().cast(),
                height: pixel_count(height)?,
                width: pixel_count(width)?,
                row_bytes: width * 4,
            },
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
            inner: ffi::vImage_Buffer {
                data: data.as_mut_ptr().cast(),
                height: pixel_count(height)?,
                width: pixel_count(width)?,
                row_bytes: width,
            },
            _marker: PhantomData,
        })
    }

    #[must_use]
    pub const fn as_ptr(&self) -> *const ffi::vImage_Buffer {
        &self.inner
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
        ffi::vImageRotate_ARGB8888(
            src.as_ptr(),
            dst.as_ptr(),
            ptr::null_mut(),
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
        ffi::vImageBoxConvolve_ARGB8888(
            src.as_ptr(),
            dst.as_ptr(),
            ptr::null_mut(),
            0,
            0,
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
    let status =
        unsafe { ffi::vImageScale_ARGB8888(src.as_ptr(), dst.as_ptr(), ptr::null_mut(), flags) };
    vimage_result(status)
}

pub fn contrast_stretch_planar8(
    src: &ImageBuffer<'_>,
    dst: &mut ImageBuffer<'_>,
    flags: u32,
) -> Result<()> {
    // SAFETY: Source and destination buffers remain valid for the duration of the call.
    let status = unsafe { ffi::vImageContrastStretch_Planar8(src.as_ptr(), dst.as_ptr(), flags) };
    vimage_result(status)
}
