import CGameBoyCore
import Foundation

public final class GameBoy {
    private let gb: UnsafeMutablePointer<CGameBoyCore.GameBoy>!

    public static let width = SCREEN_WIDTH
    public static let height = SCREEN_HEIGHT

    public init(cgb: Bool = false) {
        self.gb = gameboy_new(cgb)
    }

    deinit {
        gameboy_destroy(gb)
    }

    public func load(bootrom: [UInt8]?, rom: [UInt8]) {
        let bootromPointer = bootrom?.withUnsafeBufferPointer { $0.baseAddress }
        let romPointer = rom.withUnsafeBufferPointer { $0.baseAddress }

        let gbBootrom = Bootrom(data: bootromPointer, size: bootrom?.count ?? 0)
        let gbRom = Rom(data: romPointer, size: rom.count)

        gameboy_load(gb, gbBootrom, gbRom)
    }

    public func runFrame() {
        gameboy_run_frame(gb)
    }

    public func setButton(button: JoypadButton, value: Bool) {
        gameboy_set_joypad_button(gb, button.toCoreButton, value)
    }

    public func draw(frame: inout [UInt8]) {
        gameboy_draw_into_frame_rgba8888(gb, &frame)
    }
}
