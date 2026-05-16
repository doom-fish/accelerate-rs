use apple_accelerate::{sdot, sgemm_row_major, sgemv_row_major};

#[test]
fn cblas_smoke() {
    let dot = sdot(&[1.0_f32, 2.0, 3.0], &[4.0_f32, 5.0, 6.0]).expect("sdot");
    assert!((dot - 32.0).abs() < 1.0e-6);

    let mut gemv = vec![0.0_f32; 2];
    sgemv_row_major(
        2,
        3,
        1.0,
        &[1.0_f32, 2.0, 3.0, 4.0, 5.0, 6.0],
        &[1.0_f32, 0.5, -1.0],
        0.0,
        &mut gemv,
    )
    .expect("sgemv");
    assert!((gemv[0] + 1.0).abs() < 1.0e-6);
    assert!((gemv[1] - 0.5).abs() < 1.0e-6);

    let mut matrix_product = vec![0.0_f32; 4];
    sgemm_row_major(
        2,
        2,
        3,
        1.0,
        &[1.0_f32, 2.0, 3.0, 4.0, 5.0, 6.0],
        &[7.0_f32, 8.0, 9.0, 10.0, 11.0, 12.0],
        0.0,
        &mut matrix_product,
    )
    .expect("sgemm");
    assert!(matrix_product
        .iter()
        .zip([58.0_f32, 64.0, 139.0, 154.0])
        .all(|(actual, expected)| (*actual - expected).abs() < 1.0e-6));
}
