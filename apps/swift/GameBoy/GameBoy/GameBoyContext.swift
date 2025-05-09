import Combine
import CoreGraphics
import Foundation
import GameBoyCore

@Observable
final class GameBoyContext {
    @ObservationIgnored
    let gb = GameBoy()

    var frame = [UInt8](repeating: 0, count: Int(GameBoy.width * GameBoy.height) * 4)

    @ObservationIgnored
    private var timer: (any Cancellable)?

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
        var frame = [UInt8](repeating: 0, count: Int(GameBoy.width * GameBoy.height) * 4)
        gb.draw(frame: &frame)

        self.frame = frame
    }

    var texture: CGImage {
        let bytesPerRow = GameBoy.width * 4

        let rgbaData = CFDataCreate(nil, frame, frame.count)!
        let provider = CGDataProvider(data: rgbaData)!
        let colorSpace = CGColorSpaceCreateDeviceRGB()
        let bitmapInfo = CGBitmapInfo(rawValue: CGImageAlphaInfo.noneSkipLast.rawValue)

        return CGImage(
            width: GameBoy.width,
            height: GameBoy.height,
            bitsPerComponent: 8,
            bitsPerPixel: 32,
            bytesPerRow: bytesPerRow,
            space: colorSpace,
            bitmapInfo: bitmapInfo,
            provider: provider,
            decode: nil,
            shouldInterpolate: false,
            intent: .defaultIntent
        )!
    }
}
