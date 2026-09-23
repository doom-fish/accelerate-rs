use apple_accelerate::{
    alpha_blend_argb8888, box_convolve_argb8888, clip_to_alpha_argb8888, contrast_stretch_planar8,
    convert_argb8888_to_planar8, convert_planar8_to_argb8888, premultiply_argb8888,
    rotate_argb8888, scale_argb8888, unpremultiply_argb8888, vimage_flags, Error, ImageBuffer,
    PixelFormat,
};

#[test]
fn vimage_scale_and_conversion_smoke() {
    let mut source = vec![255_u8, 10, 20, 30];
    let mut scaled = vec![0_u8; 16];
    let src = ImageBuffer::from_argb8888(&mut source, 1, 1).expect("src");
    let mut dst = ImageBuffer::from_argb8888(&mut scaled, 2, 2).expect("dst");
    scale_argb8888(&src, &mut dst, vimage_flags::NO_FLAGS).expect("scale");
    for pixel in scaled.chunks_exact(4) {
        assert_eq!(pixel, &[255, 10, 20, 30]);
    }

    let mut interleaved = vec![255_u8, 10, 20, 30, 128, 40, 50, 60];
    let mut alpha = vec![0_u8; 2];
    let mut red = vec![0_u8; 2];
    let mut green = vec![0_u8; 2];
    let mut blue = vec![0_u8; 2];
    let src_interleaved =
        ImageBuffer::from_argb8888(&mut interleaved, 2, 1).expect("src interleaved");
    let mut alpha_plane = ImageBuffer::from_planar8(&mut alpha, 2, 1).expect("alpha");
    let mut red_plane = ImageBuffer::from_planar8(&mut red, 2, 1).expect("red");
    let mut green_plane = ImageBuffer::from_planar8(&mut green, 2, 1).expect("green");
    let mut blue_plane = ImageBuffer::from_planar8(&mut blue, 2, 1).expect("blue");
    convert_argb8888_to_planar8(
        &src_interleaved,
        &mut alpha_plane,
        &mut red_plane,
        &mut green_plane,
        &mut blue_plane,
        vimage_flags::NO_FLAGS,
    )
    .expect("deinterleave");
    assert_eq!(alpha, vec![255, 128]);
    assert_eq!(red, vec![10, 40]);
    assert_eq!(green, vec![20, 50]);
    assert_eq!(blue, vec![30, 60]);

    let mut reinterleaved = vec![0_u8; 8];
    let alpha_plane = ImageBuffer::from_planar8(&mut alpha, 2, 1).expect("alpha src");
    let red_plane = ImageBuffer::from_planar8(&mut red, 2, 1).expect("red src");
    let green_plane = ImageBuffer::from_planar8(&mut green, 2, 1).expect("green src");
    let blue_plane = ImageBuffer::from_planar8(&mut blue, 2, 1).expect("blue src");
    let mut dst_interleaved =
        ImageBuffer::from_argb8888(&mut reinterleaved, 2, 1).expect("dst interleaved");
    convert_planar8_to_argb8888(
        &alpha_plane,
        &red_plane,
        &green_plane,
        &blue_plane,
        &mut dst_interleaved,
        vimage_flags::NO_FLAGS,
    )
    .expect("reinterleave");
    assert_eq!(reinterleaved, interleaved);

    let mut top = vec![255_u8, 1, 2, 3];
    let mut bottom = vec![255_u8, 9, 9, 9];
    let mut blended = vec![0_u8; 4];
    let src_top = ImageBuffer::from_argb8888(&mut top, 1, 1).expect("top");
    let src_bottom = ImageBuffer::from_argb8888(&mut bottom, 1, 1).expect("bottom");
    let mut dst_blended = ImageBuffer::from_argb8888(&mut blended, 1, 1).expect("blended");
    alpha_blend_argb8888(
        &src_top,
        &src_bottom,
        &mut dst_blended,
        vimage_flags::NO_FLAGS,
    )
    .expect("blend");
    assert_eq!(blended, top);

    let mut clip_source = vec![64_u8, 100, 50, 10];
    let mut clip_dest = vec![0_u8; 4];
    let src_clip = ImageBuffer::from_argb8888(&mut clip_source, 1, 1).expect("clip src");
    let mut dst_clip = ImageBuffer::from_argb8888(&mut clip_dest, 1, 1).expect("clip dst");
    clip_to_alpha_argb8888(&src_clip, &mut dst_clip, vimage_flags::NO_FLAGS).expect("clip");
    assert_eq!(clip_dest, vec![64, 64, 50, 10]);
}

