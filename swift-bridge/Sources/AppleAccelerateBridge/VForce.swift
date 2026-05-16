import Accelerate

private func applyUnaryFloatOperation(
    _ input: UnsafePointer<Float>?,
    _ output: UnsafeMutablePointer<Float>?,
    _ length: Int,
    _ operation: (UnsafeMutablePointer<Float>, UnsafePointer<Float>, UnsafePointer<Int32>) -> Void
) -> Bool {
    guard let input, let output else {
        return false
    }
    var count = Int32(length)
    operation(output, input, &count)
    return true
}

@_cdecl("acc_vforce_sin_f32")
public func accVforceSinF32(_ input: UnsafePointer<Float>?, _ output: UnsafeMutablePointer<Float>?, _ length: Int) -> Bool {
    applyUnaryFloatOperation(input, output, length, vvsinf)
}

@_cdecl("acc_vforce_cos_f32")
public func accVforceCosF32(_ input: UnsafePointer<Float>?, _ output: UnsafeMutablePointer<Float>?, _ length: Int) -> Bool {
    applyUnaryFloatOperation(input, output, length, vvcosf)
}

@_cdecl("acc_vforce_exp_f32")
public func accVforceExpF32(_ input: UnsafePointer<Float>?, _ output: UnsafeMutablePointer<Float>?, _ length: Int) -> Bool {
    applyUnaryFloatOperation(input, output, length, vvexpf)
}

@_cdecl("acc_vforce_log_f32")
public func accVforceLogF32(_ input: UnsafePointer<Float>?, _ output: UnsafeMutablePointer<Float>?, _ length: Int) -> Bool {
    applyUnaryFloatOperation(input, output, length, vvlogf)
}

@_cdecl("acc_vforce_sqrt_f32")
public func accVforceSqrtF32(_ input: UnsafePointer<Float>?, _ output: UnsafeMutablePointer<Float>?, _ length: Int) -> Bool {
    applyUnaryFloatOperation(input, output, length, vvsqrtf)
}
