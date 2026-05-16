use apple_accelerate::{cos_f32, exp_f32, log_f32, sin_f32, sqrt_f32};

fn main() {
    let angles = [0.0_f32, std::f32::consts::FRAC_PI_2];
    let sines = sin_f32(&angles).expect("sin");
    let cosines = cos_f32(&angles).expect("cos");
    assert!(sines[0].abs() < 1.0e-6);
    assert!((sines[1] - 1.0).abs() < 1.0e-5);
    assert!((cosines[0] - 1.0).abs() < 1.0e-5);

    let exponents = exp_f32(&[0.0, 1.0]).expect("exp");
    let logs = log_f32(&[1.0, std::f32::consts::E]).expect("log");
    let roots = sqrt_f32(&[1.0, 4.0, 9.0]).expect("sqrt");
    assert!((exponents[1] - std::f32::consts::E).abs() < 1.0e-5);
    assert!((logs[1] - 1.0).abs() < 1.0e-5);
    assert!(roots
        .iter()
        .zip([1.0_f32, 2.0, 3.0])
        .all(|(actual, expected)| (*actual - expected).abs() < 1.0e-6));

    println!("vforce smoke passed: sines={sines:?} exponents={exponents:?}");
}
