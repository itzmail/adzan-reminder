import AppKit

let app = NSApplication.shared
let delegate = AppDelegate()
app.delegate = delegate

// Start socket reader after app launches
var socketReader: SocketReader?
DispatchQueue.main.asyncAfter(deadline: .now() + 0.5) {
    socketReader = SocketReader(delegate: delegate)
    socketReader?.start()
}

app.run()
