use rodio::{Decoder, OutputStreamBuilder, Sink};
use std::fs;
use std::io::Cursor;
use std::io::Write;
use std::thread;

const BEDUG_BYTES: &[u8] = include_bytes!("../../assets/suara_bedug.mp3");
const ADZAN_SHUBUH_BYTES: &[u8] = include_bytes!("../../assets/Adzan-Shubuh-Abu-Hazim.mp3");
const MECCA_BYTES: &[u8] = include_bytes!("../../assets/mecca_56_22.mp3");
const MOSQUE_ICON_BYTES: &[u8] = include_bytes!("../../assets/mosque.icns");

#[cfg(target_os = "macos")]
fn resolve_icon_path() -> Option<String> {
    if let Ok(exe_path) = std::env::current_exe() {
        let candidate = exe_path
            .parent()
            .unwrap_or(std::path::Path::new("/"))
            .join("assets/mosque.icns");
        if candidate.exists() {
            return Some(candidate.to_string_lossy().replace(':', "/").to_string());
        }
    }

    let home_candidate = dirs::home_dir()
        .unwrap_or_default()
        .join(".local/share/adzan/assets/mosque.icns");

    if home_candidate.exists() {
        return Some(
            home_candidate
                .to_string_lossy()
                .replace(':', "/")
                .to_string(),
        );
    }

    create_temp_icon_from_embedded()
}

#[cfg(target_os = "macos")]
fn create_temp_icon_from_embedded() -> Option<String> {
    if let Ok(temp_dir) = std::env::temp_dir().canonicalize() {
        let temp_icon_path = temp_dir.join("adzan_mosque_icon.icns");

        if let Ok(mut file) = fs::File::create(&temp_icon_path) {
            if file.write_all(MOSQUE_ICON_BYTES).is_ok() {
                return Some(
                    temp_icon_path
                        .to_string_lossy()
                        .replace(':', "/")
                        .to_string(),
                );
            }
        }
    }
    None
}

#[cfg(target_os = "macos")]
pub fn show_macos_reminder(title: &str, body: &str) {
    let safe_title = title.replace("\"", "\\\"");
    let safe_body = body.replace("\"", "\\\"");

    let script = match resolve_icon_path() {
        Some(icon_path) => format!(
            r#"
        try
            display dialog "{safe_body}" with title "{safe_title}" buttons {{"Tutup"}} default button "Tutup" with icon POSIX file "{icon_path}" giving up after 120
        end try
        "#
        ),
        None => format!(
            r#"
        try
            display dialog "{safe_body}" with title "{safe_title}" buttons {{"Tutup"}} default button "Tutup" giving up after 120
        end try
        "#
        ),
    };

    let _ = std::process::Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output();
}

#[cfg(target_os = "macos")]
fn show_macos_alert(title: &str, body: &str) -> bool {
    let safe_title = title.replace("\"", "\\\"");
    let safe_body = body.replace("\"", "\\\"");

    let script = match resolve_icon_path() {
        Some(icon_path) => format!(
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
        ),
        None => format!(
            r#"
        try
            set dialogResult to display dialog "{safe_body}" with title "{safe_title}" buttons {{"Tutup & Matikan Audio", "Biarkan"}} default button "Tutup & Matikan Audio" giving up after 120
            if button returned of dialogResult is "Tutup & Matikan Audio" then
                return "STOP"
            end if
        on error errStr
            return "ERROR: " & errStr
        end try
        "#
        ),
    };

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

pub fn play_adzan(sound_choice: String, alert_body: String) {
    if sound_choice == "mute" {
        return;
    }

    thread::spawn(move || {
        let stream_handle = match OutputStreamBuilder::open_default_stream() {
            Ok(handle) => handle,
            Err(e) => {
                eprintln!("Failed init audio output: {}", e);
                return;
            }
        };

        let sink = Sink::connect_new(&stream_handle.mixer());

        let bytes = match sound_choice.as_str() {
            "adzan_shubuh" => ADZAN_SHUBUH_BYTES,
            "adzan_mecca" => MECCA_BYTES,
            _ => BEDUG_BYTES,
        };

        let source_cursor = Cursor::new(bytes);

        match Decoder::new(source_cursor) {
            Ok(source) => {
                sink.append(source);
                sink.play();

                #[cfg(target_os = "macos")]
                {
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
                eprintln!("Failed decode MP3 embedded: {}", e);
            }
        }

        std::mem::forget(stream_handle);
    });
}
