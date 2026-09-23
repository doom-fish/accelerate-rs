#![cfg(feature = "raw-ffi")]

use apple_accelerate::ffi;
use core::ffi::c_void;
use core::ptr;

unsafe extern "C" fn square(_: *mut c_void, n: usize, x: *const f64, y: *mut f64) {
    let xs = unsafe { core::slice::from_raw_parts(x, n) };
    let ys = unsafe { core::slice::from_raw_parts_mut(y, n) };
    for (input, output) in xs.iter().zip(ys) {
        *output = input * input;
    }
}

fn integrate_raw(options: &ffi::quadrature_integrate_options) -> (f64, ffi::quadrature_status) {
    let function = ffi::quadrature_integrate_function {
        fun: Some(square),
        fun_arg: ptr::null_mut(),
    };
    let mut status = ffi::quadrature_status(i32::MIN);
    let mut abs_error = 0.0_f64;
    let integral = unsafe {
        ffi::quadrature_integrate(
            &raw const function,
            0.0,
            1.0,
            options,
            &raw mut status,
            &raw mut abs_error,
            0,
            ptr::null_mut(),
        )
    };
    (integral, status)
}

#[test]
fn raw_quadrature_status_is_an_integer_newtype() {
    let options = ffi::quadrature_integrate_options {
        integrator: ffi::quadrature_integrator::QUADRATURE_INTEGRATE_QNG,
        abs_tolerance: 1.0e-10,
        rel_tolerance: 1.0e-10,
        qag_points_per_interval: 0,
        max_intervals: 0,
    };
    let (integral, status) = integrate_raw(&options);
    assert_eq!(status, ffi::quadrature_status::QUADRATURE_SUCCESS);
    assert!((integral - 1.0 / 3.0).abs() < 1.0e-9);

    let invalid = ffi::quadrature_integrate_options {
        integrator: ffi::quadrature_integrator::QUADRATURE_INTEGRATE_QAG,
        qag_points_per_interval: 7,
        max_intervals: 16,
        ..options
    };
    let (_, status) = integrate_raw(&invalid);
    assert_ne!(status, ffi::quadrature_status::QUADRATURE_SUCCESS);
    assert!(status.0 < 0);
}

#[test]
fn raw_vbignum_functions_take_aligned_unions_by_pointer() {
    let mut a = ffi::vU256::default();
    let mut b = ffi::vU256::default();
    let mut sum = ffi::vU256::default();
    a.s.LSW = u32::MAX;
    b.s.LSW = 1;
    unsafe { ffi::vU256Add(&raw const a, &raw const b, &raw mut sum) };
    let words = unsafe { sum.s };
    assert_eq!(words.LSW, 0);
    assert_eq!(words.d7, 1);
    assert_eq!(words.MSW, 0);
    assert_eq!(core::mem::align_of::<ffi::vU256>(), 16);
    assert_eq!(core::mem::size_of::<ffi::vU256>(), 32);
}

#[test]
fn raw_simd_float4_matches_the_c_layout() {
    assert_eq!(core::mem::size_of::<ffi::simd_float4>(), 16);
    assert_eq!(core::mem::align_of::<ffi::simd_float4>(), 16);
}

fn raw_fft_zop(radix: i32, factor: usize) -> (Vec<f32>, Vec<f32>) {
    let log2n = 2;
    let count = factor << log2n;
    let mut input_real: Vec<f32> = (0..count)
        .map(|value| f32::from(u8::try_from(value).expect("small length")))
        .collect();
    let mut input_imag = vec![0.0_f32; count];
    let mut output_real = vec![0.0_f32; count];
    let mut output_imag = vec![0.0_f32; count];
    let input = ffi::DSPSplitComplex {
        realp: input_real.as_mut_ptr(),
        imagp: input_imag.as_mut_ptr(),
    };
    let output = ffi::DSPSplitComplex {
        realp: output_real.as_mut_ptr(),
        imagp: output_imag.as_mut_ptr(),
    };
    let setup = unsafe { ffi::vDSP_create_fftsetup(log2n, radix) };
    assert!(!setup.is_null());
    unsafe {
        if factor == 3 {
            ffi::vDSP_fft3_zop(setup, &raw const input, 1, &raw const output, 1, log2n, 1);
        } else {
            ffi::vDSP_fft5_zop(setup, &raw const input, 1, &raw const output, 1, log2n, 1);
        }
        ffi::vDSP_destroy_fftsetup(setup);
    }
    (output_real, output_imag)
}

#[test]
fn raw_radix3_and_radix5_transforms_are_declared() {
    let (real, imag) = raw_fft_zop(apple_accelerate::fft_radix::RADIX3, 3);
    assert!((real[0] - 66.0).abs() < 1.0e-3);
    assert!(imag[0].abs() < 1.0e-3);
    assert!((real[1] + 6.0).abs() < 1.0e-3);
    assert!((imag[1] - 6.0 / (std::f32::consts::PI / 12.0).tan()).abs() < 1.0e-3);

    let (real, imag) = raw_fft_zop(apple_accelerate::fft_radix::RADIX5, 5);
    assert!((real[0] - 190.0).abs() < 1.0e-3);
    assert!((real[1] + 10.0).abs() < 1.0e-3);
    assert!((imag[1] - 10.0 / (std::f32::consts::PI / 20.0).tan()).abs() < 1.0e-3);
}
