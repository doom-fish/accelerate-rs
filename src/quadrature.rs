use crate::bridge;
use crate::error::{Error, Result};
use core::ffi::c_void;
use core::slice;
use doom_fish_utils::panic_safe::catch_user_panic_result;

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

const QAGS_WORKSPACE_PER_INTERVAL: usize = 152;

struct Integrand<'f> {
    function: &'f mut dyn FnMut(f64) -> f64,
    panicked: bool,
}

unsafe extern "C" fn quadrature_trampoline(
    context: *mut c_void,
    n: usize,
    x: *const f64,
    y: *mut f64,
) {
    if context.is_null() || x.is_null() || y.is_null() || n == 0 {
        return;
    }
    // SAFETY: `context` points to the `Integrand` owned by the `integrate` frame blocked in this synchronous call.
    let integrand = unsafe { &mut *context.cast::<Integrand<'_>>() };
    // SAFETY: `x` is guaranteed by Accelerate to contain `n` valid f64 values for the duration of this callback.
    let xs = unsafe { slice::from_raw_parts(x, n) };
    // SAFETY: `y` is guaranteed by Accelerate to contain `n` writable f64 values for the duration of this callback.
    let ys = unsafe { slice::from_raw_parts_mut(y, n) };
    // A panic in user code must never unwind across the `extern "C"` boundary
    // into Accelerate (undefined behaviour). After a panic the closure is not
    // called again: every remaining value is NaN and `integrate` returns an error.
    if !integrand.panicked {
        let function = &mut *integrand.function;
        let completed = catch_user_panic_result("apple_accelerate::quadrature::integrate", || {
            for (input, output) in xs.iter().copied().zip(ys.iter_mut()) {
                *output = function(input);
            }
        });
        if completed.is_some() {
            return;
        }
        integrand.panicked = true;
    }
    ys.fill(f64::NAN);
}

/// Wraps `quadrature_integrate` for a Rust closure over `[a, b]`.
pub fn integrate<F>(mut f: F, a: f64, b: f64, options: Options) -> Result<QuadratureOutput>
where
    F: FnMut(f64) -> f64,
{
    if options
        .max_intervals
        .checked_mul(QAGS_WORKSPACE_PER_INTERVAL)
        .and_then(|bytes| isize::try_from(bytes).ok())
        .is_none()
    {
        return Err(Error::InvalidValue(
            "quadrature max_intervals exceeds the addressable workspace",
        ));
    }

    let mut integrand = Integrand {
        function: &mut f,
        panicked: false,
    };
    let mut status = 0_i32;
    let mut abs_error = 0.0_f64;

    // SAFETY: The callback pointer is valid for the duration of this synchronous bridge call.
    let integral = unsafe {
        bridge::acc_quadrature_integrate(
            Some(quadrature_trampoline),
            (&raw mut integrand).cast(),
            a,
            b,
            options.integrator.as_raw(),
            options.abs_tolerance,
            options.rel_tolerance,
            options.qag_points_per_interval,
            options.max_intervals,
            &raw mut status,
            &raw mut abs_error,
        )
    };

    if integrand.panicked {
        return Err(Error::IntegrandPanicked);
    }
    if status == 0 {
        Ok(QuadratureOutput {
            integral,
            abs_error,
        })
    } else {
        Err(Error::QuadratureStatus(status))
    }
}