#[test]
fn vimage_alpha_and_histogram_smoke() {
    let mut premul_source = vec![255_u8, 10, 20, 30, 0, 40, 50, 60];
    let mut premul_dest = vec![0_u8; 8];
    let src_premul = ImageBuffer::from_argb8888(&mut premul_source, 2, 1).expect("premul src");
    let mut dst_premul = ImageBuffer::from_argb8888(&mut premul_dest, 2, 1).expect("premul dst");
    premultiply_argb8888(&src_premul, &mut dst_premul, vimage_flags::NO_FLAGS)
        .expect("premultiply");
    assert_eq!(premul_dest, vec![255, 10, 20, 30, 0, 0, 0, 0]);

    let src_unpremul = ImageBuffer::from_argb8888(&mut premul_dest, 2, 1).expect("unpremul src");
    let mut restored = vec![0_u8; 8];
    let mut dst_unpremul = ImageBuffer::from_argb8888(&mut restored, 2, 1).expect("unpremul dst");
    unpremultiply_argb8888(&src_unpremul, &mut dst_unpremul, vimage_flags::NO_FLAGS)
        .expect("unpremultiply");
    assert_eq!(restored, vec![255, 10, 20, 30, 0, 0, 0, 0]);

    let mut planar_src = vec![10_u8, 20, 30, 40];
    let mut planar_dst = vec![0_u8; 4];
    let src_planar = ImageBuffer::from_planar8(&mut planar_src, 2, 2).expect("planar src");
    let mut dst_planar = ImageBuffer::from_planar8(&mut planar_dst, 2, 2).expect("planar dst");
    contrast_stretch_planar8(&src_planar, &mut dst_planar, vimage_flags::NO_FLAGS)
        .expect("stretch");
    assert_eq!(planar_dst, vec![0, 85, 170, 255]);
}

fn is_format_error(error: &Error) -> bool {
    error.to_string().contains("pixel format")
}

#[test]
fn vimage_planar_destination_is_rejected_by_argb_ops() {
    let mut argb = vec![255_u8, 10, 20, 30, 128, 40, 50, 60, 64, 1, 2, 3, 0, 4, 5, 6];
    let mut planar = vec![0xAA_u8; 4];
    let src = ImageBuffer::from_argb8888(&mut argb, 2, 2).expect("argb");
    let mut dst = ImageBuffer::from_planar8(&mut planar, 2, 2).expect("planar");

    let error = premultiply_argb8888(&src, &mut dst, vimage_flags::NO_FLAGS)
        .expect_err("planar destination");
    assert!(is_format_error(&error), "{error}");
    let error = unpremultiply_argb8888(&src, &mut dst, vimage_flags::NO_FLAGS)
        .expect_err("planar destination");
    assert!(is_format_error(&error), "{error}");
    let error = clip_to_alpha_argb8888(&src, &mut dst, vimage_flags::NO_FLAGS)
        .expect_err("planar destination");
    assert!(is_format_error(&error), "{error}");
    let error = alpha_blend_argb8888(&src, &src, &mut dst, vimage_flags::NO_FLAGS)
        .expect_err("planar destination");
    assert!(is_format_error(&error), "{error}");
    let error =
        scale_argb8888(&src, &mut dst, vimage_flags::NO_FLAGS).expect_err("planar destination");
    assert!(is_format_error(&error), "{error}");
    let error = rotate_argb8888(&src, &mut dst, 0.0, [0; 4], vimage_flags::NO_FLAGS)
        .expect_err("planar destination");
    assert!(is_format_error(&error), "{error}");
    let error = box_convolve_argb8888(&src, &mut dst, 1, 1, [0; 4], vimage_flags::EDGE_EXTEND)
        .expect_err("planar destination");
    assert!(is_format_error(&error), "{error}");
    assert_eq!(planar, vec![0xAA_u8; 4]);
}

#[test]
fn vimage_planar_source_is_rejected_by_argb_ops() {
    let mut planar = vec![1_u8, 2, 3, 4];
    let mut argb = vec![0_u8; 16];
    let src = ImageBuffer::from_planar8(&mut planar, 2, 2).expect("planar");
    let mut dst = ImageBuffer::from_argb8888(&mut argb, 2, 2).expect("argb");

    assert!(premultiply_argb8888(&src, &mut dst, vimage_flags::NO_FLAGS).is_err());
    assert!(scale_argb8888(&src, &mut dst, vimage_flags::NO_FLAGS).is_err());
    assert!(rotate_argb8888(&src, &mut dst, 0.0, [0; 4], vimage_flags::NO_FLAGS).is_err());
    assert_eq!(argb, vec![0_u8; 16]);
}

