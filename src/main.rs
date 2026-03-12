use adzan_lib::helpers::notification::play_adzan;
use adzan_lib::prayer_time::PrayerTimes;
use adzan_lib::{AppConfig, PrayerService};
use atty::Stream;
use chrono::{Local, Timelike};
use console::style;
use dirs;
use std::collections::HashSet;
use std::io::Write;
use std::time::Duration;
use std::{fs, thread};

pub mod i18n;
pub mod ui;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        // Ada argumen → langsung jalankan command
        handle_command(&args[1..]).await;
    } else {
        // Tidak ada argumen
        if atty::is(Stream::Stdout) {
            // Ada terminal → tampilkan TUI (Terminal User Interface)
            if let Err(e) = ui::run_tui().await {
                eprintln!("TUI Error: {}", e);
            }
        } else {
            // Tidak ada terminal → otomatis jalan daemon (untuk launchd/systemd)
            println!("Adzan Reminder daemon jalan di background");
            run_daemon().await;
        }
    }
}

fn print_help() {
    println!("Adzan Reminder CLI (TUI Mode Recommended)");
    println!("Perintah:");
    println!("  today         - Tampilkan jadwal hari ini");
    println!("  set-city      - (Pakai TUI via 'adzan' tanpa argumen untuk UI)");
    println!("  current-city  - Lihat kota terpilih");
    println!("  daemon        - Jalankan daemon");
    println!("  setup-autostart - Setup auto-start saat boot");
    println!("  update        - Update aplikasi ke versi terbaru");
    println!("  about         - Tentang app");
    println!("Tanpa argumen → Buka GUI/TUI Interaktif (Direkomendasikan)");
}

async fn handle_command(args: &[String]) {
    match args[0].as_str() {
        "today" => show_today().await,
        "current-city" => show_current_city(),
        "daemon" => run_daemon().await,
        "setup-autostart" => match setup_autostart() {
            Ok(msg) => println!("{}", msg),
            Err(e) => eprintln!("Error: {}", e),
        },
        "update" => {
            let res = std::thread::spawn(|| run_update().map_err(|e| e.to_string())).join();
            match res {
                Ok(Err(e)) => eprintln!("Update gagal: {}", e),
                Err(e) => eprintln!("Update thread panik! {:#?}", e),
                _ => {}
            }
        }
        "about" => show_about(),
        "--help" | "-h" => print_help(),
        _ => {
            eprintln!("Perintah '{}' tidak dikenal.", args[0]);
            println!();
            print_help();
        }
    }
}

fn run_update() -> Result<(), Box<dyn std::error::Error>> {
    println!("Pencarian update terbaru dari GitHub...");

    // bin_name harus cocok dengan prefix nama asset di GitHub release
    // Format: adzan-{target}, contoh: adzan-aarch64-apple-darwin
    let target = self_update::get_target();
    let bin_name = format!("adzan-{}", target);

    let status = self_update::backends::github::Update::configure()
        .repo_owner("itzmail")
        .repo_name("adzan-reminder")
        .bin_name(&bin_name)
        .bin_path_in_archive("adzan")
        .no_confirm(true)
        .show_download_progress(true)
        .current_version(env!("CARGO_PKG_VERSION"))
        .build()?
        .update()?;

    if status.updated() {
        println!(
            "✅ Berhasil update dari {} ke versi {}!",
            status.version(),
            status.version()
        );
        println!("Silakan jalankan ulang aplikasi.");
    } else {
        println!(
            "Aplikasi sudah menggunakan versi terbaru ({}).",
            status.version()
        );
    }

    Ok(())
}

async fn show_today() {
    let config = AppConfig::load().unwrap_or_default();

    let city_id = match config.selected_city_id.clone() {
        Some(id) => id,
        None => {
            eprintln!("❌ Belum ada kota dipilih. Jalankan 'adzan' untuk set kota di TUI.");
            return;
        }
    };

    let city_name = config.selected_city_name.as_deref().unwrap_or("Kota");

    print!("Mengambil jadwal sholat untuk {}...", city_name);
    std::io::Write::flush(&mut std::io::stdout()).unwrap();

    let service = PrayerService::new();
    let schedule = match service.get_today_schedule(&city_id).await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("\n❌ Gagal mengambil jadwal: {}", e);
            return;
        }
    };

    println!(" ✅");
    println!();

    if let Some((_, jadwal)) = schedule.data.jadwal.iter().next() {
        println!("\n📅 JADWAL SHOLAT — {}", jadwal.tanggal);
        println!("📍 {}", schedule.data.kabko);
        println!("{}", "─".repeat(30)); // Garis pemisah tipis

        // Menggunakan padding sederhana agar waktu tetap sejajar di kanan
        let list_jadwal = [
            ("Subuh", &jadwal.subuh),
            ("Dzuhur", &jadwal.dzuhur),
            ("Ashar", &jadwal.ashar),
            ("Maghrib", &jadwal.maghrib),
            ("Isya", &jadwal.isya),
        ];

        for (nama, waktu) in list_jadwal {
            // {:<10} membuat nama sholat punya lebar 10 karakter rata kiri
            // {:>10} membuat waktu punya lebar 10 karakter rata kanan
            println!("  {:<10} │ {:>10}", nama, waktu);
        }

        println!("{}", "─".repeat(30));
    }
}

