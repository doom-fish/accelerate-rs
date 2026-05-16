import Foundation

func retain(_ object: AnyObject) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

func unretained<T: AnyObject>(_ ptr: UnsafeMutableRawPointer) -> T {
    Unmanaged<T>.fromOpaque(ptr).takeUnretainedValue()
}

@_cdecl("acc_release_handle")
public func accReleaseHandle(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    Unmanaged<AnyObject>.fromOpaque(handle).release()
}
