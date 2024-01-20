// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let packageDirectory = Context.packageDirectory
let rootDirectory = "\(packageDirectory)/../.."
let coreDirectory = "\(rootDirectory)/core/gb-core-swift"

let package = Package(
    name: "GameBoyEmulator",
    platforms: [
        .macOS(.v14)
    ],
    dependencies: [
        .package(path: coreDirectory),
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
