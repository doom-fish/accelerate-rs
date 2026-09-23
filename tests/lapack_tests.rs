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

#[test]
fn lapack_empty_inputs_do_not_reach_lapack() {
    let lu = lu_decompose_f32(&[], 0).expect("empty factorization");
    assert_eq!(lu.dimension(), 0);
    assert!(lu.factors().is_empty());
    assert!(lu.pivots().is_empty());
    assert_eq!(lu.info(), 0);
    assert!(!lu.is_singular());

    let error = solve_linear_system_f32(&[], 0, &[1.0_f32]).expect_err("rhs for an empty system");
    assert_eq!(
        error,
        apple_accelerate::Error::InvalidLength {
            expected: 0,
            actual: 1,
        }
    );
    assert!(solve_linear_system_f32(&[], 0, &[])
        .expect("empty system")
        .is_empty());
}

#[test]
fn lapack_singular_factorization_is_returned_with_its_info() {
    let singular = [1.0_f32, 2.0, 2.0, 4.0];
    let lu = lu_decompose_f32(&singular, 2).expect("singular factorization");
    assert!(lu.info() > 0);
    assert!(lu.is_singular());
    assert_eq!(lu.factors().len(), 4);
    assert_eq!(lu.pivots().len(), 2);
    assert!((lu.factors()[0] - 2.0).abs() < 1.0e-6);
    assert!(lu.factors()[3].abs() < 1.0e-6);

    let error = solve_linear_system_f32(&singular, 2, &[1.0_f32, 2.0]).expect_err("singular");
    assert!(matches!(error, apple_accelerate::Error::LapackInfo(info) if info > 0));

    let regular = lu_decompose_f32(&[3.0_f32, 1.0, 1.0, 2.0], 2).expect("regular");
    assert_eq!(regular.info(), 0);
    assert!(!regular.is_singular());
}
