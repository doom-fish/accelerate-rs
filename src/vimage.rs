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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PixelFormat {
    Planar8,
    PlanarF,
    Argb8888,
    Rgba8888,
    Bgra8888,
    ArgbFFFF,
    RgbaFFFF,
    BgraFFFF,
}

impl PixelFormat {
    pub const fn bytes_per_pixel(self) -> usize {
        match self {
            Self::Planar8 => 1,
            Self::PlanarF | Self::Argb8888 | Self::Rgba8888 | Self::Bgra8888 => 4,
            Self::ArgbFFFF | Self::RgbaFFFF | Self::BgraFFFF => 16,
        }
    }

    pub const fn is_float(self) -> bool {
        matches!(
            self,
            Self::PlanarF | Self::ArgbFFFF | Self::RgbaFFFF | Self::BgraFFFF
        )
    }
}

const ARGB8888: &[PixelFormat] = &[PixelFormat::Argb8888];
const PLANAR8: &[PixelFormat] = &[PixelFormat::Planar8];
const INTERLEAVED_8888: &[PixelFormat] = &[
    PixelFormat::Argb8888,
    PixelFormat::Rgba8888,
    PixelFormat::Bgra8888,
];

fn packed_row_bytes(format: PixelFormat, width: usize) -> Result<usize> {
    width
        .checked_mul(format.bytes_per_pixel())
        .ok_or(Error::OperationFailed("image dimensions overflowed"))
}

fn expect_format(buffer: &ImageBuffer<'_>, accepted: &[PixelFormat]) -> Result<()> {
    if !accepted.contains(&buffer.format) {
        return Err(Error::InvalidValue(
            "vImage buffer pixel format does not match the operation",
        ));
    }
    if buffer.row_bytes < packed_row_bytes(buffer.format, buffer.width)? {
        return Err(Error::InvalidValue(
            "vImage row bytes must be at least width times bytes per pixel",
        ));
    }
    Ok(())
}

fn ensure_same_extent(lhs: &ImageBuffer<'_>, rhs: &ImageBuffer<'_>) -> Result<()> {
    if lhs.width == rhs.width && lhs.height == rhs.height {
        Ok(())
    } else {
        Err(Error::InvalidValue(
            "vImage buffers must share the same width and height",
        ))
    }
}

fn ensure_same_dimensions(lhs: &ImageBuffer<'_>, rhs: &ImageBuffer<'_>) -> Result<()> {
    ensure_same_extent(lhs, rhs)?;
    if lhs.format == rhs.format {
        Ok(())
    } else {
        Err(Error::InvalidValue(
            "vImage buffers must share the same pixel format",
        ))
    }
}

/// Borrowed wrapper around a caller-owned image buffer.
#[derive(Debug)]
pub struct ImageBuffer<'a> {
    data: *mut u8,
    format: PixelFormat,
    width: usize,
    height: usize,
    row_bytes: usize,
    _marker: PhantomData<&'a mut [u8]>,
}

impl<'a> ImageBuffer<'a> {
    pub fn new(
        data: &'a mut [u8],
        format: PixelFormat,
        width: usize,
        height: usize,
    ) -> Result<Self> {
        let row_bytes = packed_row_bytes(format, width)?;
        Self::with_row_bytes(data, format, width, height, row_bytes)
    }

    pub fn with_row_bytes(
        data: &'a mut [u8],
        format: PixelFormat,
        width: usize,
        height: usize,
        row_bytes: usize,
    ) -> Result<Self> {
        let min_row_bytes = packed_row_bytes(format, width)?;
        if row_bytes < min_row_bytes {
            return Err(Error::InvalidValue(
                "vImage row bytes must be at least width times bytes per pixel",
            ));
        }
        if isize::try_from(row_bytes).is_err() || isize::try_from(height).is_err() {
            return Err(Error::OperationFailed("image dimensions overflowed"));
        }
        let expected = match height.checked_sub(1) {
            Some(last_row) => last_row
                .checked_mul(row_bytes)
                .and_then(|offset| offset.checked_add(min_row_bytes))
                .ok_or(Error::OperationFailed("image dimensions overflowed"))?,
            None => 0,
        };
        if data.len() < expected {
            return Err(Error::InvalidLength {
                expected,
                actual: data.len(),
            });
        }
        let float_alignment = core::mem::align_of::<f32>();
        if format.is_float()
            && ((data.as_ptr() as usize) % float_alignment != 0 || row_bytes % float_alignment != 0)
        {
            return Err(Error::InvalidValue(
                "floating-point vImage buffers must be aligned to 4 bytes",
            ));
        }

        Ok(Self {
            data: data.as_mut_ptr(),
            format,
            width,
            height,
            row_bytes,
            _marker: PhantomData,
        })
    }

    /// Borrows caller-owned ARGB8888 storage as a `vImage_Buffer`.
    pub fn from_argb8888(data: &'a mut [u8], width: usize, height: usize) -> Result<Self> {
        Self::new(data, PixelFormat::Argb8888, width, height)
    }

    /// Borrows caller-owned Planar8 storage as a `vImage_Buffer`.
    pub fn from_planar8(data: &'a mut [u8], width: usize, height: usize) -> Result<Self> {
        Self::new(data, PixelFormat::Planar8, width, height)
    }

    pub const fn format(&self) -> PixelFormat {
        self.format
    }

    pub const fn width(&self) -> usize {
        self.width
    }

    pub const fn height(&self) -> usize {
        self.height
    }

    pub const fn row_bytes(&self) -> usize {
        self.row_bytes
    }

    fn data_ptr(&self) -> *mut c_void {
        self.data.cast()
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
    expect_format(src, INTERLEAVED_8888)?;
    expect_format(dst, &[src.format])?;

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
    expect_format(src, INTERLEAVED_8888)?;
    expect_format(dst, &[src.format])?;

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
    expect_format(src, INTERLEAVED_8888)?;
    expect_format(dst, &[src.format])?;

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
    expect_format(src, PLANAR8)?;
    expect_format(dst, PLANAR8)?;
    ensure_same_dimensions(src, dst)?;

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
    expect_format(src_top, ARGB8888)?;
    expect_format(src_bottom, ARGB8888)?;
    expect_format(dst, ARGB8888)?;
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
    expect_format(src, ARGB8888)?;
    expect_format(dst, ARGB8888)?;
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
    expect_format(src, ARGB8888)?;
    expect_format(dst, ARGB8888)?;
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
    expect_format(src, ARGB8888)?;
    expect_format(dst, ARGB8888)?;
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
    for plane in [src_alpha, src_red, src_green, src_blue] {
        expect_format(plane, PLANAR8)?;
        ensure_same_dimensions(src_alpha, plane)?;
    }
    expect_format(dst, ARGB8888)?;
    ensure_same_extent(src_alpha, dst)?;

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
    expect_format(src, ARGB8888)?;
    for plane in [&*dst_alpha, &*dst_red, &*dst_green, &*dst_blue] {
        expect_format(plane, PLANAR8)?;
        ensure_same_extent(src, plane)?;
    }

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
