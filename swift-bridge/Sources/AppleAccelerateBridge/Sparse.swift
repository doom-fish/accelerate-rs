import Accelerate

@_cdecl("acc_sparse_dot_dense_f32")
public func accSparseDotDenseF32(
    _ nz: UInt64,
    _ values: UnsafePointer<Float>?,
    _ indices: UnsafePointer<sparse_index>?,
    _ dense: UnsafePointer<Float>?
) -> Float {
    guard let values, let indices, let dense else {
        return 0
    }
    return sparse_inner_product_dense_float(sparse_dimension(nz), values, indices, dense, 1)
}

@_cdecl("acc_sparse_dot_sparse_f32")
public func accSparseDotSparseF32(
    _ lhsCount: UInt64,
    _ lhsValues: UnsafePointer<Float>?,
    _ lhsIndices: UnsafePointer<sparse_index>?,
    _ rhsCount: UInt64,
    _ rhsValues: UnsafePointer<Float>?,
    _ rhsIndices: UnsafePointer<sparse_index>?
) -> Float {
    guard let lhsValues, let lhsIndices, let rhsValues, let rhsIndices else {
        return 0
    }
    return sparse_inner_product_sparse_float(
        sparse_dimension(lhsCount),
        sparse_dimension(rhsCount),
        lhsValues,
        lhsIndices,
        rhsValues,
        rhsIndices
    )
}

@_cdecl("acc_sparse_add_to_dense_f32")
public func accSparseAddToDenseF32(
    _ nz: UInt64,
    _ alpha: Float,
    _ values: UnsafePointer<Float>?,
    _ indices: UnsafePointer<sparse_index>?,
    _ dense: UnsafeMutablePointer<Float>?
) -> Bool {
    guard let values, let indices, let dense else {
        return false
    }
    sparse_vector_add_with_scale_dense_float(sparse_dimension(nz), alpha, values, indices, dense, 1)
    return true
}
