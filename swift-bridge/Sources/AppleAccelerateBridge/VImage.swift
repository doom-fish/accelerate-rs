import Accelerate

private func makeBuffer(
    data: UnsafeMutableRawPointer?,
    width: Int,
    height: Int,
    rowBytes: Int
) -> vImage_Buffer {
    vImage_Buffer(
        data: data,
        height: vImagePixelCount(height),
        width: vImagePixelCount(width),
        rowBytes: rowBytes
    )
}

@_cdecl("acc_vimage_rotate_argb8888")
public func accVimageRotateARGB8888(
    _ srcData: UnsafeMutableRawPointer?,
    _ srcWidth: Int,
    _ srcHeight: Int,
    _ srcRowBytes: Int,
    _ dstData: UnsafeMutableRawPointer?,
    _ dstWidth: Int,
    _ dstHeight: Int,
    _ dstRowBytes: Int,
    _ angleRadians: Float,
    _ backgroundColor: UnsafePointer<UInt8>?,
    _ flags: UInt32
) -> Int {
    var src = makeBuffer(data: srcData, width: srcWidth, height: srcHeight, rowBytes: srcRowBytes)
    var dst = makeBuffer(data: dstData, width: dstWidth, height: dstHeight, rowBytes: dstRowBytes)
    return vImageRotate_ARGB8888(&src, &dst, nil, angleRadians, backgroundColor, flags)
}

@_cdecl("acc_vimage_box_convolve_argb8888")
public func accVimageBoxConvolveARGB8888(
    _ srcData: UnsafeMutableRawPointer?,
    _ srcWidth: Int,
    _ srcHeight: Int,
    _ srcRowBytes: Int,
    _ dstData: UnsafeMutableRawPointer?,
    _ dstWidth: Int,
    _ dstHeight: Int,
    _ dstRowBytes: Int,
    _ kernelHeight: UInt32,
    _ kernelWidth: UInt32,
    _ backgroundColor: UnsafePointer<UInt8>?,
    _ flags: UInt32
) -> Int {
    var src = makeBuffer(data: srcData, width: srcWidth, height: srcHeight, rowBytes: srcRowBytes)
    var dst = makeBuffer(data: dstData, width: dstWidth, height: dstHeight, rowBytes: dstRowBytes)
    return vImageBoxConvolve_ARGB8888(&src, &dst, nil, 0, 0, kernelHeight, kernelWidth, backgroundColor, flags)
}

@_cdecl("acc_vimage_scale_argb8888")
public func accVimageScaleARGB8888(
    _ srcData: UnsafeMutableRawPointer?,
    _ srcWidth: Int,
    _ srcHeight: Int,
    _ srcRowBytes: Int,
    _ dstData: UnsafeMutableRawPointer?,
    _ dstWidth: Int,
    _ dstHeight: Int,
    _ dstRowBytes: Int,
    _ flags: UInt32
) -> Int {
    var src = makeBuffer(data: srcData, width: srcWidth, height: srcHeight, rowBytes: srcRowBytes)
    var dst = makeBuffer(data: dstData, width: dstWidth, height: dstHeight, rowBytes: dstRowBytes)
    return vImageScale_ARGB8888(&src, &dst, nil, flags)
}

@_cdecl("acc_vimage_contrast_stretch_planar8")
public func accVimageContrastStretchPlanar8(
    _ srcData: UnsafeMutableRawPointer?,
    _ srcWidth: Int,
    _ srcHeight: Int,
    _ srcRowBytes: Int,
    _ dstData: UnsafeMutableRawPointer?,
    _ dstWidth: Int,
    _ dstHeight: Int,
    _ dstRowBytes: Int,
    _ flags: UInt32
) -> Int {
    var src = makeBuffer(data: srcData, width: srcWidth, height: srcHeight, rowBytes: srcRowBytes)
    var dst = makeBuffer(data: dstData, width: dstWidth, height: dstHeight, rowBytes: dstRowBytes)
    return vImageContrastStretch_Planar8(&src, &dst, flags)
}
