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

#[test]
fn sgemv_with_zero_columns_scales_y_without_calling_cblas() {
    let mut y = vec![1.0_f32, -2.0, 3.0];
    sgemv_row_major(3, 0, 1.0, &[], &[], 2.0, &mut y).expect("zero columns");
    assert_eq!(y, vec![2.0, -4.0, 6.0]);

    let mut y = vec![f32::NAN, 5.0, 7.0];
    sgemv_row_major(3, 0, 1.0, &[], &[], 0.0, &mut y).expect("zero columns, zero beta");
    assert_eq!(y, vec![0.0, 0.0, 0.0]);

    let mut empty: Vec<f32> = Vec::new();
    sgemv_row_major(0, 3, 1.0, &[], &[1.0, 2.0, 3.0], 1.0, &mut empty).expect("zero rows");
    sgemv_row_major(0, 0, 1.0, &[], &[], 1.0, &mut empty).expect("empty matrix");
}

#[test]
fn sgemm_with_zero_dimensions_does_not_reach_cblas() {
    let mut output = vec![1.0_f32, 2.0, 3.0, 4.0];
    sgemm_row_major(2, 2, 0, 1.0, &[], &[], 3.0, &mut output).expect("zero inner dimension");
    assert_eq!(output, vec![3.0, 6.0, 9.0, 12.0]);

    sgemm_row_major(2, 2, 0, 1.0, &[], &[], 0.0, &mut output).expect("zero beta");
    assert_eq!(output, vec![0.0; 4]);

    let mut empty: Vec<f32> = Vec::new();
    sgemm_row_major(0, 2, 3, 1.0, &[], &[0.0; 6], 1.0, &mut empty).expect("zero rows");
    sgemm_row_major(2, 0, 3, 1.0, &[0.0; 6], &[], 1.0, &mut empty).expect("zero columns");
    sgemm_row_major(0, 0, 0, 1.0, &[], &[], 1.0, &mut empty).expect("all zero");
}

#[test]
fn blas_length_mismatches_are_still_rejected() {
    let mut y = vec![0.0_f32; 3];
    assert!(sgemv_row_major(3, 0, 1.0, &[1.0], &[], 1.0, &mut y).is_err());
    assert!(sgemv_row_major(3, 0, 1.0, &[], &[1.0], 1.0, &mut y).is_err());
    let mut output = vec![0.0_f32; 3];
    assert!(sgemm_row_major(2, 2, 0, 1.0, &[], &[], 1.0, &mut output).is_err());
    assert!(sdot(&[], &[]).expect("empty dot").abs() < f32::EPSILON);
}
