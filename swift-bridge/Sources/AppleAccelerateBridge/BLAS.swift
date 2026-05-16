import Accelerate

@_cdecl("acc_blas_sdot")
public func accBlasSdot(
    _ length: Int32,
    _ x: UnsafePointer<Float>?,
    _ y: UnsafePointer<Float>?
) -> Float {
    guard let x, let y else {
        return 0
    }
    return cblas_sdot(length, x, 1, y, 1)
}

@_cdecl("acc_blas_sgemv_row_major")
public func accBlasSgemvRowMajor(
    _ rows: Int32,
    _ columns: Int32,
    _ alpha: Float,
    _ matrix: UnsafePointer<Float>?,
    _ x: UnsafePointer<Float>?,
    _ beta: Float,
    _ y: UnsafeMutablePointer<Float>?
) -> Bool {
    guard let matrix, let x, let y else {
        return false
    }

    cblas_sgemv(CblasRowMajor, CblasNoTrans, rows, columns, alpha, matrix, columns, x, 1, beta, y, 1)
    return true
}

@_cdecl("acc_blas_sgemm_row_major")
public func accBlasSgemmRowMajor(
    _ rows: Int32,
    _ columns: Int32,
    _ innerDimension: Int32,
    _ alpha: Float,
    _ lhs: UnsafePointer<Float>?,
    _ rhs: UnsafePointer<Float>?,
    _ beta: Float,
    _ output: UnsafeMutablePointer<Float>?
) -> Bool {
    guard let lhs, let rhs, let output else {
        return false
    }

    cblas_sgemm(
        CblasRowMajor,
        CblasNoTrans,
        CblasNoTrans,
        rows,
        columns,
        innerDimension,
        alpha,
        lhs,
        innerDimension,
        rhs,
        columns,
        beta,
        output,
        columns
    )
    return true
}
