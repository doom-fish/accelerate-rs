use apple_accelerate::{add_f32, blackman_window, dot_f32, hamming_window, sub_f32};

#[test]
fn vector_ops_and_windows_smoke() {
    let add = add_f32(&[1.0_f32, 2.0], &[3.0_f32, 4.0]).expect("add");
    assert!(add
        .iter()
        .zip([4.0_f32, 6.0])
        .all(|(actual, expected)| (*actual - expected).abs() < 1.0e-6));

    let sub = sub_f32(&[5.0_f32, 7.0], &[3.0_f32, 2.0]).expect("sub");
    assert!(sub
        .iter()
        .zip([2.0_f32, 5.0])
        .all(|(actual, expected)| (*actual - expected).abs() < 1.0e-6));

    let dot = dot_f32(&[1.0_f32, 2.0, 3.0], &[4.0_f32, 5.0, 6.0]).expect("dot");
    assert!((dot - 32.0).abs() < 1.0e-6);

    assert_eq!(hamming_window(16, 0).len(), 16);
    assert_eq!(blackman_window(16, 0).len(), 16);
}

#[test]
fn vector_op_length_mismatch_errors() {
    let error = add_f32(&[1.0_f32], &[1.0_f32, 2.0]).expect_err("length mismatch");
    assert!(error.to_string().contains("invalid length"));
}
