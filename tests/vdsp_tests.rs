use apple_accelerate::{
    add_f32, add_f64, blackman_window, blackman_window_f64, dot_f32, dot_f64, fft_direction,
    fft_radix, hamming_window, hamming_window_f64, sub_f32, sub_f64, FftSetup,
};

#[test]
fn vector_ops_and_windows_smoke() {
    let add = add_f32(&[1.0_f32, 2.0], &[3.0_f32, 4.0]).expect("add");
    assert!(add
        .iter()
        .zip([4.0_f32, 6.0])
        .all(|(actual, expected)| (*actual - expected).abs() < 1.0e-6));

    let add64 = add_f64(&[1.0_f64, 2.0], &[3.0_f64, 4.0]).expect("add64");
    assert!(add64
        .iter()
        .zip([4.0_f64, 6.0])
        .all(|(actual, expected)| (*actual - expected).abs() < 1.0e-12));

    let sub = sub_f32(&[5.0_f32, 7.0], &[3.0_f32, 2.0]).expect("sub");
    assert!(sub
        .iter()
        .zip([2.0_f32, 5.0])
        .all(|(actual, expected)| (*actual - expected).abs() < 1.0e-6));

    let sub64 = sub_f64(&[5.0_f64, 7.0], &[3.0_f64, 2.0]).expect("sub64");
    assert!(sub64
        .iter()
        .zip([2.0_f64, 5.0])
        .all(|(actual, expected)| (*actual - expected).abs() < 1.0e-12));

    let dot = dot_f32(&[1.0_f32, 2.0, 3.0], &[4.0_f32, 5.0, 6.0]).expect("dot");
    assert!((dot - 32.0).abs() < 1.0e-6);

    let dot64 = dot_f64(&[1.0_f64, 2.0, 3.0], &[4.0_f64, 5.0, 6.0]).expect("dot64");
    assert!((dot64 - 32.0).abs() < 1.0e-12);

    assert_eq!(hamming_window(16, 0).expect("hamming").len(), 16);
    assert_eq!(blackman_window(16, 0).expect("blackman").len(), 16);
    assert_eq!(hamming_window_f64(16, 0).expect("hamming f64").len(), 16);
    assert_eq!(blackman_window_f64(16, 0).expect("blackman f64").len(), 16);
}

#[test]
fn vector_op_length_mismatch_errors() {
    let error = add_f32(&[1.0_f32], &[1.0_f32, 2.0]).expect_err("length mismatch");
    assert!(error.to_string().contains("invalid length"));
}

#[test]
fn window_functions_report_values_and_accept_empty_lengths() {
    let hamming = hamming_window(4, 0).expect("hamming");
    assert!((hamming[0] - 0.08).abs() < 1.0e-6);
    assert!((hamming[2] - 1.0).abs() < 1.0e-6);
    assert!((hamming[1] - hamming[3]).abs() < 1.0e-6);
    let blackman = blackman_window_f64(4, 0).expect("blackman f64");
    assert!(blackman[0].abs() < 1.0e-12);
    assert!((blackman[2] - 1.0).abs() < 1.0e-12);
    assert!(hamming_window(0, 0).expect("empty").is_empty());
    assert!(blackman_window(0, 0).expect("empty").is_empty());
}

#[test]
fn fft_setup_rejects_sizes_and_radices_accelerate_cannot_handle() {
    assert!(FftSetup::new(usize::MAX, fft_radix::RADIX2).is_none());
    assert!(FftSetup::new(63, fft_radix::RADIX2).is_none());
    assert!(FftSetup::new(62, fft_radix::RADIX5).is_none());
    assert!(FftSetup::new(41, fft_radix::RADIX2).is_none());
    assert!(FftSetup::new(4, 3).is_none());
    assert!(FftSetup::new(4, -1).is_none());
    assert!(FftSetup::new(0, fft_radix::RADIX2).is_some());
}

#[test]
fn fft_zip_rejects_unknown_directions() {
    let setup = FftSetup::new(2, fft_radix::RADIX2).expect("setup");
    let mut real = vec![1.0_f32, 0.0, 0.0, 0.0];
    let mut imag = vec![0.0_f32; 4];
    for direction in [0, 2, -2, i32::MIN] {
        assert!(setup.fft_zip(&mut real, &mut imag, 2, direction).is_err());
    }
    assert_eq!(real, vec![1.0, 0.0, 0.0, 0.0]);
    setup
        .fft_zip(&mut real, &mut imag, 2, fft_direction::INVERSE)
        .expect("inverse");
}

#[test]
fn fft_zip_matches_across_radix_setups() {
    let input_real: Vec<f32> = (0_u8..16).map(|value| f32::from(value) * 0.25).collect();
    let input_imag: Vec<f32> = (0_u8..16)
        .map(|value| f32::from(16 - value) * 0.5)
        .collect();
    let mut results = Vec::new();
    for radix in [fft_radix::RADIX2, fft_radix::RADIX3, fft_radix::RADIX5] {
        let setup = FftSetup::new(4, radix).expect("setup");
        let mut real = input_real.clone();
        let mut imag = input_imag.clone();
        setup
            .fft_zip(&mut real, &mut imag, 4, fft_direction::FORWARD)
            .expect("forward");
        results.push((real, imag));
    }
    assert!((results[0].0[0] - 30.0).abs() < 1.0e-4);
    assert_eq!(results[0], results[1]);
    assert_eq!(results[0], results[2]);
}
