use apple_accelerate::{integrate, QuadratureOptions};

fn main() {
    let result = integrate(|x| x * x, 0.0, 1.0, QuadratureOptions::default()).expect("integrate");
    assert!((result.integral - (1.0 / 3.0)).abs() < 1.0e-9);
    println!(
        "quadrature smoke passed: integral={} error={}",
        result.integral, result.abs_error
    );
}
