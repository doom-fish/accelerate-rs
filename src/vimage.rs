use crate::bridge;
use crate::error::{Error, Result};
use core::ffi::c_void;
use core::marker::PhantomData;

/// Common `vImage_Flags` values.
pub mod vimage_flags {
    /// `vImage_Flags` value for default behavior.
    pub const NO_FLAGS: u32 = 0;
    /// `vImage_Flags` value that fills uncovered pixels from the background color.
    pub const BACKGROUND_COLOR_FILL: u32 = 4;
    /// `vImage_Flags` value that extends edge pixels beyond the source bounds.
    pub const EDGE_EXTEND: u32 = 8;
    /// `vImage_Flags` value that enables higher-quality resampling.
    pub const HIGH_QUALITY_RESAMPLING: u32 = 32;
}

fn vimage_result(status: isize) -> Result<()> {
    if status == 0 {
        Ok(())
    } else {
        Err(Error::VImageError(status))
    }
}

fn ensure_same_dimensions(lhs: &ImageBuffer<'_>, rhs: &ImageBuffer<'_>) -> Result<()> {
    if lhs.width() == rhs.width() && lhs.height() == rhs.height() {
        Ok(())
    } else {
        Err(Error::InvalidValue(
            "vImage buffers must share the same width and height",
        ))
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
    /// Borrows caller-owned ARGB8888 storage as a `vImage_Buffer`.
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

    /// Borrows caller-owned Planar8 storage as a `vImage_Buffer`.
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

/// Wraps `vImageRotate_ARGB8888`.
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

/// Wraps `vImageBoxConvolve_ARGB8888`.
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

/// Wraps `vImageScale_ARGB8888`.
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

/// Wraps `vImageContrastStretch_Planar8`.
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

/// Wraps `vImageAlphaBlend_ARGB8888`.
pub fn alpha_blend_argb8888(
    src_top: &ImageBuffer<'_>,
    src_bottom: &ImageBuffer<'_>,
    dst: &mut ImageBuffer<'_>,
    flags: u32,
) -> Result<()> {
    ensure_same_dimensions(src_top, src_bottom)?;
    ensure_same_dimensions(src_top, dst)?;

    // SAFETY: Buffers share a common geometry and remain valid for the duration of the call.
    let status = unsafe {
        bridge::acc_vimage_alpha_blend_argb8888(
            src_top.data_ptr(),
            src_top.width(),
            src_top.height(),
            src_top.row_bytes(),
            src_bottom.data_ptr(),
            src_bottom.width(),
            src_bottom.height(),
            src_bottom.row_bytes(),
            dst.data_ptr(),
            dst.width(),
            dst.height(),
            dst.row_bytes(),
            flags,
        )
    };
    vimage_result(status)
}

/// Wraps `vImageClipToAlpha_ARGB8888`.
pub fn clip_to_alpha_argb8888(
    src: &ImageBuffer<'_>,
    dst: &mut ImageBuffer<'_>,
    flags: u32,
) -> Result<()> {
    ensure_same_dimensions(src, dst)?;

    // SAFETY: Buffers share a common geometry and remain valid for the duration of the call.
    let status = unsafe {
        bridge::acc_vimage_clip_to_alpha_argb8888(
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

/// Wraps `vImagePremultiplyData_ARGB8888`.
pub fn premultiply_argb8888(
    src: &ImageBuffer<'_>,
    dst: &mut ImageBuffer<'_>,
    flags: u32,
) -> Result<()> {
    ensure_same_dimensions(src, dst)?;

    // SAFETY: Buffers share a common geometry and remain valid for the duration of the call.
    let status = unsafe {
        bridge::acc_vimage_premultiply_argb8888(
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

/// Wraps `vImageUnpremultiplyData_ARGB8888`.
pub fn unpremultiply_argb8888(
    src: &ImageBuffer<'_>,
    dst: &mut ImageBuffer<'_>,
    flags: u32,
) -> Result<()> {
    ensure_same_dimensions(src, dst)?;

    // SAFETY: Buffers share a common geometry and remain valid for the duration of the call.
    let status = unsafe {
        bridge::acc_vimage_unpremultiply_argb8888(
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

/// Wraps `vImageConvert_Planar8toARGB8888`.
pub fn convert_planar8_to_argb8888(
    src_alpha: &ImageBuffer<'_>,
    src_red: &ImageBuffer<'_>,
    src_green: &ImageBuffer<'_>,
    src_blue: &ImageBuffer<'_>,
    dst: &mut ImageBuffer<'_>,
    flags: u32,
) -> Result<()> {
    ensure_same_dimensions(src_alpha, src_red)?;
    ensure_same_dimensions(src_alpha, src_green)?;
    ensure_same_dimensions(src_alpha, src_blue)?;
    ensure_same_dimensions(src_alpha, dst)?;

    // SAFETY: Buffers share a common geometry and remain valid for the duration of the call.
    let status = unsafe {
        bridge::acc_vimage_convert_planar8_to_argb8888(
            src_alpha.data_ptr(),
            src_alpha.width(),
            src_alpha.height(),
            src_alpha.row_bytes(),
            src_red.data_ptr(),
            src_red.width(),
            src_red.height(),
            src_red.row_bytes(),
            src_green.data_ptr(),
            src_green.width(),
            src_green.height(),
            src_green.row_bytes(),
            src_blue.data_ptr(),
            src_blue.width(),
            src_blue.height(),
            src_blue.row_bytes(),
            dst.data_ptr(),
            dst.width(),
            dst.height(),
            dst.row_bytes(),
            flags,
        )
    };
    vimage_result(status)
}

/// Wraps `vImageConvert_ARGB8888toPlanar8`.
pub fn convert_argb8888_to_planar8(
    src: &ImageBuffer<'_>,
    dst_alpha: &mut ImageBuffer<'_>,
    dst_red: &mut ImageBuffer<'_>,
    dst_green: &mut ImageBuffer<'_>,
    dst_blue: &mut ImageBuffer<'_>,
    flags: u32,
) -> Result<()> {
    ensure_same_dimensions(src, dst_alpha)?;
    ensure_same_dimensions(src, dst_red)?;
    ensure_same_dimensions(src, dst_green)?;
    ensure_same_dimensions(src, dst_blue)?;

    // SAFETY: Buffers share a common geometry and remain valid for the duration of the call.
    let status = unsafe {
        bridge::acc_vimage_convert_argb8888_to_planar8(
            src.data_ptr(),
            src.width(),
            src.height(),
            src.row_bytes(),
            dst_alpha.data_ptr(),
            dst_alpha.width(),
            dst_alpha.height(),
            dst_alpha.row_bytes(),
            dst_red.data_ptr(),
            dst_red.width(),
            dst_red.height(),
            dst_red.row_bytes(),
            dst_green.data_ptr(),
            dst_green.width(),
            dst_green.height(),
            dst_green.row_bytes(),
            dst_blue.data_ptr(),
            dst_blue.width(),
            dst_blue.height(),
            dst_blue.row_bytes(),
            flags,
        )
    };
    vimage_result(status)
}
