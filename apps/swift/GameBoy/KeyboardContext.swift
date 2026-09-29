//
//  KeyboardContext.swift
//  GameBoy
//
//  Created by Marcello Cordeiro on 14/09/2026.
//

import Foundation
import GameBoyCore
import GameController

@MainActor
@Observable
final class KeyboardContext: Sendable {
    let gbContext: GameBoyContext
    var buttonsState = JoypadButton.allCases.map { _ in false }

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
            
            self.setUp(input: input)
        }
    }

    nonisolated
    func setUp(input: GCKeyboardInput) {
        for button in JoypadButton.allCases {
            let mappedTo = button.mappedToGCKeyCode

            input.button(forKeyCode: mappedTo)?.pressedChangedHandler = { _, _, isPressed in
                // The notification block is guaranteed to run in the main queue, so we can safely assume this is correct. In case anything changes we'll get an immediate crash due to Swift's dynamic checking
                MainActor.assumeIsolated {
                    self.buttonsState[button.rawValue] = isPressed
                    self.gbContext.setButton(button: button, value: isPressed)
                }
            }
        }
    }
}
