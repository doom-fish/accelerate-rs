use apple_accelerate::{
    alpha_blend_argb8888, clip_to_alpha_argb8888, contrast_stretch_planar8,
    convert_argb8888_to_planar8, convert_planar8_to_argb8888, premultiply_argb8888, scale_argb8888,
    unpremultiply_argb8888, vimage_flags, ImageBuffer,
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
