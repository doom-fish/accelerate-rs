use apple_accelerate::{
    clip_to_alpha_argb8888, contrast_stretch_planar8, convert_argb8888_to_planar8,
    convert_planar8_to_argb8888, scale_argb8888, vimage_flags, ImageBuffer,
};

fn main() {
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

    let alpha_plane = ImageBuffer::from_planar8(&mut alpha, 2, 1).expect("alpha src");
    let red_plane = ImageBuffer::from_planar8(&mut red, 2, 1).expect("red src");
    let green_plane = ImageBuffer::from_planar8(&mut green, 2, 1).expect("green src");
    let blue_plane = ImageBuffer::from_planar8(&mut blue, 2, 1).expect("blue src");
    let mut reinterleaved = vec![0_u8; 8];
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

    let mut clip_source = vec![64_u8, 100, 50, 10];
    let mut clip_dest = vec![0_u8; 4];
    let src_clip = ImageBuffer::from_argb8888(&mut clip_source, 1, 1).expect("clip src");
    let mut dst_clip = ImageBuffer::from_argb8888(&mut clip_dest, 1, 1).expect("clip dst");
    clip_to_alpha_argb8888(&src_clip, &mut dst_clip, vimage_flags::NO_FLAGS).expect("clip");
    assert_eq!(clip_dest, vec![64, 64, 50, 10]);

    let mut planar_src = vec![5_u8, 15, 25, 35];
    let mut planar_dst = vec![0_u8; 4];
    let src_planar = ImageBuffer::from_planar8(&mut planar_src, 2, 2).expect("planar src");
    let mut dst_planar = ImageBuffer::from_planar8(&mut planar_dst, 2, 2).expect("planar dst");
    contrast_stretch_planar8(&src_planar, &mut dst_planar, vimage_flags::NO_FLAGS)
        .expect("stretch");
    assert_eq!(planar_dst, vec![0, 85, 170, 255]);

    println!("vimage smoke passed: scaled={scaled:?} reinterleaved={reinterleaved:?} clipped={clip_dest:?}");
}
