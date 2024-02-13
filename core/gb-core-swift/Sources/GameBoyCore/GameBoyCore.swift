import CGameBoyCore
import Foundation

public final class GameBoy {
    let gb = gameboy_new()
    
    public static let width = Int(SCREEN_WIDTH)
    public static let height = Int(SCREEN_HEIGHT)
    
    public init() {}
    
    deinit {
        gameboy_destroy(gb)
    }
    
    public func load(_ rom: [UInt8]) {
        gameboy_load_cartridge(gb, rom, UInt(rom.count))
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
