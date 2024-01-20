import CGameBoyCore
import Foundation

extension GameBoyButton: Equatable, Hashable {}
extension GameBoyButton {
    public var allCases: [Self] {
        [
            A,
            B,
            SELECT,
            START,
            RIGHT,
            LEFT,
            UP,
            DOWN
        ]
    }
}

public final class GameBoy {
    let gb = gameboy_new()
    
    public enum Button: Int, CaseIterable {
        case a = 0
        case b = 1
        case select = 2
        case start = 3
        case right = 4
        case left = 5
        case up = 6
        case down = 7
        
        var toGameBoyButton: GameBoyButton {
            switch self {
            case .a:
                A
            case .b:
                B
            case .select:
                SELECT
            case .start:
                START
            case .right:
                RIGHT
            case .left:
                LEFT
            case .up:
                UP
            case .down:
                DOWN
            }
        }
    }
    
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
    
    public func setButton(button: Button, value: Bool) {
        gameboy_set_key(gb, button.toGameBoyButton, value)
    }
    
    public func draw(frame: inout [UInt8]) {
        gameboy_draw_into_frame_rgba8888(gb, &frame)
    }
}
