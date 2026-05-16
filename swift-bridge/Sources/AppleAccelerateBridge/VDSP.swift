import Accelerate
import Foundation

private final class FFTSetupHandle {
    let setup: FFTSetup

    init?(log2n: Int, radix: Int32) {
        guard let setup = vDSP_create_fftsetup(vDSP_Length(log2n), FFTRadix(radix)) else {
            return nil
        }
        self.setup = setup
    }

    deinit {
        vDSP_destroy_fftsetup(setup)
    }
}

private final class BiquadSetupHandle {
    let setup: vDSP_biquad_Setup

    init?(coefficients: UnsafePointer<Double>, count: Int) {
        guard count > 0, count % 5 == 0,
              let setup = vDSP_biquad_CreateSetup(coefficients, vDSP_Length(count / 5))
        else {
            return nil
        }
        self.setup = setup
    }

    deinit {
        vDSP_biquad_DestroySetup(setup)
    }
}

@_cdecl("acc_vdsp_fft_setup_create")
public func accVdspFftSetupCreate(_ log2n: Int, _ radix: Int32) -> UnsafeMutableRawPointer? {
    guard let handle = FFTSetupHandle(log2n: log2n, radix: radix) else {
        return nil
    }
    return retain(handle)
}

@_cdecl("acc_vdsp_fft_setup_apply")
public func accVdspFftSetupApply(
    _ handle: UnsafeMutableRawPointer?,
    _ real: UnsafeMutablePointer<Float>?,
    _ imag: UnsafeMutablePointer<Float>?,
    _ log2n: Int,
    _ direction: Int32
) -> Bool {
    guard let handle, let real, let imag else {
        return false
    }

    let setup: FFTSetupHandle = unretained(handle)
    var split = DSPSplitComplex(realp: real, imagp: imag)
    vDSP_fft_zip(setup.setup, &split, 1, vDSP_Length(log2n), FFTDirection(direction))
    return true
}

@_cdecl("acc_vdsp_biquad_setup_create")
public func accVdspBiquadSetupCreate(
    _ coefficients: UnsafePointer<Double>?,
    _ count: Int
) -> UnsafeMutableRawPointer? {
    guard let coefficients,
          let handle = BiquadSetupHandle(coefficients: coefficients, count: count)
    else {
        return nil
    }
    return retain(handle)
}

@_cdecl("acc_vdsp_biquad_setup_apply")
public func accVdspBiquadSetupApply(
    _ handle: UnsafeMutableRawPointer?,
    _ delay: UnsafeMutablePointer<Float>?,
    _ input: UnsafePointer<Float>?,
    _ output: UnsafeMutablePointer<Float>?,
    _ length: Int
) -> Bool {
    guard let handle, let delay, let input, let output else {
        return false
    }

    let setup: BiquadSetupHandle = unretained(handle)
    vDSP_biquad(setup.setup, delay, input, 1, output, 1, vDSP_Length(length))
    return true
}

@_cdecl("acc_vdsp_add_f32")
public func accVdspAddF32(
    _ a: UnsafePointer<Float>?,
    _ b: UnsafePointer<Float>?,
    _ output: UnsafeMutablePointer<Float>?,
    _ length: Int
) -> Bool {
    guard let a, let b, let output else {
        return false
    }
    vDSP_vadd(a, 1, b, 1, output, 1, vDSP_Length(length))
    return true
}

@_cdecl("acc_vdsp_add_f64")
public func accVdspAddF64(
    _ a: UnsafePointer<Double>?,
    _ b: UnsafePointer<Double>?,
    _ output: UnsafeMutablePointer<Double>?,
    _ length: Int
) -> Bool {
    guard let a, let b, let output else {
        return false
    }
    vDSP_vaddD(a, 1, b, 1, output, 1, vDSP_Length(length))
    return true
}

@_cdecl("acc_vdsp_sub_f32")
public func accVdspSubF32(
    _ a: UnsafePointer<Float>?,
    _ b: UnsafePointer<Float>?,
    _ output: UnsafeMutablePointer<Float>?,
    _ length: Int
) -> Bool {
    guard let a, let b, let output else {
        return false
    }
    vDSP_vsub(b, 1, a, 1, output, 1, vDSP_Length(length))
    return true
}

@_cdecl("acc_vdsp_sub_f64")
public func accVdspSubF64(
    _ a: UnsafePointer<Double>?,
    _ b: UnsafePointer<Double>?,
    _ output: UnsafeMutablePointer<Double>?,
    _ length: Int
) -> Bool {
    guard let a, let b, let output else {
        return false
    }
    vDSP_vsubD(b, 1, a, 1, output, 1, vDSP_Length(length))
    return true
}

@_cdecl("acc_vdsp_dot_f32")
public func accVdspDotF32(
    _ a: UnsafePointer<Float>?,
    _ b: UnsafePointer<Float>?,
    _ output: UnsafeMutablePointer<Float>?,
    _ length: Int
) -> Bool {
    guard let a, let b, let output else {
        return false
    }
    vDSP_dotpr(a, 1, b, 1, output, vDSP_Length(length))
    return true
}