fn show_current_city() {
    let config = AppConfig::load().unwrap_or_default();
    match config.selected_city_name {
        Some(name) => println!("📍 Kota aktif: {}", name),
        None => println!("❌ Belum ada kota dipilih. Jalankan 'adzan' untuk set kota di TUI."),
    }
}

async fn run_daemon() {
    println!("🕌 Adzan Reminder daemon mulai...");
    println!("Tekan Ctrl+C untuk berhenti.\n");

    let config = AppConfig::load().unwrap_or_default();

    println!("Setting aktif:");
    println!("- Peringatan awal: {} menit", config.notification_time);
    println!("- Suara sholat:    {}", config.sound_choice);

    let city_id = match config.selected_city_id.clone() {
        Some(id) => id,
        None => {
            eprintln!("Belum ada kota dipilih. Jalankan 'adzan' untuk set kota.");
            return;
        }
    };

    let city_name = config.selected_city_name.as_deref().unwrap_or("Kota");

    let service = PrayerService::new();
    let schedule = match service.get_today_schedule(&city_id).await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Gagal fetch jadwal: {}", e);
            return;
        }
    };

    let prayer_times = PrayerTimes::from_schedule(&schedule);
    println!("Jadwal {} berhasil dimuat. Daemon berjalan...\n", city_name);

    let mut reminded_five_min: HashSet<String> = HashSet::new();
    let mut reminded_exact: HashSet<String> = HashSet::new();

    loop {
        // Here we read config again so it supports hot-reloading setting
        let r_config = AppConfig::load().unwrap_or_default();

        if let Some(message) = prayer_times.check_reminder(r_config.notification_time) {
            let prayer_name = message.split(' ').next().unwrap_or("Sholat").to_string();

            if message.contains("sekarang") {
                if !reminded_exact.contains(&prayer_name) {
                    let alert_msg = adzan_lib::helpers::quotes::get_random_message();
                    play_adzan(r_config.sound_choice.clone(), alert_msg);

                    reminded_exact.insert(prayer_name.clone());

                    println!("🔊 Memainkan Adzan/Suara untuk {}", prayer_name);
                }
            } else if message.contains("menit lagi") {
                if !reminded_five_min.contains(&prayer_name) {
                    #[cfg(target_os = "macos")]
                    adzan_lib::helpers::notification::show_macos_reminder(
                        "Sebentar Lagi Sholat",
                        &message,
                    );

                    reminded_five_min.insert(prayer_name.clone());

                    println!(
                        "⚠️ Reminder {} menit untuk {}",
                        r_config.notification_time, prayer_name
                    );
                }
            }
        }

        // 3. Reset di tengah malam
        let now = Local::now();
        if now.hour() == 0 && now.minute() == 0 {
            reminded_five_min.clear();
            reminded_exact.clear();
            println!("Hari baru — reset reminder.");
        }

        std::thread::sleep(Duration::from_secs(60));
    }
}

pub fn setup_autostart() -> Result<String, Box<dyn std::error::Error>> {
    #[cfg(target_os = "linux")]
    {
        let mut msg = setup_systemd()?;
        msg.push_str("\n\nLinux (systemd) setup done!\nRun this command: \n systemctl --user enable --now adzan-reminder.service");
        Ok(msg)
    }

    #[cfg(target_os = "macos")]
    {
        let mut msg = setup_launchd()?;
        // Try to load, silently failing if already loaded
        let plist_path = dirs::home_dir()
            .unwrap()
            .join("Library/LaunchAgents/com.adzan.reminder.plist");
        let _ = std::process::Command::new("launchctl")
            .arg("load")
            .arg(&plist_path)
            .output();

        msg.push_str("\n\nmacos (launchd) setup done & daemon started");
        Ok(msg)
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        Ok("Autostart setup not supported on this OS.".to_string())
    }
}

pub fn teardown_autostart() -> Result<String, Box<dyn std::error::Error>> {
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("systemctl")
            .args(["--user", "stop", "adzan-reminder.service"])
            .output();
        let _ = std::process::Command::new("systemctl")
            .args(["--user", "disable", "adzan-reminder.service"])
            .output();

        if let Some(config_dir) = dirs::config_dir() {
            let service_path = config_dir.join("systemd/user/adzan-reminder.service");
            if service_path.exists() {
                std::fs::remove_file(service_path)?;
            }
        }
        let _ = std::process::Command::new("systemctl")
            .args(["--user", "daemon-reload"])
            .output();

        return Ok("Daemon berhasil dihentikan dan dihapus dari systemd.".to_string());
    }

    #[cfg(target_os = "macos")]
    {
        let plist_path = dirs::home_dir()
            .unwrap()
            .join("Library/LaunchAgents/com.adzan.reminder.plist");

        let _ = std::process::Command::new("launchctl")
            .arg("unload")
            .arg(&plist_path)
            .output();

        if plist_path.exists() {
            std::fs::remove_file(plist_path)?;
        }

        return Ok("Daemon berhasil dihentikan dan dihapus dari launchd.".to_string());
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        return Ok("Autostart teardown not supported on this OS.".to_string());
    }
}

