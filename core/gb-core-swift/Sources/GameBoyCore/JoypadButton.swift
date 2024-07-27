import CGameBoyCore

public enum JoypadButton: Int, CaseIterable {
    case a = 0
    case b = 1
    case select = 2
    case start = 3
    case right = 4
    case left = 5
    case up = 6
    case down = 7

    public var toString: String {
        switch self {
        case .a:
            "A"
        case .b:
            "B"
        case .select:
            "Select"
        case .start:
            "Start"
        case .right:
            "Right"
        case .left:
            "Left"
        case .up:
            "Up"
        case .down:
            "Down"
        }
    }

    var toCoreButton: Button {
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
