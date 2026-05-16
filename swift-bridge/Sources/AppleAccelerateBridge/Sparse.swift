import Accelerate

private let sparseIllegalParameter = Int32(-1000)

private func sparseStatusCode(_ status: sparse_status) -> Int32 {
    unsafeBitCast(status, to: Int32.self)
}

private func sparseProperty(_ value: Int32) -> sparse_matrix_property? {
    switch value {
    case 1, 2, 4, 8:
        return unsafeBitCast(UInt32(bitPattern: value), to: sparse_matrix_property.self)
    default:
        return nil
    }
}

private func cblasTranspose(_ value: Int32) -> CBLAS_TRANSPOSE? {
    switch value {
    case 111:
        return CblasNoTrans
    case 112:
        return CblasTrans
    case 113:
        return CblasConjTrans
    default:
        return nil
    }
}

private func cblasOrder(_ value: Int32) -> CBLAS_ORDER? {
    switch value {
    case 101:
        return CblasRowMajor
    case 102:
        return CblasColMajor
    default:
        return nil
    }
}

private func rawMatrixPointer(_ matrix: sparse_matrix_float) -> UnsafeMutableRawPointer {
    UnsafeMutableRawPointer(matrix)
}

private final class SparseMatrixFloatHandle {
    let matrix: sparse_matrix_float

    init?(rows: UInt64, columns: UInt64) {
        guard let matrix = sparse_matrix_create_float(sparse_dimension(rows), sparse_dimension(columns)) else {
            return nil
        }
        self.matrix = matrix
    }

    deinit {
        _ = sparse_matrix_destroy(rawMatrixPointer(matrix))
    }
}

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

@_cdecl("acc_sparse_matrix_f32_create")
public func accSparseMatrixF32Create(_ rows: UInt64, _ columns: UInt64) -> UnsafeMutableRawPointer? {
    guard let handle = SparseMatrixFloatHandle(rows: rows, columns: columns) else {
        return nil
    }
    return retain(handle)
}

@_cdecl("acc_sparse_matrix_f32_set_property")
public func accSparseMatrixF32SetProperty(_ handle: UnsafeMutableRawPointer?, _ property: Int32) -> Int32 {
    guard let handle,
          let property = sparseProperty(property)
    else {
        return sparseIllegalParameter
    }

    let matrix: SparseMatrixFloatHandle = unretained(handle)
    return sparseStatusCode(
        sparse_set_matrix_property(rawMatrixPointer(matrix.matrix), property)
    )
}

@_cdecl("acc_sparse_matrix_f32_insert_entry")
public func accSparseMatrixF32InsertEntry(
    _ handle: UnsafeMutableRawPointer?,
    _ value: Float,
    _ row: Int64,
    _ column: Int64
) -> Int32 {
    guard let handle else {
        return sparseIllegalParameter
    }

    let matrix: SparseMatrixFloatHandle = unretained(handle)
    return sparseStatusCode(sparse_insert_entry_float(matrix.matrix, value, sparse_index(row), sparse_index(column)))
}

@_cdecl("acc_sparse_matrix_f32_commit")
public func accSparseMatrixF32Commit(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let handle else {
        return sparseIllegalParameter
    }

    let matrix: SparseMatrixFloatHandle = unretained(handle)
    return sparseStatusCode(sparse_commit(rawMatrixPointer(matrix.matrix)))
}

@_cdecl("acc_sparse_matrix_f32_rows")
public func accSparseMatrixF32Rows(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let handle else {
        return 0
    }

    let matrix: SparseMatrixFloatHandle = unretained(handle)
    return UInt64(sparse_get_matrix_number_of_rows(rawMatrixPointer(matrix.matrix)))
}

@_cdecl("acc_sparse_matrix_f32_columns")
public func accSparseMatrixF32Columns(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let handle else {
        return 0
    }

    let matrix: SparseMatrixFloatHandle = unretained(handle)
    return UInt64(sparse_get_matrix_number_of_columns(rawMatrixPointer(matrix.matrix)))
}

@_cdecl("acc_sparse_matrix_f32_nonzero_count")
public func accSparseMatrixF32NonzeroCount(_ handle: UnsafeMutableRawPointer?) -> Int64 {
    guard let handle else {
        return -1
    }

    let matrix: SparseMatrixFloatHandle = unretained(handle)
    return Int64(sparse_get_matrix_nonzero_count(rawMatrixPointer(matrix.matrix)))
}

@_cdecl("acc_sparse_matrix_f32_triangular_solve_vector")
public func accSparseMatrixF32TriangularSolveVector(
    _ handle: UnsafeMutableRawPointer?,
    _ transpose: Int32,
    _ alpha: Float,
    _ values: UnsafeMutablePointer<Float>?,
    _ length: UInt64
) -> Int32 {
    guard let handle,
          let values,
          let transpose = cblasTranspose(transpose)
    else {
        return sparseIllegalParameter
    }

    _ = length
    let matrix: SparseMatrixFloatHandle = unretained(handle)
    return sparseStatusCode(
        sparse_vector_triangular_solve_dense_float(transpose, alpha, matrix.matrix, values, 1)
    )
}

@_cdecl("acc_sparse_matrix_f32_triangular_solve_matrix")
public func accSparseMatrixF32TriangularSolveMatrix(
    _ handle: UnsafeMutableRawPointer?,
    _ order: Int32,
    _ transpose: Int32,
    _ rhsCount: UInt64,
    _ alpha: Float,
    _ values: UnsafeMutablePointer<Float>?,
    _ ldb: UInt64
) -> Int32 {
    guard let handle,
          let values,
          let order = cblasOrder(order),
          let transpose = cblasTranspose(transpose)
    else {
        return sparseIllegalParameter
    }

    let matrix: SparseMatrixFloatHandle = unretained(handle)
    return sparseStatusCode(
        sparse_matrix_triangular_solve_dense_float(
            order,
            transpose,
            sparse_dimension(rhsCount),
            alpha,
            matrix.matrix,
            values,
            sparse_dimension(ldb)
        )
    )
}
