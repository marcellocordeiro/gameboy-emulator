// swift-tools-version: 5.8
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let packageDirectory = Context.packageDirectory
let rootDirectory = "\(packageDirectory)/.."

let gbHeaderPath = "\(rootDirectory)/gb-core-c/gb-bindings.h"

#if DEBUG
let libDirectory = "\(packageDirectory)/../target/debug"
#else
let libDirectory = "\(packageDirectory)/../target/release"
#endif

let staticLibPath = "\(libDirectory)/libgb_core_c.a"

let linkerSettings: [PackageDescription.LinkerSetting]
#if os(macOS)
linkerSettings = [
    .unsafeFlags(["-L\(libDirectory)/"]),
    .linkedLibrary("gb_core_c")
]
#else
linkerSettings = [.linkedLibrary(staticLibPath)]
#endif

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
            linkerSettings: linkerSettings
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
