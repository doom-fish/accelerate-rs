use apple_accelerate::{integrate, QuadratureIntegrator, QuadratureOptions};

#[test]
fn quadrature_integrates_polynomial() {
    let output = integrate(|x| x * x, 0.0, 1.0, QuadratureOptions::default()).expect("integrate");
    assert!((output.integral - (1.0 / 3.0)).abs() < 1.0e-9);
    assert!(output.abs_error < 1.0e-9);
}

#[test]
fn quadrature_supports_qag() {
    let options = QuadratureOptions {
        integrator: QuadratureIntegrator::Qag,
        abs_tolerance: 1.0e-10,
        rel_tolerance: 1.0e-10,
        qag_points_per_interval: 21,
        max_intervals: 64,
    };
    let output = integrate(f64::sin, 0.0, std::f64::consts::PI, options).expect("integrate");
    assert!((output.integral - 2.0).abs() < 1.0e-9);
}
