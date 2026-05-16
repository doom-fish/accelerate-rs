use apple_accelerate::{add_f32x4, dot_f32x4, length_f32x4, normalize_f32x4};

#[test]
fn simd_helpers_smoke() {
    let a = [1.0_f32, 2.0, 3.0, 4.0];
    let b = [4.0_f32, 3.0, 2.0, 1.0];

    let added = add_f32x4(a, b).expect("add");
    assert!(added
        .iter()
        .zip([5.0_f32, 5.0, 5.0, 5.0])
        .all(|(actual, expected)| (*actual - expected).abs() < 1.0e-6));
    assert!((dot_f32x4(a, b).expect("dot") - 20.0).abs() < 1.0e-6);
    assert!((length_f32x4(a).expect("length") - 5.477_226).abs() < 1.0e-5);

    let normalized = normalize_f32x4(a).expect("normalize");
    assert!((normalized[0] - 0.182_574_18).abs() < 1.0e-6);
}

#[test]
fn simd_zero_vector_is_rejected() {
    let error = normalize_f32x4([0.0_f32; 4]).expect_err("zero vector");
    assert!(error.to_string().contains("zero vector"));
}
