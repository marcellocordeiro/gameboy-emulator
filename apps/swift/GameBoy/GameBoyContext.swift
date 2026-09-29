//
//  GameBoyContext.swift
//  GameBoy
//
//  Created by Marcello Cordeiro on 14/09/2026.
//

import Combine
import CoreGraphics
import Foundation
import GameBoyCore

@Observable
final class GameBoyContext {
    @ObservationIgnored
    let gb = GameBoy()

    @ObservationIgnored
    var frame: [UInt8]
    var image: CGImage

    @ObservationIgnored
    private var timer: (any Cancellable)?
    
    init() {
        let frame = [UInt8](repeating: 0, count: Int(GameBoy.width * GameBoy.height) * 4)
        
        self.frame = frame
        self.image = Self.bytesToImage(bytes: frame)
    }

    func load(_ url: URL) throws {
        let rom = try [UInt8](Data(contentsOf: url))

        gb.load(bootrom: nil, rom: rom)

        timer = Timer.publish(every: 1 / 60, on: .current, in: .default)
            .autoconnect()
            .sink { _ in
                self.runFrame()
                self.draw()
            }
    }

    func runFrame() {
        gb.runFrame()
    }

    func setButton(button: JoypadButton, value: Bool) {
        gb.setButton(button: button, value: value)
    }

    func draw() {
        gb.draw(frame: &frame)
        image = Self.bytesToImage(bytes: frame)
    }
    
    private static func bytesToImage(bytes: [UInt8]) -> CGImage {
        let width = GameBoy.width
        let height = GameBoy.height
        let bytesPerRow = width * 4
        let space = CGColorSpaceCreateDeviceRGB()
        let bitmapInfo = CGBitmapInfo(rawValue: CGImageAlphaInfo.noneSkipLast.rawValue)
        let provider = CGDataProvider(data: Data(bytes) as CFData)!

        return CGImage(
            width: width,
            height: height,
            bitsPerComponent: 8,
            bitsPerPixel: 32,
            bytesPerRow: bytesPerRow,
            space: space,
            bitmapInfo: bitmapInfo,
            provider: provider,
            decode: nil,
            shouldInterpolate: false,
            intent: .defaultIntent
        )!
    }
}
