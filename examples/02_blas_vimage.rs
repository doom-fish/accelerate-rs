use apple_accelerate::{
    contrast_stretch_planar8, rotate_argb8888, sgemm_row_major, vimage_flags, ImageBuffer,
};

fn main() {
    let mut output = vec![0.0_f32; 4];
    sgemm_row_major(
        2,
        2,
        3,
        1.0,
        &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        &[7.0, 8.0, 9.0, 10.0, 11.0, 12.0],
        0.0,
        &mut output,
    )
    .expect("sgemm failed");
    let expected = [58.0_f32, 64.0, 139.0, 154.0];
    for (actual, expected_value) in output.iter().zip(expected) {
        assert!(
            (actual - expected_value).abs() < 1.0e-5,
            "unexpected sgemm output: {output:?}"
        );
    }

    let source_pixels = vec![
        255_u8, 10, 20, 30, 255, 40, 50, 60, 255, 70, 80, 90, 255, 100, 110, 120,
    ];
    let mut src_pixels = source_pixels.clone();
    let mut rotated_pixels = vec![0_u8; source_pixels.len()];
    let src = ImageBuffer::from_argb8888(&mut src_pixels, 2, 2).expect("src buffer");
    let mut rotated =
        ImageBuffer::from_argb8888(&mut rotated_pixels, 2, 2).expect("rotated buffer");
    rotate_argb8888(
        &src,
        &mut rotated,
        0.0,
        [0, 0, 0, 0],
        vimage_flags::NO_FLAGS,
    )
    .expect("rotate failed");
    assert_eq!(rotated_pixels, source_pixels);

    let mut planar_src = vec![10_u8, 20, 30, 40];
    let mut planar_dst = vec![0_u8; 4];
    let planar_src_buffer = ImageBuffer::from_planar8(&mut planar_src, 2, 2).expect("planar src");
    let mut planar_dst_buffer =
        ImageBuffer::from_planar8(&mut planar_dst, 2, 2).expect("planar dst");
    contrast_stretch_planar8(
        &planar_src_buffer,
        &mut planar_dst_buffer,
        vimage_flags::NO_FLAGS,
    )
    .expect("contrast stretch failed");
    assert_eq!(planar_dst, vec![0, 85, 170, 255]);

    println!("blas+vimage smoke passed: sgemm={output:?} contrast={planar_dst:?}");
}
