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

@_cdecl("acc_vimage_alpha_blend_argb8888")
public func accVimageAlphaBlendARGB8888(
    _ srcTopData: UnsafeMutableRawPointer?,
    _ srcTopWidth: Int,
    _ srcTopHeight: Int,
    _ srcTopRowBytes: Int,
    _ srcBottomData: UnsafeMutableRawPointer?,
    _ srcBottomWidth: Int,
    _ srcBottomHeight: Int,
    _ srcBottomRowBytes: Int,
    _ dstData: UnsafeMutableRawPointer?,
    _ dstWidth: Int,
    _ dstHeight: Int,
    _ dstRowBytes: Int,
    _ flags: UInt32
) -> Int {
    var srcTop = makeBuffer(data: srcTopData, width: srcTopWidth, height: srcTopHeight, rowBytes: srcTopRowBytes)
    var srcBottom = makeBuffer(data: srcBottomData, width: srcBottomWidth, height: srcBottomHeight, rowBytes: srcBottomRowBytes)
    var dst = makeBuffer(data: dstData, width: dstWidth, height: dstHeight, rowBytes: dstRowBytes)
    return vImageAlphaBlend_ARGB8888(&srcTop, &srcBottom, &dst, flags)
}

@_cdecl("acc_vimage_clip_to_alpha_argb8888")
public func accVimageClipToAlphaARGB8888(
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
    return vImageClipToAlpha_ARGB8888(&src, &dst, flags)
}

@_cdecl("acc_vimage_premultiply_argb8888")
public func accVimagePremultiplyARGB8888(
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
    return vImagePremultiplyData_ARGB8888(&src, &dst, flags)
}

@_cdecl("acc_vimage_unpremultiply_argb8888")
public func accVimageUnpremultiplyARGB8888(
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
    return vImageUnpremultiplyData_ARGB8888(&src, &dst, flags)
}

@_cdecl("acc_vimage_convert_planar8_to_argb8888")
public func accVimageConvertPlanar8ToARGB8888(
    _ srcAlphaData: UnsafeMutableRawPointer?,
    _ srcAlphaWidth: Int,
    _ srcAlphaHeight: Int,
    _ srcAlphaRowBytes: Int,
    _ srcRedData: UnsafeMutableRawPointer?,
    _ srcRedWidth: Int,
    _ srcRedHeight: Int,
    _ srcRedRowBytes: Int,
    _ srcGreenData: UnsafeMutableRawPointer?,
    _ srcGreenWidth: Int,
    _ srcGreenHeight: Int,
    _ srcGreenRowBytes: Int,
    _ srcBlueData: UnsafeMutableRawPointer?,
    _ srcBlueWidth: Int,
    _ srcBlueHeight: Int,
    _ srcBlueRowBytes: Int,
    _ dstData: UnsafeMutableRawPointer?,
    _ dstWidth: Int,
    _ dstHeight: Int,
    _ dstRowBytes: Int,
    _ flags: UInt32
) -> Int {
    var srcAlpha = makeBuffer(data: srcAlphaData, width: srcAlphaWidth, height: srcAlphaHeight, rowBytes: srcAlphaRowBytes)
    var srcRed = makeBuffer(data: srcRedData, width: srcRedWidth, height: srcRedHeight, rowBytes: srcRedRowBytes)
    var srcGreen = makeBuffer(data: srcGreenData, width: srcGreenWidth, height: srcGreenHeight, rowBytes: srcGreenRowBytes)
    var srcBlue = makeBuffer(data: srcBlueData, width: srcBlueWidth, height: srcBlueHeight, rowBytes: srcBlueRowBytes)
    var dst = makeBuffer(data: dstData, width: dstWidth, height: dstHeight, rowBytes: dstRowBytes)
    return vImageConvert_Planar8toARGB8888(&srcAlpha, &srcRed, &srcGreen, &srcBlue, &dst, flags)
}

@_cdecl("acc_vimage_convert_argb8888_to_planar8")
public func accVimageConvertARGB8888ToPlanar8(
    _ srcData: UnsafeMutableRawPointer?,
    _ srcWidth: Int,
    _ srcHeight: Int,
    _ srcRowBytes: Int,
    _ dstAlphaData: UnsafeMutableRawPointer?,
    _ dstAlphaWidth: Int,
    _ dstAlphaHeight: Int,
    _ dstAlphaRowBytes: Int,
    _ dstRedData: UnsafeMutableRawPointer?,
    _ dstRedWidth: Int,
    _ dstRedHeight: Int,
    _ dstRedRowBytes: Int,
    _ dstGreenData: UnsafeMutableRawPointer?,
    _ dstGreenWidth: Int,
    _ dstGreenHeight: Int,
    _ dstGreenRowBytes: Int,
    _ dstBlueData: UnsafeMutableRawPointer?,
    _ dstBlueWidth: Int,
    _ dstBlueHeight: Int,
    _ dstBlueRowBytes: Int,
    _ flags: UInt32
) -> Int {
    var src = makeBuffer(data: srcData, width: srcWidth, height: srcHeight, rowBytes: srcRowBytes)
    var dstAlpha = makeBuffer(data: dstAlphaData, width: dstAlphaWidth, height: dstAlphaHeight, rowBytes: dstAlphaRowBytes)
    var dstRed = makeBuffer(data: dstRedData, width: dstRedWidth, height: dstRedHeight, rowBytes: dstRedRowBytes)
    var dstGreen = makeBuffer(data: dstGreenData, width: dstGreenWidth, height: dstGreenHeight, rowBytes: dstGreenRowBytes)
    var dstBlue = makeBuffer(data: dstBlueData, width: dstBlueWidth, height: dstBlueHeight, rowBytes: dstBlueRowBytes)
    return vImageConvert_ARGB8888toPlanar8(&src, &dstAlpha, &dstRed, &dstGreen, &dstBlue, flags)
}
