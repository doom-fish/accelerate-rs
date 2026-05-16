#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]

pub mod blas;
pub mod bnns;
pub mod error;
pub mod ffi;
pub mod vdsp;
pub mod vimage;

pub use crate::blas::{blas_order, blas_transpose, sdot, sgemm_row_major, sgemv_row_major};
pub use crate::bnns::Filter as BnnsFilter;
pub use crate::error::{Error, Result};
pub use crate::vdsp::{
    add_f32, blackman_window, dot_f32, fft_direction, fft_radix, hamming_window, max_f32, mean_f32,
    min_f32, sub_f32, sum_f32, window_flags, BiquadSetup, FftSetup,
};
pub use crate::vimage::{
    box_convolve_argb8888, contrast_stretch_planar8, rotate_argb8888, scale_argb8888, vimage_flags,
    ImageBuffer,
};
