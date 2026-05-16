use apple_accelerate::{lu_decompose_f32, solve_linear_system_f32};

fn main() {
    // Column-major 2x2 matrix [[3, 1], [1, 2]].
    let matrix = [3.0_f32, 1.0, 1.0, 2.0];
    let lu = lu_decompose_f32(&matrix, 2).expect("lu");
    assert_eq!(lu.dimension(), 2);
    assert_eq!(lu.factors().len(), 4);

    let solution = solve_linear_system_f32(&matrix, 2, &[9.0_f32, 8.0]).expect("solve");
    assert!((solution[0] - 2.0).abs() < 1.0e-5);
    assert!((solution[1] - 3.0).abs() < 1.0e-5);

    println!(
        "lapack smoke passed: pivots={:?} solution={solution:?}",
        lu.pivots()
    );
}
