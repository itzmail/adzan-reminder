use rodio::{Decoder, OutputStreamBuilder, Sink};
use std::io::Cursor;
use std::thread;

const BEDUG_BYTES: &[u8] = include_bytes!("../../assets/suara_bedug.mp3");
const ADZAN_SHUBUH_BYTES: &[u8] = include_bytes!("../../assets/Adzan-Shubuh-Abu-Hazim.mp3");
const MECCA_BYTES: &[u8] = include_bytes!("../../assets/mecca_56_22.mp3");

#[cfg(target_os = "macos")]
fn resolve_icon_path() -> String {
    // Cari assets/mosque.icns relatif terhadap executable
    if let Ok(exe_path) = std::env::current_exe() {
        let candidate = exe_path
            .parent()
            .unwrap_or(std::path::Path::new("/"))
            .join("assets/mosque.icns");
        if candidate.exists() {
            return candidate.to_string_lossy().to_string();
        }
    }
    // Fallback: assets di samping binary (installed via install.sh)
    let home_candidate = dirs::home_dir()
        .unwrap_or_default()
        .join(".local/share/adzan/assets/mosque.icns");
    home_candidate.to_string_lossy().to_string()
}

#[cfg(target_os = "macos")]
pub fn show_macos_reminder(title: &str, body: &str) {
    let safe_title = title.replace("\"", "\\\"");
    let safe_body = body.replace("\"", "\\\"");
    let icon_path = resolve_icon_path();

    let script = format!(
        r#"
        try
            display dialog "{safe_body}" with title "{safe_title}" buttons {{"Tutup"}} default button "Tutup" with icon POSIX file "{icon_path}" giving up after 120
        end try
        "#
    );

    let _ = std::process::Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output();
}

#[cfg(target_os = "macos")]
fn show_macos_alert(title: &str, body: &str) -> bool {
    let safe_title = title.replace("\"", "\\\"");
    let safe_body = body.replace("\"", "\\\"");

    let icon_path = resolve_icon_path();

    let script = format!(
        r#"
        try
            set dialogResult to display dialog "{safe_body}" with title "{safe_title}" buttons {{"Tutup & Matikan Audio", "Biarkan"}} default button "Tutup & Matikan Audio" with icon POSIX file "{icon_path}" giving up after 120
            if button returned of dialogResult is "Tutup & Matikan Audio" then
                return "STOP"
            end if
        on error errStr
            return "ERROR: " & errStr
        end try
        "#
    );

    if let Ok(output) = std::process::Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if stdout == "STOP" {
            return true;
        } else if stdout.starts_with("ERROR") {
            eprintln!("AppleScript error: {}", stdout);
        }
    }

    false
}

/// Play adzan MP3 (non-blocking) and wait for AppleScript alert on macOS
pub fn play_adzan(sound_choice: String, alert_body: String) {
    if sound_choice == "mute" {
        return;
    }

    thread::spawn(move || {
        let stream_handle = match OutputStreamBuilder::open_default_stream() {
            Ok(handle) => handle,
            Err(e) => {
                eprintln!("Gagal init audio output: {}", e);
                return;
            }
        };

        let sink = Sink::connect_new(&stream_handle.mixer());

        let bytes = match sound_choice.as_str() {
            "adzan_shubuh" => ADZAN_SHUBUH_BYTES,
            "adzan_mecca" => MECCA_BYTES,
            _ => BEDUG_BYTES, // default to bedug
        };

        let source_cursor = Cursor::new(bytes);

        match Decoder::new(source_cursor) {
            Ok(source) => {
                sink.append(source);
                sink.play();

                #[cfg(target_os = "macos")]
                {
                    // Show a native blocking AppleScript dialog in this thread
                    if show_macos_alert("Waktu Sholat Telah Tiba!", &alert_body) {
                        sink.stop();
                    } else {
                        sink.sleep_until_end();
                    }
                }

                #[cfg(not(target_os = "macos"))]
                {
                    sink.sleep_until_end();
                }
            }
            Err(e) => {
                eprintln!("Gagal decode MP3 embedded: {}", e);
            }
        }

        // Mencegah rodio mencetak pesan "Dropping OutputStream..." yang merusak TUI
        std::mem::forget(stream_handle);
    });
}
