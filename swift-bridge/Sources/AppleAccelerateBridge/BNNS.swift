import Accelerate

private let bnnsFloat32 = unsafeBitCast(UInt32(0x10020), to: BNNSDataType.self)
private let bnnsVectorLayout = unsafeBitCast(UInt32(0x10000), to: BNNSDataLayout.self)
private let bnnsReLU = unsafeBitCast(UInt32(1), to: BNNSActivationFunction.self)
private let bnnsSigmoid = unsafeBitCast(UInt32(3), to: BNNSActivationFunction.self)

private func vectorDescriptor(
    length: Int,
    data: UnsafeMutableRawPointer?
) -> BNNSNDArrayDescriptor {
    BNNSNDArrayDescriptor(
        flags: BNNSNDArrayFlags(rawValue: 0),
        layout: bnnsVectorLayout,
        size: (numericCast(length), 0, 0, 0, 0, 0, 0, 0),
        stride: (1, 0, 0, 0, 0, 0, 0, 0),
        data: data,
        data_type: bnnsFloat32,
        table_data: nil,
        table_data_type: bnnsFloat32,
        data_scale: 1,
        data_bias: 0
    )
}

private func applyActivation(
    function: BNNSActivationFunction,
    input: UnsafePointer<Float>,
    output: UnsafeMutablePointer<Float>,
    length: Int
) -> Int32 {
    let inputDescriptor = vectorDescriptor(length: length, data: UnsafeMutableRawPointer(mutating: input))
    let outputDescriptor = vectorDescriptor(length: length, data: output)
    let activation = BNNSActivation(
        function: function,
        alpha: 0,
        beta: 0,
        iscale: 0,
        ioffset: 0,
        ishift: 0,
        iscale_per_channel: nil,
        ioffset_per_channel: nil,
        ishift_per_channel: nil
    )
    var params = BNNSLayerParametersActivation(
        i_desc: inputDescriptor,
        o_desc: outputDescriptor,
        activation: activation,
        axis_flags: 0
    )

    guard let filter = BNNSFilterCreateLayerActivation(&params, nil) else {
        return -1
    }
    defer { BNNSFilterDestroy(filter) }
    return Int32(BNNSFilterApply(filter, input, output))
}

private func bnnsGraphOptimizationPreference(_ rawValue: UInt32) -> BNNSGraphOptimizationPreference? {
    switch rawValue {
    case 0, 1:
        return unsafeBitCast(rawValue, to: BNNSGraphOptimizationPreference.self)
    default:
        return nil
    }
}

private func bnnsGraphOptimizationPreferenceRaw(_ preference: BNNSGraphOptimizationPreference) -> UInt32 {
    unsafeBitCast(preference, to: UInt32.self)
}

@available(macOS 15.0, *)
private final class BNNSGraphCompileOptionsHandle {
    let options: bnns_graph_compile_options_t

    init?() {
        let options = BNNSGraphCompileOptionsMakeDefault()
        guard options.data != nil else {
            return nil
        }
        self.options = options
    }

    deinit {
        BNNSGraphCompileOptionsDestroy(options)
    }
}

@_cdecl("acc_bnns_relu_f32")
public func accBnnsReluF32(_ input: UnsafePointer<Float>?, _ output: UnsafeMutablePointer<Float>?, _ length: Int) -> Int32 {
    guard let input, let output else {
        return -1
    }
    return applyActivation(function: bnnsReLU, input: input, output: output, length: length)
}

@_cdecl("acc_bnns_sigmoid_f32")
public func accBnnsSigmoidF32(_ input: UnsafePointer<Float>?, _ output: UnsafeMutablePointer<Float>?, _ length: Int) -> Int32 {
    guard let input, let output else {
        return -1
    }
    return applyActivation(function: bnnsSigmoid, input: input, output: output, length: length)
}

@_cdecl("acc_bnns_graph_compile_options_create")
public func accBnnsGraphCompileOptionsCreate() -> UnsafeMutableRawPointer? {
    guard #available(macOS 15.0, *) else {
        return nil
    }
    guard let handle = BNNSGraphCompileOptionsHandle() else {
        return nil
    }
    return retain(handle)
}

@_cdecl("acc_bnns_graph_compile_options_set_target_single_thread")
public func accBnnsGraphCompileOptionsSetTargetSingleThread(
    _ handle: UnsafeMutableRawPointer?,
    _ value: Bool
) -> Bool {
    guard #available(macOS 15.0, *), let handle else {
        return false
    }

    let options: BNNSGraphCompileOptionsHandle = unretained(handle)
    BNNSGraphCompileOptionsSetTargetSingleThread(options.options, value)
    return true
}

@_cdecl("acc_bnns_graph_compile_options_get_target_single_thread")
public func accBnnsGraphCompileOptionsGetTargetSingleThread(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard #available(macOS 15.0, *), let handle else {
        return false
    }

    let options: BNNSGraphCompileOptionsHandle = unretained(handle)
    return BNNSGraphCompileOptionsGetTargetSingleThread(options.options)
}

@_cdecl("acc_bnns_graph_compile_options_set_generate_debug_info")
public func accBnnsGraphCompileOptionsSetGenerateDebugInfo(
    _ handle: UnsafeMutableRawPointer?,
    _ value: Bool
) -> Bool {
    guard #available(macOS 15.0, *), let handle else {
        return false
    }

    let options: BNNSGraphCompileOptionsHandle = unretained(handle)
    BNNSGraphCompileOptionsSetGenerateDebugInfo(options.options, value)
    return true
}

@_cdecl("acc_bnns_graph_compile_options_get_generate_debug_info")
public func accBnnsGraphCompileOptionsGetGenerateDebugInfo(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard #available(macOS 15.0, *), let handle else {
        return false
    }

    let options: BNNSGraphCompileOptionsHandle = unretained(handle)
    return BNNSGraphCompileOptionsGetGenerateDebugInfo(options.options)
}

@_cdecl("acc_bnns_graph_compile_options_set_optimization_preference")
public func accBnnsGraphCompileOptionsSetOptimizationPreference(
    _ handle: UnsafeMutableRawPointer?,
    _ preference: UInt32
) -> Bool {
    guard #available(macOS 15.0, *),
          let handle,
          let preference = bnnsGraphOptimizationPreference(preference)
    else {
        return false
    }

    let options: BNNSGraphCompileOptionsHandle = unretained(handle)
    BNNSGraphCompileOptionsSetOptimizationPreference(options.options, preference)
    return true
}

@_cdecl("acc_bnns_graph_compile_options_get_optimization_preference")
public func accBnnsGraphCompileOptionsGetOptimizationPreference(_ handle: UnsafeMutableRawPointer?) -> UInt32 {
    guard #available(macOS 15.0, *), let handle else {
        return 0
    }

    let options: BNNSGraphCompileOptionsHandle = unretained(handle)
    return bnnsGraphOptimizationPreferenceRaw(
        BNNSGraphCompileOptionsGetOptimizationPreference(options.options)
    )
}
