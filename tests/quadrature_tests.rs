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

#[test]
fn quadrature_panicking_integrand_is_an_error() {
    let error = integrate(
        |x| {
            assert!(x < 0.5, "integrand rejects {x}");
            x
        },
        0.0,
        1.0,
        QuadratureOptions::default(),
    )
    .expect_err("panicking integrand");
    assert_eq!(error, apple_accelerate::Error::IntegrandPanicked);
}

#[test]
fn quadrature_integrand_is_not_called_after_it_panics() {
    let mut calls_before_panic = 0_usize;
    let mut calls_after_panic = 0_usize;
    let mut panicked = false;
    let options = QuadratureOptions {
        integrator: QuadratureIntegrator::Qags,
        abs_tolerance: 1.0e-12,
        rel_tolerance: 1.0e-12,
        qag_points_per_interval: 0,
        max_intervals: 64,
    };
    let result = integrate(
        |x| {
            if panicked {
                calls_after_panic += 1;
                return x;
            }
            calls_before_panic += 1;
            if calls_before_panic == 5 {
                panicked = true;
                panic!("integrand failure");
            }
            x
        },
        0.0,
        1.0,
        options,
    );
    assert_eq!(result, Err(apple_accelerate::Error::IntegrandPanicked));
    assert!(panicked);
    assert_eq!(calls_before_panic, 5);
    assert_eq!(calls_after_panic, 0);
}

#[test]
fn quadrature_accepts_borrowing_closures() {
    let mut evaluations = 0_usize;
    let scale = 3.0_f64;
    let output = integrate(
        |x| {
            evaluations += 1;
            scale * x
        },
        0.0,
        2.0,
        QuadratureOptions::default(),
    )
    .expect("integrate");
    assert!((output.integral - 6.0).abs() < 1.0e-9);
    assert!(evaluations > 0);
}

#[test]
fn quadrature_rejects_unaddressable_interval_counts() {
    let options = QuadratureOptions {
        integrator: QuadratureIntegrator::Qags,
        max_intervals: usize::MAX / 64,
        ..QuadratureOptions::default()
    };
    let error = integrate(|x| x, 0.0, 1.0, options).expect_err("huge max_intervals");
    assert!(matches!(error, apple_accelerate::Error::InvalidValue(_)));
}
