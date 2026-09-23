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

#[test]
fn sparse_negative_leading_index_is_rejected() {
    let values = [7.0_f32, 1.0];
    let indices = [-5_i64, 3_i64];
    let dense = [1.0_f32, 2.0, 3.0, 4.0];

    let error = sparse_dot_dense_f32(&values, &indices, &dense).expect_err("negative index");
    assert!(error.to_string().contains("non-negative"));

    let mut target = vec![0.0_f32; 4];
    let error =
        sparse_add_to_dense_f32(&values, &indices, 1.0, &mut target).expect_err("negative index");
    assert!(error.to_string().contains("non-negative"));
    assert_eq!(target, vec![0.0_f32; 4]);

    let error = sparse_dot_sparse_f32(&values, &indices, &[1.0_f32], &[3_i64])
        .expect_err("negative lhs index");
    assert!(error.to_string().contains("non-negative"));
    let error = sparse_dot_sparse_f32(&[1.0_f32], &[3_i64], &values, &indices)
        .expect_err("negative rhs index");
    assert!(error.to_string().contains("non-negative"));
}

#[test]
fn sparse_every_index_is_validated() {
    let dense = [1.0_f32, 2.0, 3.0];
    let mut target = vec![0.0_f32; 3];

    for indices in [
        [i64::MIN, 0],
        [-1, 2],
        [0, 3],
        [1, i64::MAX],
        [2, 1],
        [3, -5],
    ] {
        let values = [1.0_f32, 1.0];
        assert!(
            sparse_dot_dense_f32(&values, &indices, &dense).is_err(),
            "dot accepted {indices:?}"
        );
        assert!(
            sparse_add_to_dense_f32(&values, &indices, 1.0, &mut target).is_err(),
            "add accepted {indices:?}"
        );
    }
    assert_eq!(target, vec![0.0_f32; 3]);

    let error = sparse_dot_dense_f32(&[1.0_f32], &[3_i64], &dense).expect_err("index past end");
    assert_eq!(
        error,
        apple_accelerate::Error::InvalidLength {
            expected: 4,
            actual: 3,
        }
    );
}

#[test]
fn sparse_boundary_indices_are_accepted() {
    let dense = [1.0_f32, 2.0, 3.0];
    let dot = sparse_dot_dense_f32(&[1.0_f32, 1.0], &[0_i64, 2_i64], &dense).expect("dot");
    assert!((dot - 4.0).abs() < 1.0e-6);

    assert!(
        sparse_dot_dense_f32(&[], &[], &[])
            .expect("empty dot")
            .abs()
            < f32::EPSILON
    );

    let mut target = vec![0.0_f32; 3];
    sparse_add_to_dense_f32(&[2.0_f32], &[2_i64], 1.0, &mut target).expect("add");
    assert_eq!(target, vec![0.0, 0.0, 2.0]);
}

#[test]
fn sparse_matrix_is_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<SparseMatrixF32>();
}

#[test]
fn sparse_concurrent_solves_with_pending_inserts() {
    let dimension = 64;
    let mut matrix = SparseMatrixF32::new(dimension, dimension).expect("matrix");
    matrix
        .set_property(sparse_matrix_property::LOWER_TRIANGULAR)
        .expect("property");
    for row in 0..dimension {
        matrix.insert_entry(row, row, 2.0).expect("diagonal");
        if row > 0 {
            matrix.insert_entry(row, row - 1, 1.0).expect("subdiagonal");
        }
    }

    let matrix = std::sync::Arc::new(matrix);
    let workers: Vec<_> = (0..8)
        .map(|_| {
            let matrix = std::sync::Arc::clone(&matrix);
            std::thread::spawn(move || {
                let mut rhs = vec![0.0_f32; dimension];
                rhs[0] = 2.0;
                for value in rhs.iter_mut().skip(1) {
                    *value = 3.0;
                }
                matrix
                    .triangular_solve_vector(blas_transpose::NO_TRANS, 1.0, &mut rhs)
                    .expect("solve");
                let nonzeros = matrix.nonzero_count().expect("nnz");
                (rhs, nonzeros)
            })
        })
        .collect();

    for worker in workers {
        let (solution, nonzeros) = worker.join().expect("worker");
        assert_eq!(nonzeros, 2 * dimension - 1);
        assert!(solution.iter().all(|value| (*value - 1.0).abs() < 1.0e-5));
    }
}
