use apple_accelerate::{sparse_add_to_dense_f32, sparse_dot_dense_f32, sparse_dot_sparse_f32};

#[test]
fn sparse_vector_ops_smoke() {
    let values = [2.0_f32, 4.0];
    let indices = [0_i64, 2_i64];
    let dense = [1.0_f32, 2.0, 3.0];

    let dense_dot = sparse_dot_dense_f32(&values, &indices, &dense).expect("dense dot");
    assert!((dense_dot - 14.0).abs() < 1.0e-6);

    let sparse_dot = sparse_dot_sparse_f32(&values, &indices, &[3.0_f32, 5.0], &[0_i64, 2_i64])
        .expect("sparse dot");
    assert!((sparse_dot - 26.0).abs() < 1.0e-6);

    let mut output = vec![10.0_f32, 10.0, 10.0];
    sparse_add_to_dense_f32(&values, &indices, 0.5, &mut output).expect("add");
    assert!(output
        .iter()
        .zip([11.0_f32, 10.0, 12.0])
        .all(|(actual, expected)| (*actual - expected).abs() < 1.0e-6));
}

#[test]
fn sparse_indices_must_be_sorted() {
    let error = sparse_dot_dense_f32(&[1.0_f32, 2.0], &[1_i64, 1_i64], &[1.0_f32, 2.0])
        .expect_err("duplicate indices");
    assert!(error.to_string().contains("strictly increasing"));
}
