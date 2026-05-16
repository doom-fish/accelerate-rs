use apple_accelerate::{
    add_f32, add_f64, blackman_window, blackman_window_f64, dot_f32, dot_f64, fft_direction,
    fft_radix, hamming_window, hamming_window_f64, FftSetup,
};

fn main() {
    let added = add_f32(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]).expect("vector add failed");
    assert!(added
        .iter()
        .zip([5.0_f32, 7.0, 9.0])
        .all(|(actual, expected)| (*actual - expected).abs() < 1.0e-6));

    let added64 = add_f64(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]).expect("vector add f64 failed");
    assert!(added64
        .iter()
        .zip([5.0_f64, 7.0, 9.0])
        .all(|(actual, expected)| (*actual - expected).abs() < 1.0e-12));

    let dot = dot_f32(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]).expect("dot product failed");
    assert!((dot - 32.0).abs() < 1.0e-6);

    let dot64 = dot_f64(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]).expect("dot product f64 failed");
    assert!((dot64 - 32.0).abs() < 1.0e-12);

    let hamm = hamming_window(8, 0);
    let hamm64 = hamming_window_f64(8, 0);
    let blk = blackman_window(8, 0);
    let blk64 = blackman_window_f64(8, 0);
    assert_eq!(hamm.len(), 8);
    assert_eq!(hamm64.len(), 8);
    assert_eq!(blk.len(), 8);
    assert_eq!(blk64.len(), 8);

    let setup = FftSetup::new(2, fft_radix::RADIX2).expect("failed to create FFT setup");
    let mut real = vec![1.0_f32, 0.0, 0.0, 0.0];
    let mut imag = vec![0.0_f32; 4];
    setup
        .fft_zip(&mut real, &mut imag, 2, fft_direction::FORWARD)
        .expect("fft failed");

    for value in &real {
        assert!(
            (*value - 1.0).abs() < 1.0e-5,
            "unexpected FFT real output: {real:?}"
        );
    }
    for value in &imag {
        assert!(value.abs() < 1.0e-5, "unexpected FFT imag output: {imag:?}");
    }

    println!(
        "vdsp smoke passed: added={added:?} added64={added64:?} dot={dot} dot64={dot64} fft_real={real:?}"
    );
}