@_cdecl("acc_vdsp_dot_f64")
public func accVdspDotF64(
    _ a: UnsafePointer<Double>?,
    _ b: UnsafePointer<Double>?,
    _ output: UnsafeMutablePointer<Double>?,
    _ length: Int
) -> Bool {
    guard let a, let b, let output else {
        return false
    }
    vDSP_dotprD(a, 1, b, 1, output, vDSP_Length(length))
    return true
}

@_cdecl("acc_vdsp_max_f32")
public func accVdspMaxF32(
    _ input: UnsafePointer<Float>?,
    _ output: UnsafeMutablePointer<Float>?,
    _ length: Int
) -> Bool {
    guard let input, let output else {
        return false
    }
    vDSP_maxv(input, 1, output, vDSP_Length(length))
    return true
}

@_cdecl("acc_vdsp_max_f64")
public func accVdspMaxF64(
    _ input: UnsafePointer<Double>?,
    _ output: UnsafeMutablePointer<Double>?,
    _ length: Int
) -> Bool {
    guard let input, let output else {
        return false
    }
    vDSP_maxvD(input, 1, output, vDSP_Length(length))
    return true
}

@_cdecl("acc_vdsp_min_f32")
public func accVdspMinF32(
    _ input: UnsafePointer<Float>?,
    _ output: UnsafeMutablePointer<Float>?,
    _ length: Int
) -> Bool {
    guard let input, let output else {
        return false
    }
    vDSP_minv(input, 1, output, vDSP_Length(length))
    return true
}

@_cdecl("acc_vdsp_min_f64")
public func accVdspMinF64(
    _ input: UnsafePointer<Double>?,
    _ output: UnsafeMutablePointer<Double>?,
    _ length: Int
) -> Bool {
    guard let input, let output else {
        return false
    }
    vDSP_minvD(input, 1, output, vDSP_Length(length))
    return true
}

@_cdecl("acc_vdsp_mean_f32")
public func accVdspMeanF32(
    _ input: UnsafePointer<Float>?,
    _ output: UnsafeMutablePointer<Float>?,
    _ length: Int
) -> Bool {
    guard let input, let output else {
        return false
    }
    vDSP_meanv(input, 1, output, vDSP_Length(length))
    return true
}

@_cdecl("acc_vdsp_mean_f64")
public func accVdspMeanF64(
    _ input: UnsafePointer<Double>?,
    _ output: UnsafeMutablePointer<Double>?,
    _ length: Int
) -> Bool {
    guard let input, let output else {
        return false
    }
    vDSP_meanvD(input, 1, output, vDSP_Length(length))
    return true
}

@_cdecl("acc_vdsp_sum_f32")
public func accVdspSumF32(
    _ input: UnsafePointer<Float>?,
    _ output: UnsafeMutablePointer<Float>?,
    _ length: Int
) -> Bool {
    guard let input, let output else {
        return false
    }
    vDSP_sve(input, 1, output, vDSP_Length(length))
    return true
}

@_cdecl("acc_vdsp_sum_f64")
public func accVdspSumF64(
    _ input: UnsafePointer<Double>?,
    _ output: UnsafeMutablePointer<Double>?,
    _ length: Int
) -> Bool {
    guard let input, let output else {
        return false
    }
    vDSP_sveD(input, 1, output, vDSP_Length(length))
    return true
}

@_cdecl("acc_vdsp_hamming_window")
public func accVdspHammingWindow(_ output: UnsafeMutablePointer<Float>?, _ length: Int, _ flags: Int32) -> Bool {
    guard let output else {
        return false
    }
    vDSP_hamm_window(output, vDSP_Length(length), flags)
    return true
}

@_cdecl("acc_vdsp_hamming_window_f64")
public func accVdspHammingWindowF64(
    _ output: UnsafeMutablePointer<Double>?,
    _ length: Int,
    _ flags: Int32
) -> Bool {
    guard let output else {
        return false
    }
    vDSP_hamm_windowD(output, vDSP_Length(length), flags)
    return true
}

@_cdecl("acc_vdsp_blackman_window")
public func accVdspBlackmanWindow(_ output: UnsafeMutablePointer<Float>?, _ length: Int, _ flags: Int32) -> Bool {
    guard let output else {
        return false
    }
    vDSP_blkman_window(output, vDSP_Length(length), flags)
    return true
}

@_cdecl("acc_vdsp_blackman_window_f64")
public func accVdspBlackmanWindowF64(
    _ output: UnsafeMutablePointer<Double>?,
    _ length: Int,
    _ flags: Int32
) -> Bool {
    guard let output else {
        return false
    }
    vDSP_blkman_windowD(output, vDSP_Length(length), flags)
    return true
}
