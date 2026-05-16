#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]

mod bridge;
#[path = "ffi/mod.rs"]
mod raw_ffi;

pub mod blas;
pub mod bnns;
pub mod error;
pub mod lapack;
pub mod quadrature;
pub mod simd;
pub mod sparse;
pub mod vdsp;
pub mod vforce;
pub mod vimage;

#[cfg(feature = "raw-ffi")]
#[cfg_attr(docsrs, doc(cfg(feature = "raw-ffi")))]
pub mod ffi {
    pub use crate::raw_ffi::*;
}

pub use crate::blas::{blas_order, blas_transpose, sdot, sgemm_row_major, sgemv_row_major};
pub use crate::bnns::{
    relu_f32 as bnns_relu_f32, sigmoid_f32 as bnns_sigmoid_f32, Filter as BnnsFilter,
};
pub use crate::error::{Error, Result};
pub use crate::lapack::{lu_decompose_f32, solve_linear_system_f32, LuDecompositionF32};
pub use crate::quadrature::{
    integrate, Integrator as QuadratureIntegrator, Options as QuadratureOptions, QuadratureOutput,
};
pub use crate::simd::{add_f32x4, dot_f32x4, length_f32x4, normalize_f32x4, Float4};
pub use crate::sparse::{
    add_to_dense_f32 as sparse_add_to_dense_f32, dot_dense_f32 as sparse_dot_dense_f32,
    dot_sparse_f32 as sparse_dot_sparse_f32, SparseIndex,
};
pub use crate::vdsp::{
    add_f32, blackman_window, dot_f32, fft_direction, fft_radix, hamming_window, max_f32, mean_f32,
    min_f32, sub_f32, sum_f32, window_flags, BiquadSetup, FftSetup,
};
pub use crate::vforce::{cos_f32, exp_f32, log_f32, sin_f32, sqrt_f32};
pub use crate::vimage::{
    box_convolve_argb8888, contrast_stretch_planar8, rotate_argb8888, scale_argb8888, vimage_flags,
    ImageBuffer,
};
