use apple_accelerate::{
    blas_transpose, sparse_add_to_dense_f32, sparse_dot_dense_f32, sparse_dot_sparse_f32,
    sparse_matrix_property, SparseMatrixF32,
};

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
fn sparse_matrix_triangular_solve_smoke() {
    let mut matrix = SparseMatrixF32::new(2, 2).expect("matrix");
    matrix
        .set_property(sparse_matrix_property::LOWER_TRIANGULAR)
        .expect("property");
    matrix.insert_entry(0, 0, 2.0).expect("a00");
    matrix.insert_entry(1, 0, 1.0).expect("a10");
    matrix.insert_entry(1, 1, 3.0).expect("a11");
    matrix.commit().expect("commit");

    assert_eq!(matrix.rows().expect("rows"), 2);
    assert_eq!(matrix.columns().expect("cols"), 2);
    assert_eq!(matrix.nonzero_count().expect("nnz"), 3);

    let mut rhs = [2.0_f32, 7.0];
    matrix
        .triangular_solve_vector(blas_transpose::NO_TRANS, 1.0, &mut rhs)
        .expect("solve vector");
    assert!(rhs
        .iter()
        .zip([1.0_f32, 2.0])
        .all(|(actual, expected)| (*actual - expected).abs() < 1.0e-6));

    let mut rhs_matrix = [2.0_f32, 4.0, 7.0, 10.0];
    matrix
        .triangular_solve_matrix_row_major(blas_transpose::NO_TRANS, 2, 1.0, &mut rhs_matrix)
        .expect("solve matrix");
    assert!(rhs_matrix
        .iter()
        .zip([1.0_f32, 2.0, 2.0, 8.0 / 3.0])
        .all(|(actual, expected)| (*actual - expected).abs() < 1.0e-5));
}

#[test]
fn sparse_indices_must_be_sorted() {
    let error = sparse_dot_dense_f32(&[1.0_f32, 2.0], &[1_i64, 1_i64], &[1.0_f32, 2.0])
        .expect_err("duplicate indices");
    assert!(error.to_string().contains("strictly increasing"));
}
