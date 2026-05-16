use apple_accelerate::{sdot, sgemv_row_major};

fn main() {
    let dot = sdot(&[1.0_f32, 2.0, 3.0], &[4.0_f32, 5.0, 6.0]).expect("sdot");
    assert!((dot - 32.0).abs() < 1.0e-6);

    let matrix = [1.0_f32, 2.0, 3.0, 4.0, 5.0, 6.0];
    let vector = [1.0_f32, 0.5, -1.0];
    let mut output = vec![0.0_f32; 2];
    sgemv_row_major(2, 3, 1.0, &matrix, &vector, 0.0, &mut output).expect("sgemv");
    assert!((output[0] - (-1.0)).abs() < 1.0e-6);
    assert!((output[1] - 0.5).abs() < 1.0e-6);

    println!("blas smoke passed: dot={dot} gemv={output:?}");
}
