import Accelerate

@_cdecl("acc_lapack_sgetrf")
public func accLapackSgetrf(
    _ matrix: UnsafeMutablePointer<Float>?,
    _ dimension: Int32,
    _ pivots: UnsafeMutablePointer<__CLPK_integer>?
) -> Int32 {
    guard let matrix, let pivots else {
        return -1
    }

    var m = dimension
    var n = dimension
    var lda = dimension
    var info: Int32 = 0
    sgetrf_(&m, &n, matrix, &lda, pivots, &info)
    return info
}

@_cdecl("acc_lapack_sgesv")
public func accLapackSgesv(
    _ matrix: UnsafeMutablePointer<Float>?,
    _ dimension: Int32,
    _ rhs: UnsafeMutablePointer<Float>?,
    _ rhsCount: Int32,
    _ pivots: UnsafeMutablePointer<__CLPK_integer>?
) -> Int32 {
    guard let matrix, let rhs, let pivots else {
        return -1
    }

    var n = dimension
    var nrhs = rhsCount
    var lda = dimension
    var ldb = dimension
    var info: Int32 = 0
    sgesv_(&n, &nrhs, matrix, &lda, pivots, rhs, &ldb, &info)
    return info
}
