// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let packageDirectory = Context.packageDirectory
let rootDirectory = "\(packageDirectory)/../.."

#if DEBUG
let libDirectory = "\(rootDirectory)/target/debug"
#else
let libDirectory = "\(rootDirectory)/target/release"
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
    name: "GameBoyCore",
    platforms: [
        .macOS(.v14)
    ],
    products: [
        .library(name: "CGameBoyCore", type: .static, targets: ["CGameBoyCore"]),
        .library(name: "GameBoyCore", type: .static, targets: ["GameBoyCore"]),
    ],
    targets: [
        .target(
            name: "CGameBoyCore",
            dependencies: [],
            linkerSettings: linkerSettings
        ),
        .target(
            name: "GameBoyCore",
            dependencies: ["CGameBoyCore"]
        )
    ]
)
