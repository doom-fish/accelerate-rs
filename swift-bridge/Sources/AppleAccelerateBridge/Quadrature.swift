import Accelerate

public typealias RustQuadratureCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    Int,
    UnsafePointer<Double>,
    UnsafeMutablePointer<Double>
) -> Void

@_cdecl("acc_quadrature_integrate")
public func accQuadratureIntegrate(
    _ callback: RustQuadratureCallback?,
    _ context: UnsafeMutableRawPointer?,
    _ a: Double,
    _ b: Double,
    _ integrator: Int32,
    _ absTolerance: Double,
    _ relTolerance: Double,
    _ qagPointsPerInterval: Int,
    _ maxIntervals: Int,
    _ statusOut: UnsafeMutablePointer<Int32>?,
    _ absErrorOut: UnsafeMutablePointer<Double>?
) -> Double {
    guard let callback else {
        statusOut?.pointee = Int32(QUADRATURE_INVALID_ARG_ERROR.rawValue)
        absErrorOut?.pointee = .nan
        return .nan
    }

    let integratorValue: quadrature_integrator
    switch integrator {
    case 0:
        integratorValue = QUADRATURE_INTEGRATE_QNG
    case 1:
        integratorValue = QUADRATURE_INTEGRATE_QAG
    case 2:
        integratorValue = QUADRATURE_INTEGRATE_QAGS
    default:
        statusOut?.pointee = Int32(QUADRATURE_INVALID_ARG_ERROR.rawValue)
        absErrorOut?.pointee = .nan
        return .nan
    }

    var function = quadrature_integrate_function(fun: callback, fun_arg: context)
    let options = quadrature_integrate_options(
        integrator: integratorValue,
        abs_tolerance: absTolerance,
        rel_tolerance: relTolerance,
        qag_points_per_interval: qagPointsPerInterval,
        max_intervals: maxIntervals
    )
    var status = QUADRATURE_SUCCESS
    var absError = 0.0
    let result = quadrature_integrate(&function, a, b, [options], &status, &absError, 0, nil)
    statusOut?.pointee = Int32(status.rawValue)
    absErrorOut?.pointee = absError
    return result
}
