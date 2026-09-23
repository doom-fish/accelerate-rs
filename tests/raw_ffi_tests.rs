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
