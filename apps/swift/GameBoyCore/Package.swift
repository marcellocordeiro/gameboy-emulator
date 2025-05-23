// swift-tools-version: 6.1
// The swift-tools-version declares the minimum version of Swift required to build this package.

// TODO: automation
// `cmake --install ./out/build/macos-appleclang --prefix ./core/gb-core-swift/Sources/CGameBoyCore`
// xcodebuild -create-xcframework -library ./lib/libgb_core_c.a -headers ./include -output CGameBoyCore.xcframework

import PackageDescription

#if DEBUG
let configDir = "debug"
#else
let configDir = "release"
#endif

let packageDir = Context.packageDirectory
let rootDir = "\(packageDir)/../../.."
let libsDir = "\(rootDir)/target/\(configDir)"

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
            linkerSettings: [
                .unsafeFlags(["-L\(libsDir)/"]),
            ],
        ),
        .target(
            name: "GameBoyCore",
            dependencies: ["CGameBoyCore"],
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
    ],
)
