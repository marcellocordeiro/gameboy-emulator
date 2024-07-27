import CGameBoyCore
import Foundation

public final class GameBoy {
    private let gb: OpaquePointer!

    public static let width = Int(SCREEN_WIDTH)
    public static let height = Int(SCREEN_HEIGHT)

    public init(cgb: Bool = false) {
        self.gb = gameboy_new(cgb)
    }

    deinit {
        gameboy_destroy(gb)
    }

    public func load(rom: [UInt8], bootrom: [UInt8]?) {
        gameboy_load(gb, rom, UInt(rom.count), bootrom, UInt(bootrom?.count ?? 0))
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
