use apple_accelerate::{contrast_stretch_planar8, scale_argb8888, vimage_flags, ImageBuffer};

#[test]
fn vimage_scale_and_contrast_smoke() {
    let mut source = vec![255_u8, 10, 20, 30];
    let mut scaled = vec![0_u8; 16];
    let src = ImageBuffer::from_argb8888(&mut source, 1, 1).expect("src");
    let mut dst = ImageBuffer::from_argb8888(&mut scaled, 2, 2).expect("dst");
    scale_argb8888(&src, &mut dst, vimage_flags::NO_FLAGS).expect("scale");
    for pixel in scaled.chunks_exact(4) {
        assert_eq!(pixel, &[255, 10, 20, 30]);
    }

    let mut planar_src = vec![10_u8, 20, 30, 40];
    let mut planar_dst = vec![0_u8; 4];
    let src_planar = ImageBuffer::from_planar8(&mut planar_src, 2, 2).expect("planar src");
    let mut dst_planar = ImageBuffer::from_planar8(&mut planar_dst, 2, 2).expect("planar dst");
    contrast_stretch_planar8(&src_planar, &mut dst_planar, vimage_flags::NO_FLAGS)
        .expect("stretch");
    assert_eq!(planar_dst, vec![0, 85, 170, 255]);
}
