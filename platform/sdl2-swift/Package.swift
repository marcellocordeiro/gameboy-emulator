// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let packageDir = Context.packageDirectory
let rootDir = "\(packageDir)/../.."
let coreDir = "\(rootDir)/core/gb-core-swift"

let package = Package(
    name: "GameBoyEmulator",
    platforms: [
        .macOS(.v14)
    ],
    dependencies: [
        .package(path: coreDir),
        .package(url: "https://github.com/ctreffs/SwiftSDL2.git", from: "1.4.1")
    ],
    targets: [
        .executableTarget(
            name: "GameBoy",
            dependencies: [
                .product(name: "GameBoyCore", package: "gb-core-swift"),
                .product(name: "SDL", package: "SwiftSDL2")
            ]
        )
    ]
)
