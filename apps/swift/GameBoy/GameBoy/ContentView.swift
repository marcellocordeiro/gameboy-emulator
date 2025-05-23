import GameBoyCore
import SwiftUI
import UniformTypeIdentifiers

struct EmulatorError: LocalizedError {
    let wrapped: any Error

    var errorDescription: String? {
        wrapped.localizedDescription
    }
}

struct ContentView: View {
    @State private var gbContext = GameBoyContext()

    @State private var romPath: URL?
    @State private var showFileImporter = true

    @State private var showAlert = false
    @State private var alertError: EmulatorError?

    var body: some View {
        VStack(spacing: 0) {
            Image(gbContext.texture, scale: 0.25, label: Text("Frame"))

            ButtonsView(
                keyboardContext: .init(gbContext: gbContext),
                showFileImporter: $showFileImporter
            )
            .padding()
        }
        .alert("Error", isPresented: $showAlert, presenting: alertError) { _ in
            Button("OK") {}
        } message: { error in
            Text(error.localizedDescription)
        }
        .fileImporter(
            isPresented: $showFileImporter,
            allowedContentTypes: [
                .init(filenameExtension: "gb")!,
                .init(filenameExtension: "gbc")!,
            ]
        ) { result in
            switch result {
            case let .success(success):
                romPath = success

                do {
                    try gbContext.load(success)
                } catch {
                    alertError = EmulatorError(wrapped: error)
                    showAlert = true
                }
            case let .failure(failure):
                alertError = EmulatorError(wrapped: failure)
                showAlert = true
            }
        }
    }
}

#Preview {
    ContentView()
}
