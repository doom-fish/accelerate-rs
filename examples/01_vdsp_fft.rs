use apple_accelerate::{
    add_f32, blackman_window, dot_f32, fft_direction, fft_radix, hamming_window, FftSetup,
};

fn main() {
    let added = add_f32(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]).expect("vector add failed");
    assert!(added
        .iter()
        .zip([5.0_f32, 7.0, 9.0])
        .all(|(actual, expected)| (*actual - expected).abs() < 1.0e-6));

    let dot = dot_f32(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]).expect("dot product failed");
    assert!((dot - 32.0).abs() < 1.0e-6);

    let hamm = hamming_window(8, 0);
    let blk = blackman_window(8, 0);
    assert_eq!(hamm.len(), 8);
    assert_eq!(blk.len(), 8);

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

    println!("vdsp smoke passed: added={added:?} dot={dot} fft_real={real:?}");
}