#[test]
fn vimage_contrast_stretch_requires_planar8() {
    let mut argb_src = vec![0_u8; 16];
    let mut argb_dst = vec![0_u8; 16];
    let src = ImageBuffer::from_argb8888(&mut argb_src, 2, 2).expect("src");
    let mut dst = ImageBuffer::from_argb8888(&mut argb_dst, 2, 2).expect("dst");
    let error =
        contrast_stretch_planar8(&src, &mut dst, vimage_flags::NO_FLAGS).expect_err("argb buffers");
    assert!(is_format_error(&error), "{error}");

    let mut planar_src = vec![0_u8; 4];
    let mut planar_dst = vec![0_u8; 2];
    let src = ImageBuffer::from_planar8(&mut planar_src, 2, 2).expect("src");
    let mut dst = ImageBuffer::from_planar8(&mut planar_dst, 2, 1).expect("dst");
    assert!(contrast_stretch_planar8(&src, &mut dst, vimage_flags::NO_FLAGS).is_err());
}

#[test]
fn vimage_conversions_check_every_plane() {
    let mut interleaved = vec![0_u8; 8];
    let mut alpha = vec![0_u8; 2];
    let mut red = vec![0_u8; 8];
    let mut green = vec![0_u8; 2];
    let mut blue = vec![0_u8; 2];
    let src = ImageBuffer::from_argb8888(&mut interleaved, 2, 1).expect("src");
    let mut alpha = ImageBuffer::from_planar8(&mut alpha, 2, 1).expect("alpha");
    let mut red = ImageBuffer::from_argb8888(&mut red, 2, 1).expect("red");
    let mut green = ImageBuffer::from_planar8(&mut green, 2, 1).expect("green");
    let mut blue = ImageBuffer::from_planar8(&mut blue, 2, 1).expect("blue");
    let error = convert_argb8888_to_planar8(
        &src,
        &mut alpha,
        &mut red,
        &mut green,
        &mut blue,
        vimage_flags::NO_FLAGS,
    )
    .expect_err("argb plane");
    assert!(is_format_error(&error), "{error}");

    let mut out = vec![0_u8; 8];
    let mut dst = ImageBuffer::from_argb8888(&mut out, 2, 1).expect("dst");
    let error = convert_planar8_to_argb8888(
        &alpha,
        &red,
        &green,
        &blue,
        &mut dst,
        vimage_flags::NO_FLAGS,
    )
    .expect_err("argb source plane");
    assert!(is_format_error(&error), "{error}");

    let mut small = vec![0_u8; 1];
    let small = ImageBuffer::from_planar8(&mut small, 1, 1).expect("1x1");
    let error = convert_planar8_to_argb8888(
        &alpha,
        &small,
        &green,
        &blue,
        &mut dst,
        vimage_flags::NO_FLAGS,
    )
    .expect_err("plane extent");
    assert!(error.to_string().contains("width and height"), "{error}");
    assert_eq!(out, vec![0_u8; 8]);
}

#[test]
fn vimage_huge_width_with_zero_height_is_rejected() {
    assert!(ImageBuffer::from_argb8888(&mut [], usize::MAX, 0).is_err());
    assert!(ImageBuffer::from_argb8888(&mut [], usize::MAX / 4 + 1, 0).is_err());
    assert!(ImageBuffer::from_planar8(&mut [], usize::MAX, 0).is_err());
    assert!(ImageBuffer::new(&mut [], PixelFormat::ArgbFFFF, usize::MAX / 8, 0).is_err());
    let empty = ImageBuffer::from_argb8888(&mut [], 4, 0).expect("zero height");
    assert_eq!(empty.row_bytes(), 16);
    assert_eq!(empty.height(), 0);
}

#[test]
fn vimage_row_bytes_are_validated() {
    let mut storage = vec![0_u8; 64];
    assert!(ImageBuffer::with_row_bytes(&mut storage, PixelFormat::Argb8888, 4, 2, 15).is_err());
    let error = ImageBuffer::with_row_bytes(&mut storage[..23], PixelFormat::Argb8888, 2, 2, 16)
        .expect_err("short storage");
    assert_eq!(
        error,
        Error::InvalidLength {
            expected: 24,
            actual: 23,
        }
    );
    let buffer = ImageBuffer::with_row_bytes(&mut storage[..24], PixelFormat::Argb8888, 2, 2, 16)
        .expect("padded");
    assert_eq!(buffer.format(), PixelFormat::Argb8888);
    assert_eq!(buffer.width(), 2);
    assert_eq!(buffer.height(), 2);
    assert_eq!(buffer.row_bytes(), 16);
}

