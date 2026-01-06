use notify_rust::Notification;
use rodio::{Decoder, OutputStreamBuilder, Sink};
use std::io::Cursor;
use std::thread;

const ADZAN_BYTES: &[u8] = include_bytes!("../../assets/suara_bedug.mp3");

pub fn send_notification(title: &str, body: &str) {
    let _ = Notification::new()
        .summary(title)
        .body(body)
        .icon("appointment-soon")
        .timeout(notify_rust::Timeout::Milliseconds(12000))
        .show()
        .map_err(|e| eprintln!("Gagal kirim notif: {}", e));
}

pub fn send_prayer_notification(title: &str, body: &str) {
    send_notification(title, body);
}

/// Play adzan MP3 (non-blocking)
pub fn play_adzan() {
    thread::spawn(move || {
        let stream_handle = match OutputStreamBuilder::open_default_stream() {
            Ok(handle) => handle,
            Err(e) => {
                eprintln!("Gagal init audio output: {}", e);
                return;
            }
        };

        let sink = Sink::connect_new(&stream_handle.mixer());

        // 3. Gunakan Cursor untuk membaca bytes dari memory seolah-olah itu file
        let source_cursor = Cursor::new(ADZAN_BYTES);

        match Decoder::new(source_cursor) {
            Ok(source) => {
                sink.append(source);
                sink.sleep_until_end();
            }
            Err(e) => {
                eprintln!("Gagal decode MP3 embedded: {}", e);
            }
        }
    });
}
