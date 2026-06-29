import AppKit

let app = NSApplication.shared
let delegate = AppDelegate()
app.delegate = delegate

// Start socket reader after app launches
DispatchQueue.main.asyncAfter(deadline: .now() + 0.5) {
    let reader = SocketReader(delegate: delegate)
    reader.start()
}

app.run()
