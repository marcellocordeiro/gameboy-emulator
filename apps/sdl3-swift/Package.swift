// swift-tools-version: 6.4
// The swift-tools-version declares the minimum version of Swift required to build this package.

import Foundation
import PackageDescription

let package = Package(
    name: "GameBoy",
    platforms: [
        .macOS(.v27),
    ],
    dependencies: [
        .package(name: "GameBoyCore", path: "../../bindings/swift"),
    ],
    targets: [
        .executableTarget(
            name: "GameBoy",
            dependencies: [
                .product(name: "GameBoyCore", package: "GameBoyCore"),
                "CSDL3",
            ],
            swiftSettings: [
                .enableUpcomingFeature("InternalImportsByDefault"),
                .enableUpcomingFeature("MemberImportVisibility"),
                .enableUpcomingFeature("ExistentialAny"),
            ],
        ),
        .systemLibrary(
            name: "CSDL3",
            pkgConfig: "sdl3",
            // providers: [.brew(["sdl3"])]
        ),
    ],
)
