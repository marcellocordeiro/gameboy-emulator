import GameBoyCore
import SwiftUI

struct ButtonsView: View {
    @State var keyboardContext: KeyboardContext
    @Binding var showFileImporter: Bool

    var body: some View {
        HStack {
            Button("Load game") {
                showFileImporter = true
            }

            Spacer()

            ForEach(JoypadButton.allCases, id: \.self) { button in
                Text(button.toString)
                    .foregroundStyle(
                        keyboardContext.buttonsState[button.rawValue] ? Color.accentColor : Color.primary
                    )
            }
        }
        .background {
            KeyEventHandling()
        }
    }
}

// Prevents the beeps when pressing managed keys.
// TODO: There's probably a better way to handle this.
private struct KeyEventHandling: NSViewRepresentable {
    class KeyView: NSView {
        override var acceptsFirstResponder: Bool { true }

        override func keyDown(with event: NSEvent) {
            let keyCode = event.keyCode
            let keyIsHandled = JoypadButton.allCases.lazy.contains { button in
                button.mappedToCGKeyCode == keyCode
            }

            if !keyIsHandled {
                super.keyDown(with: event)
            }
        }
    }

    func makeNSView(context _: Context) -> NSView {
        let view = KeyView()

        DispatchQueue.main.async {
            view.window?.makeFirstResponder(view)
        }
        return view
    }

    func updateNSView(_: NSView, context _: Context) {}
}

#Preview {
    struct PreviewWrapper: View {
        @State private var gbContext = GameBoyContext()

        var body: some View {
            ButtonsView(
                keyboardContext: .init(gbContext: gbContext),
                showFileImporter: .constant(false)
            )
            .padding()
        }
    }

    return PreviewWrapper()
}
