//
//  JoypadButtons+Extensions.swift
//  GameBoyEmulator
//
//  Created by Marcello Cordeiro on 21/01/2024.
//

import Foundation
import GameBoyCore
import GameController
import SwiftUI

extension JoypadButton {
    var mappedToGCKeyCode: GCKeyCode {
        switch self {
        case .a:
            .keyX
        case .b:
            .keyZ
        case .select:
            .deleteOrBackspace
        case .start:
            .returnOrEnter
        case .right:
            .rightArrow
        case .left:
            .leftArrow
        case .up:
            .upArrow
        case .down:
            .downArrow
        }
    }
    
    var mappedToCGKeyCode: CGKeyCode {
        switch self {
        case .a:
            7
        case .b:
            6
        case .select:
            51
        case .start:
            36
        case .right:
            124
        case .left:
            123
        case .up:
            126
        case .down:
            125
        }
    }
    
    var mappedToKeyEquivalent: KeyEquivalent {
        switch self {
        case .a:
            .init("x")
        case .b:
            .init("z")
        case .select:
            .delete
        case .start:
            .return
        case .right:
            .rightArrow
        case .left:
            .leftArrow
        case .up:
            .upArrow
        case .down:
            .downArrow
        }
    }
}
