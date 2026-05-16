// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "AppleAccelerateBridge",
    platforms: [
        .macOS(.v11)
    ],
    products: [
        .library(
            name: "AppleAccelerateBridge",
            type: .static,
            targets: ["AppleAccelerateBridge"]
        )
    ],
    targets: [
        .target(
            name: "AppleAccelerateBridge",
            path: "Sources/AppleAccelerateBridge",
            publicHeadersPath: "include"
        )
    ]
)
