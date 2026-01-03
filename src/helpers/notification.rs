use notify_rust::Notification;
use rodio::{Decoder, OutputStreamBuilder, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::thread;

/// Kirim notification desktop
pub fn send_notification(title: &str, body: &str) {
    let _ = Notification::new()
        .summary(title)
        .body(body)
        .icon("appointment-soon")
        .timeout(notify_rust::Timeout::Milliseconds(12000))
        .show()
        .map_err(|e| eprintln!("Gagal kirim notif: {}", e));
}

/// Alias untuk send_notification dengan nama yang diharapkan oleh lib.rs
pub fn send_prayer_notification(title: &str, body: &str) {
    send_notification(title, body);
}

/// Play adzan MP3 (non-blocking)
pub fn play_adzan(adzan_path: &Path) {
    if !adzan_path.exists() {
        eprintln!("File adzan tidak ditemukan: {:?}", adzan_path);
        return;
    }

    // Convert Path ke PathBuf agar bisa di-move ke dalam thread
    let path_to_move = adzan_path.to_path_buf();

    thread::spawn(move || {
        // 1. Inisialisasi OutputStream dan Handle.
        // Variabel stream_handle HARUS tetap ada (tidak boleh kena drop) selama pemutaran.
        let stream_handle = match OutputStreamBuilder::open_default_stream() {
            Ok(handle) => handle,
            Err(e) => {
                eprintln!("Gagal init audio output: {}", e);
                return;
            }
        };

        // 2. Buat Sink menggunakan stream_handle
        let sink = Sink::connect_new(&stream_handle.mixer());

        // 3. Buka file
        let file = match File::open(&path_to_move) {
            Ok(f) => BufReader::new(f),
            Err(e) => {
                eprintln!("Gagal buka file: {}", e);
                return;
            }
        };

        // 4. Decode dan mainkan
        match Decoder::new(file) {
            Ok(source) => {
                sink.append(source);
                // Menunggu audio selesai sebelum thread berakhir dan _stream kena drop
                sink.sleep_until_end();
            }
            Err(e) => {
                eprintln!("Gagal decode MP3: {}", e);
            }
        }
    });
}