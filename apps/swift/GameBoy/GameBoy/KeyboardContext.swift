import Foundation
import GameBoyCore
import GameController

@MainActor
@Observable
class KeyboardContext: Sendable {
    let gbContext: GameBoyContext
    var buttonsState = JoypadButton.allCases.map { _ in false }

    private var keyboard: GCKeyboard?
    private var input: GCKeyboardInput?

    init(
        gbContext: GameBoyContext
    ) {
        self.gbContext = gbContext

        NotificationCenter.default.addObserver(
            forName: .GCKeyboardDidConnect,
            object: nil,
            queue: .main
        ) { [weak self] notification in
            guard let self else {
                return
            }

            let keyboard = notification.object as! GCKeyboard
            let input = keyboard.keyboardInput!

            MainActor.assumeIsolated {
                self.keyboard = keyboard
                self.input = input

                self.setUp(input: input)
            }
        }
    }

    func setUp(input: GCKeyboardInput) {
        for button in JoypadButton.allCases {
            let mappedTo = button.mappedToGCKeyCode

            input.button(forKeyCode: mappedTo)?.pressedChangedHandler = { _, _, isPressed in
                self.buttonsState[button.rawValue] = isPressed
                self.gbContext.setButton(button: button, value: isPressed)
            }
        }
    }
}
