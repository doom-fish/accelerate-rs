import Foundation
import simd

private func loadVector(_ pointer: UnsafePointer<Float>) -> SIMD4<Float> {
    SIMD4(pointer[0], pointer[1], pointer[2], pointer[3])
}

private func storeVector(_ vector: SIMD4<Float>, into output: UnsafeMutablePointer<Float>) {
    output[0] = vector.x
    output[1] = vector.y
    output[2] = vector.z
    output[3] = vector.w
}

@_cdecl("acc_simd_add_f32x4")
public func accSimdAddF32x4(
    _ lhs: UnsafePointer<Float>?,
    _ rhs: UnsafePointer<Float>?,
    _ output: UnsafeMutablePointer<Float>?
) -> Bool {
    guard let lhs, let rhs, let output else {
        return false
    }
    storeVector(loadVector(lhs) + loadVector(rhs), into: output)
    return true
}

@_cdecl("acc_simd_dot_f32x4")
public func accSimdDotF32x4(
    _ lhs: UnsafePointer<Float>?,
    _ rhs: UnsafePointer<Float>?,
    _ output: UnsafeMutablePointer<Float>?
) -> Bool {
    guard let lhs, let rhs, let output else {
        return false
    }
    output.pointee = simd_dot(loadVector(lhs), loadVector(rhs))
    return true
}

@_cdecl("acc_simd_length_f32x4")
public func accSimdLengthF32x4(
    _ input: UnsafePointer<Float>?,
    _ output: UnsafeMutablePointer<Float>?
) -> Bool {
    guard let input, let output else {
        return false
    }
    output.pointee = simd_length(loadVector(input))
    return true
}

@_cdecl("acc_simd_normalize_f32x4")
public func accSimdNormalizeF32x4(
    _ input: UnsafePointer<Float>?,
    _ output: UnsafeMutablePointer<Float>?
) -> Bool {
    guard let input, let output else {
        return false
    }
    storeVector(simd_normalize(loadVector(input)), into: output)
    return true
}
