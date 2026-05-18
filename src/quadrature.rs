use crate::bridge;
use crate::error::{Error, Result};
use core::ffi::c_void;
use core::slice;

/// Selects the Accelerate integrator passed to `quadrature_integrate`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Integrator {
    Qng,
    Qag,
    Qags,
}

impl Integrator {
    const fn as_raw(self) -> i32 {
        match self {
            Self::Qng => 0,
            Self::Qag => 1,
            Self::Qags => 2,
        }
    }
}

/// Options passed to `quadrature_integrate`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Options {
    pub integrator: Integrator,
    pub abs_tolerance: f64,
    pub rel_tolerance: f64,
    pub qag_points_per_interval: usize,
    pub max_intervals: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            integrator: Integrator::Qng,
            abs_tolerance: 1.0e-12,
            rel_tolerance: 1.0e-12,
            qag_points_per_interval: 0,
            max_intervals: 0,
        }
    }
}

/// Output values returned by `quadrature_integrate`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QuadratureOutput {
    pub integral: f64,
    pub abs_error: f64,
}

unsafe extern "C" fn quadrature_trampoline(
    context: *mut c_void,
    n: usize,
    x: *const f64,
    y: *mut f64,
) {
    // SAFETY: `context` is a valid pointer to `Box<dyn FnMut(f64) -> f64>` injected by `integrate`.
    let callback = unsafe { &mut *context.cast::<Box<dyn FnMut(f64) -> f64>>() };
    // SAFETY: `x` is guaranteed by Accelerate to contain `n` valid f64 values for the duration of this callback.
    let xs = unsafe { slice::from_raw_parts(x, n) };
    // SAFETY: `y` is guaranteed by Accelerate to contain `n` writable f64 values for the duration of this callback.
    let ys = unsafe { slice::from_raw_parts_mut(y, n) };
    for (input, output) in xs.iter().copied().zip(ys.iter_mut()) {
        *output = callback.as_mut()(input);
    }
}

/// Wraps `quadrature_integrate` for a Rust closure over `[a, b]`.
pub fn integrate<F>(f: F, a: f64, b: f64, options: Options) -> Result<QuadratureOutput>
where
    F: FnMut(f64) -> f64 + 'static,
{
    let mut callback: Box<dyn FnMut(f64) -> f64> = Box::new(f);
    let mut status = 0_i32;
    let mut abs_error = 0.0_f64;

    // SAFETY: The callback pointer is valid for the duration of this synchronous bridge call.
    let integral = unsafe {
        bridge::acc_quadrature_integrate(
            Some(quadrature_trampoline),
            std::ptr::addr_of_mut!(callback).cast(),
            a,
            b,
            options.integrator.as_raw(),
            options.abs_tolerance,
            options.rel_tolerance,
            options.qag_points_per_interval,
            options.max_intervals,
            &mut status,
            &mut abs_error,
        )
    };

    if status == 0 {
        Ok(QuadratureOutput {
            integral,
            abs_error,
        })
    } else {
        Err(Error::QuadratureStatus(status))
    }
}
