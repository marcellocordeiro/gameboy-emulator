// swift-tools-version: 6.1
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

#if DEBUG
let configDir = "debug"
#else
let configDir = "release"
#endif

let packageDir = Context.packageDirectory
let rootDir = "\(packageDir)/../.."
let libsDir = "\(rootDir)/target/\(configDir)"

let libName = "gb_core_c"
let staticLibFile = "lib\(libName).a"

let linkerSettings: [PackageDescription.LinkerSetting]
#if os(macOS)
linkerSettings = [
    .unsafeFlags(["-L\(libsDir)/"]),
    .linkedLibrary(libName),
]
#else
linkerSettings = [.linkedLibrary("\(libsDir)/\(staticLibFile)")]
#endif

let package = Package(
    name: "GameBoyCore",
    platforms: [
        .macOS(.v15),
    ],
    products: [
        .library(name: "GameBoyCore", targets: ["GameBoyCore"]),
    ],
    targets: [
        .target(
            name: "CGameBoyCore",
            swiftSettings: [
                .enableUpcomingFeature("InternalImportsByDefault"),
                .enableUpcomingFeature("MemberImportVisibility"),
                .enableUpcomingFeature("ExistentialAny"),
            ],
            linkerSettings: linkerSettings
        ),
        .target(
            name: "GameBoyCore",
            dependencies: ["CGameBoyCore"],
            swiftSettings: [
                .enableUpcomingFeature("InternalImportsByDefault"),
                .enableUpcomingFeature("MemberImportVisibility"),
                .enableUpcomingFeature("ExistentialAny"),
            ]
        ),
    ]
)
