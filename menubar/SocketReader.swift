import Foundation
import AppKit

class SocketReader {
    private let socketPath = "/tmp/adzan-menubar.sock"
    private weak var delegate: AppDelegate?

    init(delegate: AppDelegate) {
        self.delegate = delegate
    }

    func start() {
        DispatchQueue.global(qos: .background).async { [weak self] in
            self?.connect()
        }
    }

    private func connect() {
        while true {
            let sock = socket(AF_UNIX, SOCK_STREAM, 0)
            guard sock >= 0 else { sleep(2); continue }

            var addr = sockaddr_un()
            addr.sun_family = sa_family_t(AF_UNIX)
            withUnsafeMutablePointer(to: &addr.sun_path) { ptr in
                ptr.withMemoryRebound(to: CChar.self, capacity: 108) { charPtr in
                    socketPath.withCString { src in _ = strcpy(charPtr, src) }
                }
            }
            let addrLen = socklen_t(MemoryLayout<sockaddr_un>.size)

            let connected = withUnsafePointer(to: &addr) {
                $0.withMemoryRebound(to: sockaddr.self, capacity: 1) {
                    Foundation.connect(sock, $0, addrLen)
                }
            }

            if connected != 0 { close(sock); sleep(2); continue }

            // Read newline-delimited JSON lines
            guard let file = fdopen(sock, "r") else { close(sock); sleep(2); continue }
            var buffer = [CChar](repeating: 0, count: 4096)
            while fgets(&buffer, 4096, file) != nil {
                let line = String(cString: buffer).trimmingCharacters(in: .whitespacesAndNewlines)
                if let data = line.data(using: .utf8),
                   let json = try? JSONSerialization.jsonObject(with: data) as? [String: String] {
                    DispatchQueue.main.async { [weak self] in
                        self?.delegate?.cityItem.title = "📍 \(json["city"] ?? "")"
                        let prayer = json["prayer"] ?? "Sholat"
                        let countdown = json["countdown"] ?? "--:--:--"
                        self?.delegate?.countdownItem.title = "\(prayer) dalam \(countdown)"
                    }
                }
            }
            fclose(file)
            sleep(2) // reconnect
        }
    }
}
