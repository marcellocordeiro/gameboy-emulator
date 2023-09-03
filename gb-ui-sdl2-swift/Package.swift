// swift-tools-version: 5.7
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

#if DEBUG
let lib_path = "-L\(#file)/../../target/debug/"
#else
let lib_path = "-L\(#file)/../../target/release/"
#endif

let package = Package(
    name: "gb-ui-sdl2-swift",
    platforms: [
        .macOS(.v13)
    ],
    dependencies: [
        .package(url: "https://github.com/ctreffs/SwiftSDL2.git", from: "1.4.1")
    ],
    targets: [
        .target(
            name: "GameBoy",
            dependencies: [],
            linkerSettings: [
                .unsafeFlags([lib_path]),
                .linkedLibrary("gb_core_c")
            ]
        ),
        .executableTarget(
            name: "GameBoyEmulator",
            dependencies: [
                "GameBoy",
                .product(name: "SDL", package: "SwiftSDL2")
            ]
        )
    ]
)
