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
