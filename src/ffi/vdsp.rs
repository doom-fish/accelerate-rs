#![allow(missing_docs, non_camel_case_types, non_upper_case_globals)]

pub type vDSP_Length = usize;
pub type vDSP_Stride = isize;
pub type FFTDirection = i32;
pub type FFTRadix = i32;

pub enum OpaqueFFTSetup {}
pub type FFTSetup = *mut OpaqueFFTSetup;

pub enum vDSP_biquad_SetupStruct {}
pub type vDSP_biquad_Setup = *mut vDSP_biquad_SetupStruct;

#[repr(C)]
pub struct DSPSplitComplex {
    pub realp: *mut f32,
    pub imagp: *mut f32,
}

#[link(name = "Accelerate", kind = "framework")]
unsafe extern "C" {
    pub fn vDSP_create_fftsetup(log2n: vDSP_Length, radix: FFTRadix) -> FFTSetup;
    pub fn vDSP_destroy_fftsetup(setup: FFTSetup);
    pub fn vDSP_fft_zip(
        setup: FFTSetup,
        split_complex: *const DSPSplitComplex,
        stride: vDSP_Stride,
        log2n: vDSP_Length,
        direction: FFTDirection,
    );
    pub fn vDSP_biquad_CreateSetup(
        coefficients: *const f64,
        sections: vDSP_Length,
    ) -> vDSP_biquad_Setup;
    pub fn vDSP_biquad_DestroySetup(setup: vDSP_biquad_Setup);
    pub fn vDSP_biquad(
        setup: vDSP_biquad_Setup,
        delay: *mut f32,
        input: *const f32,
        input_stride: vDSP_Stride,
        output: *mut f32,
        output_stride: vDSP_Stride,
        length: vDSP_Length,
    );
    pub fn vDSP_vadd(
        a: *const f32,
        a_stride: vDSP_Stride,
        b: *const f32,
        b_stride: vDSP_Stride,
        c: *mut f32,
        c_stride: vDSP_Stride,
        length: vDSP_Length,
    );
    pub fn vDSP_vaddD(
        a: *const f64,
        a_stride: vDSP_Stride,
        b: *const f64,
        b_stride: vDSP_Stride,
        c: *mut f64,
        c_stride: vDSP_Stride,
        length: vDSP_Length,
    );
    pub fn vDSP_vsub(
        b: *const f32,
        b_stride: vDSP_Stride,
        a: *const f32,
        a_stride: vDSP_Stride,
        c: *mut f32,
        c_stride: vDSP_Stride,
        length: vDSP_Length,
    );
    pub fn vDSP_vsubD(
        b: *const f64,
        b_stride: vDSP_Stride,
        a: *const f64,
        a_stride: vDSP_Stride,
        c: *mut f64,
        c_stride: vDSP_Stride,
        length: vDSP_Length,
    );
    pub fn vDSP_dotpr(
        a: *const f32,
        a_stride: vDSP_Stride,
        b: *const f32,
        b_stride: vDSP_Stride,
        result: *mut f32,
        length: vDSP_Length,
    );
    pub fn vDSP_dotprD(
        a: *const f64,
        a_stride: vDSP_Stride,
        b: *const f64,
        b_stride: vDSP_Stride,
        result: *mut f64,
        length: vDSP_Length,
    );
    pub fn vDSP_maxv(input: *const f32, stride: vDSP_Stride, result: *mut f32, length: vDSP_Length);
    pub fn vDSP_maxvD(
        input: *const f64,
        stride: vDSP_Stride,
        result: *mut f64,
        length: vDSP_Length,
    );
    pub fn vDSP_minv(input: *const f32, stride: vDSP_Stride, result: *mut f32, length: vDSP_Length);
    pub fn vDSP_minvD(
        input: *const f64,
        stride: vDSP_Stride,
        result: *mut f64,
        length: vDSP_Length,
    );
    pub fn vDSP_meanv(
        input: *const f32,
        stride: vDSP_Stride,
        result: *mut f32,
        length: vDSP_Length,
    );
    pub fn vDSP_meanvD(
        input: *const f64,
        stride: vDSP_Stride,
        result: *mut f64,
        length: vDSP_Length,
    );
    pub fn vDSP_sve(input: *const f32, stride: vDSP_Stride, result: *mut f32, length: vDSP_Length);
    pub fn vDSP_sveD(
        input: *const f64,
        stride: vDSP_Stride,
        result: *mut f64,
        length: vDSP_Length,
    );
    pub fn vDSP_hamm_window(output: *mut f32, length: vDSP_Length, flags: i32);
    pub fn vDSP_hamm_windowD(output: *mut f64, length: vDSP_Length, flags: i32);
    pub fn vDSP_blkman_window(output: *mut f32, length: vDSP_Length, flags: i32);
    pub fn vDSP_blkman_windowD(output: *mut f64, length: vDSP_Length, flags: i32);
}
