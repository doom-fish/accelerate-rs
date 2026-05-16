use apple_accelerate::{add_f32x4, dot_f32x4, length_f32x4, normalize_f32x4};

fn main() {
    let a = [1.0_f32, 2.0, 3.0, 4.0];
    let b = [4.0_f32, 3.0, 2.0, 1.0];
    let added = add_f32x4(a, b).expect("add");
    assert!(added.iter().all(|value| (*value - 5.0).abs() < 1.0e-6));

    let dot = dot_f32x4(a, b).expect("dot");
    assert!((dot - 20.0).abs() < 1.0e-6);

    let length = length_f32x4(a).expect("length");
    assert!((length - 5.477_226).abs() < 1.0e-5);

    let normalized = normalize_f32x4(a).expect("normalize");
    assert!((normalized[0] - 0.182_574_18).abs() < 1.0e-6);

    println!("simd smoke passed: added={added:?} dot={dot} normalized={normalized:?}");
}
