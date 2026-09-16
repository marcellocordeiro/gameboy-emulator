// swift-tools-version: 6.4
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "GameBoyCore",
    platforms: [
        .macOS(.v27),
    ],
    products: [
        .library(name: "GameBoyCore", targets: ["GameBoyCore"]),
    ],
    targets: [
        .target(
            name: "GameBoyCore",
            dependencies: ["GameBoyCoreFFI"],
            swiftSettings: [
                .enableUpcomingFeature("InternalImportsByDefault"),
                .enableUpcomingFeature("MemberImportVisibility"),
                .enableUpcomingFeature("ExistentialAny"),
            ],
        ),
        .testTarget(
            name: "GameBoyCoreTests",
            dependencies: ["GameBoyCore"],
        ),
        .binaryTarget(
            name: "GameBoyCoreFFI",
            path: "Frameworks/GameBoyCoreFFI.xcframework",
        ),
    ],
)
