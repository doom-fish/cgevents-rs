// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "CGEventsBridge",
    platforms: [.macOS(.v10_15)],
    products: [
        .library(name: "CGEventsBridge", type: .static, targets: ["CGEventsBridge"])
    ],
    targets: [
        .target(name: "CGEventsBridge", path: "Sources/CGEventsBridge")
    ]
)