pub fn restart_daemon() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "linux")]
    {
        let status = std::process::Command::new("systemctl")
            .args(["--user", "restart", "adzan-reminder.service"])
            .status()?;
        if !status.success() {
            return Err("Failed to restart systemd service".into());
        }
    }

    #[cfg(target_os = "macos")]
    {
        // MacOS launchctl doesn't elegantly "restart" agents without unloading/loading
        // using the plist path directly, unless using modern bootout/bootstrap or kickstart.
        // We will safely unload then load.
        let plist_path = dirs::home_dir()
            .unwrap()
            .join("Library/LaunchAgents/com.adzan.reminder.plist");

        let _ = std::process::Command::new("launchctl")
            .arg("unload")
            .arg(&plist_path)
            .output();

        let status = std::process::Command::new("launchctl")
            .arg("load")
            .arg(&plist_path)
            .status()?;

        if !status.success() {
            return Err("Failed to reload macOS launchd service".into());
        }
    }

    Ok(())
}

#[cfg(target_os = "linux")]
fn setup_systemd() -> Result<String, Box<dyn std::error::Error>> {
    let service_content = r#"[Unit]
Description=Adzan Reminder Daemon
After=network.target

[Service]
Type=simple
ExecStart=%h/.local/bin/adzan daemon
Restart=always
RestartSec=10

[Install]
WantedBy=default.target
"#;

    let config_dir = dirs::config_dir().ok_or("Tidak bisa menemukan config dir")?;
    let service_dir = config_dir.join("systemd/user");
    fs::create_dir_all(&service_dir)?;

    let service_path = service_dir.join("adzan-reminder.service");
    let mut file = fs::File::create(&service_path)?;
    file.write_all(service_content.as_bytes())?;

    let msg = format!(
        "✅ Service systemd berhasil dibuat di: {}\n\n\
        📋 Jalankan perintah berikut untuk mengaktifkan:\n\n\
        ┌────────────────────────────────────────────────────────────────┐\n\
        │ systemctl --user daemon-reload                                │\n\
        │ systemctl --user enable --now adzan-reminder.service          │\n\
        └────────────────────────────────────────────────────────────────┘\n\n\
        💡 Tips: Copy paste perintah di atas satu per satu ke terminal",
        service_path.display()
    );

    Ok(msg)
}

#[cfg(target_os = "macos")]
fn setup_launchd() -> Result<String, Box<dyn std::error::Error>> {
    let plist_content = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.adzan.reminder</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
        <string>daemon</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>{}/adzan.log</string>
    <key>StandardErrorPath</key>
    <string>{}/adzan.log</string>
</dict>
</plist>"#,
        std::env::current_exe()?.display(),
        dirs::home_dir().unwrap().join("Library/Logs").display(),
        dirs::home_dir().unwrap().join("Library/Logs").display()
    );

    let launch_agents_dir = dirs::home_dir()
        .ok_or("Tidak bisa menemukan home dir")?
        .join("Library/LaunchAgents");
    fs::create_dir_all(&launch_agents_dir)?;

    let plist_path = launch_agents_dir.join("com.adzan.reminder.plist");
    let mut file = fs::File::create(&plist_path)?;
    file.write_all(plist_content.as_bytes())?;

    let msg = format!(
        "✅ Service launchd berhasil dibuat di: {}\n\n\
        📋 Jalankan perintah berikut untuk mengaktifkan:\n\n\
        ┌─────────────────────────────────────────────────────────────────────────┐\n\
        │ launchctl load {}                        │\n\
        └─────────────────────────────────────────────────────────────────────────┘\n\n\
        💡 Tips: Copy paste perintah di atas ke terminal\n\
        📝 Log file akan tersimpan di: ~/Library/Logs/adzan.log",
        plist_path.display(),
        plist_path.display()
    );

    Ok(msg)
}

fn show_about() {
    // Animasi loading sederhana
    print!("Memuat info pembuat");
    for _ in 0..3 {
        print!(".");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        thread::sleep(Duration::from_millis(500));
    }
    println!("\n");

    let about_text = r#"
╔══════════════════════════════════════════╗
║               ADZAN REMINDER             ║
║                                          ║
║  Dibuat dengan ❤️ oleh:                  ║
║  Ismail Nur Alam                         ║
║  GitHub: github.com/itzmail              ║
║                                          ║
║  "Dan ingatkanlah mereka, karena         ║
║   sesungguhnya peringatan itu            ║
║   bermanfaat bagi orang-orang mukmin."   ║
║  (QS. Adz-Dzariyat: 55)                  ║
║                                          ║
║  Terima kasih telah menggunakan app ini! ║
║  Semoga menjadi amal jariyah 🤲           ║
╚══════════════════════════════════════════╝
"#;

    println!("{}", style(about_text).cyan());

    println!("\nTekan Enter untuk kembali ke menu...");
    let _ = std::io::stdin().read_line(&mut String::new());
}