#[test]
fn vimage_padded_rows_leave_padding_untouched() {
    let mut src_storage = vec![0x55_u8; 12];
    src_storage[..4].copy_from_slice(&[255, 10, 20, 30]);
    src_storage[8..12].copy_from_slice(&[0, 40, 50, 60]);
    let mut dst_storage = vec![0xEE_u8; 12];
    let src =
        ImageBuffer::with_row_bytes(&mut src_storage, PixelFormat::Argb8888, 1, 2, 8).expect("src");
    let mut dst =
        ImageBuffer::with_row_bytes(&mut dst_storage, PixelFormat::Argb8888, 1, 2, 8).expect("dst");
    premultiply_argb8888(&src, &mut dst, vimage_flags::NO_FLAGS).expect("premultiply");
    assert_eq!(&dst_storage[..4], &[255, 10, 20, 30]);
    assert_eq!(&dst_storage[4..8], &[0xEE; 4]);
    assert_eq!(&dst_storage[8..12], &[0, 0, 0, 0]);
}

#[test]
fn vimage_geometry_ops_accept_other_8888_orders_but_not_mixed() {
    let mut bgra = vec![30_u8, 20, 10, 255];
    let mut scaled = vec![0_u8; 16];
    let src = ImageBuffer::new(&mut bgra, PixelFormat::Bgra8888, 1, 1).expect("bgra");
    let mut dst = ImageBuffer::new(&mut scaled, PixelFormat::Bgra8888, 2, 2).expect("dst");
    scale_argb8888(&src, &mut dst, vimage_flags::NO_FLAGS).expect("scale");
    for pixel in scaled.chunks_exact(4) {
        assert_eq!(pixel, &[30, 20, 10, 255]);
    }

    let mut argb_out = vec![0_u8; 16];
    let mut dst = ImageBuffer::from_argb8888(&mut argb_out, 2, 2).expect("argb");
    let error = scale_argb8888(&src, &mut dst, vimage_flags::NO_FLAGS).expect_err("mixed");
    assert!(is_format_error(&error), "{error}");

    let mut premul_out = vec![0_u8; 4];
    let mut dst = ImageBuffer::new(&mut premul_out, PixelFormat::Bgra8888, 1, 1).expect("dst");
    let error =
        premultiply_argb8888(&src, &mut dst, vimage_flags::NO_FLAGS).expect_err("alpha-last");
    assert!(is_format_error(&error), "{error}");
}

#[test]
fn vimage_pixel_formats_report_their_size() {
    assert_eq!(PixelFormat::Planar8.bytes_per_pixel(), 1);
    assert_eq!(PixelFormat::PlanarF.bytes_per_pixel(), 4);
    assert_eq!(PixelFormat::Argb8888.bytes_per_pixel(), 4);
    assert_eq!(PixelFormat::Rgba8888.bytes_per_pixel(), 4);
    assert_eq!(PixelFormat::Bgra8888.bytes_per_pixel(), 4);
    assert_eq!(PixelFormat::ArgbFFFF.bytes_per_pixel(), 16);
    assert_eq!(PixelFormat::RgbaFFFF.bytes_per_pixel(), 16);
    assert_eq!(PixelFormat::BgraFFFF.bytes_per_pixel(), 16);
    assert!(PixelFormat::PlanarF.is_float());
    assert!(!PixelFormat::Planar8.is_float());

    let mut storage = vec![0_u32; 5];
    let bytes = unsafe { std::slice::from_raw_parts_mut(storage.as_mut_ptr().cast::<u8>(), 20) };
    assert!(ImageBuffer::new(&mut bytes[1..17], PixelFormat::PlanarF, 2, 2).is_err());
    assert!(ImageBuffer::with_row_bytes(&mut bytes[..], PixelFormat::PlanarF, 1, 2, 6).is_err());
    let planar_f = ImageBuffer::new(&mut bytes[..16], PixelFormat::PlanarF, 2, 2).expect("aligned");
    assert_eq!(planar_f.row_bytes(), 8);
}
