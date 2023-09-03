// swift-tools-version: 5.8
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let packageDirectory = Context.packageDirectory

#if DEBUG
let libDirectory = "\(packageDirectory)/../target/debug"
#else
let libDirectory = "\(packageDirectory)/../target/release"
#endif

let staticLibFile = "libgb_core_c.a"
let linkerFlag = "-l\(libDirectory)/\(staticLibFile)"

let package = Package(
    name: "GameBoyEmulator",
    platforms: [
        .macOS(.v13)
    ],
    dependencies: [
        .package(url: "https://github.com/ctreffs/SwiftSDL2.git", from: "1.4.1")
    ],
    targets: [
        .target(
            name: "GameBoyCore",
            dependencies: [],
            linkerSettings: [
                .unsafeFlags([linkerFlag]),
            ]
        ),
        .executableTarget(
            name: "GameBoy",
            dependencies: [
                "GameBoyCore",
                .product(name: "SDL", package: "SwiftSDL2")
            ]
        )
    ]
)
