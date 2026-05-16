use apple_accelerate::{
    blas_transpose, sparse_add_to_dense_f32, sparse_dot_dense_f32, sparse_dot_sparse_f32,
    sparse_matrix_property, SparseMatrixF32,
};

fn main() {
    let values = [2.0_f32, 4.0];
    let indices = [0_i64, 2_i64];
    let dense = [1.0_f32, 2.0, 3.0];
    let dot = sparse_dot_dense_f32(&values, &indices, &dense).expect("dense dot");
    assert!((dot - 14.0).abs() < 1.0e-6);

    let sparse_dot = sparse_dot_sparse_f32(&values, &indices, &[3.0_f32, 5.0], &[0_i64, 2_i64])
        .expect("sparse dot");
    assert!((sparse_dot - 26.0).abs() < 1.0e-6);

    let mut accum = vec![10.0_f32, 10.0, 10.0];
    sparse_add_to_dense_f32(&values, &indices, 0.5, &mut accum).expect("accumulate");
    assert!(accum
        .iter()
        .zip([11.0_f32, 10.0, 12.0])
        .all(|(actual, expected)| (*actual - expected).abs() < 1.0e-6));

    let mut matrix = SparseMatrixF32::new(2, 2).expect("matrix");
    matrix
        .set_property(sparse_matrix_property::LOWER_TRIANGULAR)
        .expect("property");
    matrix.insert_entry(0, 0, 2.0).expect("a00");
    matrix.insert_entry(1, 0, 1.0).expect("a10");
    matrix.insert_entry(1, 1, 3.0).expect("a11");
    matrix.commit().expect("commit");

    let mut rhs = [2.0_f32, 7.0];
    matrix
        .triangular_solve_vector(blas_transpose::NO_TRANS, 1.0, &mut rhs)
        .expect("solve vector");
    assert!(rhs
        .iter()
        .zip([1.0_f32, 2.0])
        .all(|(actual, expected)| (*actual - expected).abs() < 1.0e-6));

    println!("sparse smoke passed: dot={dot} accum={accum:?} solve={rhs:?}");
}
