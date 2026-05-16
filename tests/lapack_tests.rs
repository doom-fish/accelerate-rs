use apple_accelerate::{lu_decompose_f32, solve_linear_system_f32};

#[test]
fn lapack_factor_and_solve_smoke() {
    let matrix = [3.0_f32, 1.0, 1.0, 2.0];
    let lu = lu_decompose_f32(&matrix, 2).expect("lu");
    assert_eq!(lu.dimension(), 2);
    assert_eq!(lu.factors().len(), 4);
    assert_eq!(lu.pivots().len(), 2);

    let solution = solve_linear_system_f32(&matrix, 2, &[9.0_f32, 8.0]).expect("solve");
    assert!((solution[0] - 2.0).abs() < 1.0e-5);
    assert!((solution[1] - 3.0).abs() < 1.0e-5);
}

#[test]
fn lapack_length_checks() {
    let error = solve_linear_system_f32(&[1.0_f32, 2.0, 3.0], 2, &[1.0_f32, 2.0])
        .expect_err("matrix length mismatch");
    assert!(error.to_string().contains("invalid length"));
}
