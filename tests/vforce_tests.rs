use apple_accelerate::{cos_f32, exp_f32, log_f32, sin_f32, sqrt_f32};

#[test]
fn transcendental_functions_smoke() {
    let sine = sin_f32(&[0.0_f32, std::f32::consts::FRAC_PI_2]).expect("sin");
    assert!(sine[0].abs() < 1.0e-6);
    assert!((sine[1] - 1.0).abs() < 1.0e-5);

    let cosine = cos_f32(&[0.0_f32]).expect("cos");
    assert!((cosine[0] - 1.0).abs() < 1.0e-5);

    let exp = exp_f32(&[0.0_f32, 1.0]).expect("exp");
    assert!((exp[1] - std::f32::consts::E).abs() < 1.0e-5);

    let log = log_f32(&[1.0_f32, std::f32::consts::E]).expect("log");
    assert!(log[0].abs() < 1.0e-6);
    assert!((log[1] - 1.0).abs() < 1.0e-5);

    let sqrt = sqrt_f32(&[1.0_f32, 4.0, 9.0]).expect("sqrt");
    assert!(sqrt
        .iter()
        .zip([1.0_f32, 2.0, 3.0])
        .all(|(actual, expected)| (*actual - expected).abs() < 1.0e-6));
}
