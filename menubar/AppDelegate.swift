import AppKit

class AppDelegate: NSObject, NSApplicationDelegate {
    var statusItem: NSStatusItem!
    var menu: NSMenu!
    var countdownItem: NSMenuItem!
    var cityItem: NSMenuItem!

    func applicationDidFinishLaunching(_ notification: Notification) {
        NSApp.setActivationPolicy(.accessory) // no dock icon

        statusItem = NSStatusBar.system.statusItem(withLength: NSStatusItem.variableLength)
        statusItem.button?.title = "🕌"

        menu = NSMenu()

        cityItem = NSMenuItem(title: "Loading...", action: nil, keyEquivalent: "")
        cityItem.isEnabled = false
        menu.addItem(cityItem)

        countdownItem = NSMenuItem(title: "--:--:--", action: nil, keyEquivalent: "")
        countdownItem.isEnabled = false
        menu.addItem(countdownItem)

        menu.addItem(.separator())

        let settingsItem = NSMenuItem(title: "Settings", action: #selector(openSettings), keyEquivalent: ",")
        settingsItem.target = self
        menu.addItem(settingsItem)

        menu.addItem(.separator())

        let quitItem = NSMenuItem(title: "Quit", action: #selector(NSApplication.terminate(_:)), keyEquivalent: "q")
        menu.addItem(quitItem)

        statusItem.menu = menu
    }

    @objc func openSettings() {
        // Find the adzan binary (same dir as this binary)
        let selfPath = Bundle.main.executablePath ?? ""
        let binDir = (selfPath as NSString).deletingLastPathComponent
        let adzanBin = "\(binDir)/adzan"

        let script = "tell application \"Terminal\" to do script \"\(adzanBin)\""
        var error: NSDictionary?
        NSAppleScript(source: script)?.executeAndReturnError(&error)
    }
}
