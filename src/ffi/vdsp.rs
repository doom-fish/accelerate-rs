#![allow(missing_docs, non_camel_case_types, non_upper_case_globals)]

/// Raw FFI type alias for `vDSP_Length`.
pub type vDSP_Length = usize;
/// Raw FFI type alias for `vDSP_Stride`.
pub type vDSP_Stride = isize;
/// Raw FFI type alias for `FFTDirection`.
pub type FFTDirection = i32;
/// Raw FFI type alias for `FFTRadix`.
pub type FFTRadix = i32;

/// Raw FFI opaque handle type for `OpaqueFFTSetup`.
pub enum OpaqueFFTSetup {}
/// Raw FFI type alias for `FFTSetup`.
pub type FFTSetup = *mut OpaqueFFTSetup;

/// Raw FFI opaque handle type for `vDSP_biquad_SetupStruct`.
pub enum vDSP_biquad_SetupStruct {}
/// Raw FFI type alias for `vDSP_biquad_Setup`.
pub type vDSP_biquad_Setup = *mut vDSP_biquad_SetupStruct;

/// Raw FFI struct for `DSPSplitComplex`.
#[repr(C)]
pub struct DSPSplitComplex {
    pub realp: *mut f32,
    pub imagp: *mut f32,
}

#[link(name = "Accelerate", kind = "framework")]
unsafe extern "C" {
    /// Raw FFI declaration for `vDSP_create_fftsetup`.
    pub fn vDSP_create_fftsetup(log2n: vDSP_Length, radix: FFTRadix) -> FFTSetup;
    /// Raw FFI declaration for `vDSP_destroy_fftsetup`.
    pub fn vDSP_destroy_fftsetup(setup: FFTSetup);
    /// Raw FFI declaration for `vDSP_fft_zip`.
    pub fn vDSP_fft_zip(
        setup: FFTSetup,
        split_complex: *const DSPSplitComplex,
        stride: vDSP_Stride,
        log2n: vDSP_Length,
        direction: FFTDirection,
    );
    /// Raw FFI declaration for `vDSP_biquad_CreateSetup`.
    pub fn vDSP_biquad_CreateSetup(
        coefficients: *const f64,
        sections: vDSP_Length,
    ) -> vDSP_biquad_Setup;
    /// Raw FFI declaration for `vDSP_biquad_DestroySetup`.
    pub fn vDSP_biquad_DestroySetup(setup: vDSP_biquad_Setup);
    /// Raw FFI declaration for `vDSP_biquad`.
    pub fn vDSP_biquad(
        setup: vDSP_biquad_Setup,
        delay: *mut f32,
        input: *const f32,
        input_stride: vDSP_Stride,
        output: *mut f32,
        output_stride: vDSP_Stride,
        length: vDSP_Length,
    );
    /// Raw FFI declaration for `vDSP_vadd`.
    pub fn vDSP_vadd(
        a: *const f32,
        a_stride: vDSP_Stride,
        b: *const f32,
        b_stride: vDSP_Stride,
        c: *mut f32,
        c_stride: vDSP_Stride,
        length: vDSP_Length,
    );
    /// Raw FFI declaration for `vDSP_vaddD`.
    pub fn vDSP_vaddD(
        a: *const f64,
        a_stride: vDSP_Stride,
        b: *const f64,
        b_stride: vDSP_Stride,
        c: *mut f64,
        c_stride: vDSP_Stride,
        length: vDSP_Length,
    );
    /// Raw FFI declaration for `vDSP_vsub`.
    pub fn vDSP_vsub(
        b: *const f32,
        b_stride: vDSP_Stride,
        a: *const f32,
        a_stride: vDSP_Stride,
        c: *mut f32,
        c_stride: vDSP_Stride,
        length: vDSP_Length,
    );
    /// Raw FFI declaration for `vDSP_vsubD`.
    pub fn vDSP_vsubD(
        b: *const f64,
        b_stride: vDSP_Stride,
        a: *const f64,
        a_stride: vDSP_Stride,
        c: *mut f64,
        c_stride: vDSP_Stride,
        length: vDSP_Length,
    );
    /// Raw FFI declaration for `vDSP_dotpr`.
    pub fn vDSP_dotpr(
        a: *const f32,
        a_stride: vDSP_Stride,
        b: *const f32,
        b_stride: vDSP_Stride,
        result: *mut f32,
        length: vDSP_Length,
    );
    /// Raw FFI declaration for `vDSP_dotprD`.
    pub fn vDSP_dotprD(
        a: *const f64,
        a_stride: vDSP_Stride,
        b: *const f64,
        b_stride: vDSP_Stride,
        result: *mut f64,
        length: vDSP_Length,
    );
    /// Raw FFI declaration for `vDSP_maxv`.
    pub fn vDSP_maxv(input: *const f32, stride: vDSP_Stride, result: *mut f32, length: vDSP_Length);
    /// Raw FFI declaration for `vDSP_maxvD`.
    pub fn vDSP_maxvD(
        input: *const f64,
        stride: vDSP_Stride,
        result: *mut f64,
        length: vDSP_Length,
    );
    /// Raw FFI declaration for `vDSP_minv`.
    pub fn vDSP_minv(input: *const f32, stride: vDSP_Stride, result: *mut f32, length: vDSP_Length);
    /// Raw FFI declaration for `vDSP_minvD`.
    pub fn vDSP_minvD(
        input: *const f64,
        stride: vDSP_Stride,
        result: *mut f64,
        length: vDSP_Length,
    );
    /// Raw FFI declaration for `vDSP_meanv`.
    pub fn vDSP_meanv(
        input: *const f32,
        stride: vDSP_Stride,
        result: *mut f32,
        length: vDSP_Length,
    );
    /// Raw FFI declaration for `vDSP_meanvD`.
    pub fn vDSP_meanvD(
        input: *const f64,
        stride: vDSP_Stride,
        result: *mut f64,
        length: vDSP_Length,
    );
    /// Raw FFI declaration for `vDSP_sve`.
    pub fn vDSP_sve(input: *const f32, stride: vDSP_Stride, result: *mut f32, length: vDSP_Length);
    /// Raw FFI declaration for `vDSP_sveD`.
    pub fn vDSP_sveD(input: *const f64, stride: vDSP_Stride, result: *mut f64, length: vDSP_Length);
    /// Raw FFI declaration for `vDSP_hamm_window`.
    pub fn vDSP_hamm_window(output: *mut f32, length: vDSP_Length, flags: i32);
    /// Raw FFI declaration for `vDSP_hamm_windowD`.
    pub fn vDSP_hamm_windowD(output: *mut f64, length: vDSP_Length, flags: i32);
    /// Raw FFI declaration for `vDSP_blkman_window`.
    pub fn vDSP_blkman_window(output: *mut f32, length: vDSP_Length, flags: i32);
    /// Raw FFI declaration for `vDSP_blkman_windowD`.
    pub fn vDSP_blkman_windowD(output: *mut f64, length: vDSP_Length, flags: i32);
}

#[allow(
    missing_docs,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_imports,
    dead_code,
    improper_ctypes,
    improper_ctypes_definitions,
    unnecessary_transmutes
)]
#[allow(clippy::all, clippy::pedantic, clippy::nursery, clippy::style, clippy::complexity, clippy::correctness, clippy::perf)]
mod generated {
    #![allow(clippy::all, clippy::pedantic, clippy::nursery, clippy::style, clippy::complexity, clippy::correctness, clippy::perf)]
    use super::*;
    include!("generated/vdsp_missing.rs");
}

pub use generated::*;
