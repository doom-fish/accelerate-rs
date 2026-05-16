#![allow(missing_docs, non_camel_case_types, non_upper_case_globals)]

use core::ffi::c_void;

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

pub type CBLAS_ORDER = i32;
pub type CBLAS_TRANSPOSE = i32;

pub type vImagePixelCount = u64;
pub type vImage_Error = isize;
pub type vImage_Flags = u32;
pub type Pixel_8888 = [u8; 4];

#[repr(C)]
pub struct vImage_Buffer {
    pub data: *mut c_void,
    pub height: vImagePixelCount,
    pub width: vImagePixelCount,
    pub row_bytes: usize,
}

pub type BNNSFilter = *mut c_void;

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
    pub fn vDSP_vsub(
        b: *const f32,
        b_stride: vDSP_Stride,
        a: *const f32,
        a_stride: vDSP_Stride,
        c: *mut f32,
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
    pub fn vDSP_maxv(input: *const f32, stride: vDSP_Stride, result: *mut f32, length: vDSP_Length);
    pub fn vDSP_minv(input: *const f32, stride: vDSP_Stride, result: *mut f32, length: vDSP_Length);
    pub fn vDSP_meanv(
        input: *const f32,
        stride: vDSP_Stride,
        result: *mut f32,
        length: vDSP_Length,
    );
    pub fn vDSP_sve(input: *const f32, stride: vDSP_Stride, result: *mut f32, length: vDSP_Length);
    pub fn vDSP_hamm_window(output: *mut f32, length: vDSP_Length, flags: i32);
    pub fn vDSP_blkman_window(output: *mut f32, length: vDSP_Length, flags: i32);

    pub fn cblas_sdot(n: i32, x: *const f32, inc_x: i32, y: *const f32, inc_y: i32) -> f32;
    pub fn cblas_sgemv(
        order: CBLAS_ORDER,
        transpose: CBLAS_TRANSPOSE,
        m: i32,
        n: i32,
        alpha: f32,
        a: *const f32,
        lda: i32,
        x: *const f32,
        inc_x: i32,
        beta: f32,
        y: *mut f32,
        inc_y: i32,
    );
    pub fn cblas_sgemm(
        order: CBLAS_ORDER,
        transpose_a: CBLAS_TRANSPOSE,
        transpose_b: CBLAS_TRANSPOSE,
        m: i32,
        n: i32,
        k: i32,
        alpha: f32,
        a: *const f32,
        lda: i32,
        b: *const f32,
        ldb: i32,
        beta: f32,
        c: *mut f32,
        ldc: i32,
    );

    pub fn vImageRotate_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        temp_buffer: *mut c_void,
        angle_in_radians: f32,
        background_color: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
    pub fn vImageBoxConvolve_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        temp_buffer: *mut c_void,
        src_offset_to_roi_x: vImagePixelCount,
        src_offset_to_roi_y: vImagePixelCount,
        kernel_height: u32,
        kernel_width: u32,
        background_color: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
    pub fn vImageScale_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        temp_buffer: *mut c_void,
        flags: vImage_Flags,
    ) -> vImage_Error;
    pub fn vImageContrastStretch_Planar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;

    pub fn BNNSFilterCreateLayerConvolution(
        layer_params: *const c_void,
        filter_params: *const c_void,
    ) -> BNNSFilter;
    pub fn BNNSFilterCreateLayerFullyConnected(
        layer_params: *const c_void,
        filter_params: *const c_void,
    ) -> BNNSFilter;
    pub fn BNNSFilterApply(filter: BNNSFilter, input: *const c_void, output: *mut c_void) -> i32;
    pub fn BNNSFilterDestroy(filter: BNNSFilter);
}
