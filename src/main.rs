use adzan_lib::helpers::notification::play_adzan;
use adzan_lib::prayer_time::PrayerTimes;
use adzan_lib::{AppConfig, PrayerService};
use atty::Stream;
use chrono::{Local, NaiveTime, Timelike};
use console::style;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use dirs;
use skim::prelude::*;
use skim::Skim;
use std::collections::HashSet;
use std::io::Cursor;
use std::io::Write;
use std::time::Duration;
use std::{fs, thread};

const BANNER: &str = r#"
▄▖ ▌        ▄▖     ▘   ▌      ▄▖▖ ▄▖
▌▌▛▌▀▌▀▌▛▌  ▙▘█▌▛▛▌▌▛▌▛▌█▌▛▘  ▌ ▌ ▐
▛▌▙▌▙▖█▌▌▌  ▌▌▙▖▌▌▌▌▌▌▙▌▙▖▌   ▙▖▙▖▟▖
"#;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        // Ada argumen → langsung jalankan command
        handle_command(&args[1..]).await;
    } else {
        // Tidak ada argumen
        if atty::is(Stream::Stdout) {
            // Ada terminal → tampilkan menu interaktif
            interactive_menu().await;
        } else {
            // Tidak ada terminal → otomatis jalan daemon (untuk launchd/systemd)
            println!("Adzan Reminder daemon jalan di background");
            run_daemon().await;
        }
    }
}

fn print_help() {
    println!("Adzan Reminder CLI");
    println!("Perintah:");
    println!("  today         - Tampilkan jadwal hari ini");
    println!("  set-city      - Pilih kota");
    println!("  current-city  - Lihat kota terpilih");
    println!("  daemon        - Jalankan daemon");
    println!("  setup-autostart - Setup auto-start saat boot");
    println!("  about         - Tentang app");
    println!("Tanpa argumen → menu interaktif");
}

async fn handle_command(args: &[String]) {
    match args[0].as_str() {
        "daemon" => run_daemon().await,
        "setup-autostart" => {
            setup_autostart().unwrap_or_else(|e| eprintln!("Error: {}", e));
        }
        "set-city" => set_city_interactive().await,
        "today" => show_today_schedule().await,
        "current-city" => show_current_city().await,
        "about" => show_about(),
        "--help" | "-h" => print_help(),
        _ => print_help(),
    }
}

async fn interactive_menu() {
    let term = Term::stdout();
    let theme = ColorfulTheme::default();

    loop {
        term.clear_screen().unwrap_or(());
        println!("{}", console::style(BANNER).cyan().bold());
        println!("{}", console::style("Adzan Reminder CLI").green().bold());
        println!();

        let items = vec![
            "1. Tampilkan jadwal hari ini",
            "2. Menu Settings",
            "3. Lihat kota terpilih",
            "4. Run Daemon",
            "5. About",
            "6. Keluar",
        ];

        let selection = Select::with_theme(&theme)
            .items(&items)
            .default(0)
            .interact_on_opt(&term)
            .unwrap_or(None);

        match selection {
            Some(0) => show_today_schedule().await,
            Some(1) => settings_menu().await,
            Some(2) => show_current_city().await,
            Some(3) => run_daemon().await,
            Some(4) => show_about(),
            Some(5) => {
                println!("Keluar dari aplikasi. Semoga bermanfaat! 🕌");
                break;
            }
            None => break,
            _ => unreachable!(),
        }

        println!();
        println!("Tekan Enter untuk kembali ke menu...");
        let _ = term.read_line();
    }
}

async fn settings_menu() {
    let term = Term::stdout();
    let theme = ColorfulTheme::default();

    loop {
        term.clear_screen().unwrap_or(());
        println!("{}", console::style("⚙️  Menu Settings").cyan().bold());
        println!();

        let items = vec![
            "1. Ubah Kota",
            "2. Ubah Waktu Peringatan Awal (Menit)",
            "3. Pilih Suara Adzan",
            "4. Test Notifikasi & Suara",
            "5. Setup Daemon Autostart (OS)",
            "6. Kembali ke Menu Utama",
        ];

        let selection = Select::with_theme(&theme)
            .items(&items)
            .default(0)
            .interact_on_opt(&term)
            .unwrap_or(None);

        match selection {
            Some(0) => set_city_interactive().await,
            Some(1) => set_notification_time_interactive(),
            Some(2) => set_sound_interactive(),
            Some(3) => test_notification_interactive(),
            Some(4) => {
                setup_autostart().unwrap_or_else(|e| eprintln!("Error: {}", e));
                println!("\nTekan Enter untuk melanjutkan...");
                let _ = term.read_line();
            }
            Some(5) | None => break,
            _ => unreachable!(),
        }
    }
}

fn set_notification_time_interactive() {
    let mut config = AppConfig::load().unwrap_or_default();

    let input: String = dialoguer::Input::new()
        .with_prompt("Masukkan menit untuk pengingat sebelum sholat (0 = matikan)")
        .default(config.notification_time.to_string())
        .interact_text()
        .unwrap();

    if let Ok(m) = input.parse::<u32>() {
        config.notification_time = m;
        if let Err(e) = config.save() {
            eprintln!("Gagal menyimpan config: {}", e);
        } else {
            println!(
                "✅ Waktu peringatan berhasil diatur menjadi {} menit sebelum sholat",
                m
            );
        }
    } else {
        println!("❌ Input tidak valid");
    }

    let term = Term::stdout();
    println!("\nTekan Enter untuk melanjutkan...");
    let _ = term.read_line();
}

fn set_sound_interactive() {
    let mut config = AppConfig::load().unwrap_or_default();
    let theme = ColorfulTheme::default();

    println!("Pilih suara untuk Notifikasi Sholat:");
    let sounds = vec!["Bedug", "Adzan Mecca", "Adzan Shubuh Abu Hazim", "Mute"];

    // tentukan default
    let default_idx = match config.sound_choice.as_str() {
        "bedug" => 0,
        "adzan_mecca" => 1,
        "adzan_shubuh" => 2,
        "mute" => 3,
        _ => 0,
    };

    let selection = Select::with_theme(&theme)
        .items(&sounds)
        .default(default_idx)
        .interact()
        .unwrap();

    let choice = match selection {
        0 => "bedug",
        1 => "adzan_mecca",
        2 => "adzan_shubuh",
        3 => "mute",
        _ => "bedug",
    };

    config.sound_choice = choice.to_string();
    if let Err(e) = config.save() {
        eprintln!("Gagal menyimpan config: {}", e);
    } else {
        println!("✅ Suara berhasil diubah ke: {}", sounds[selection]);
    }

    let term = Term::stdout();
    println!("\nTekan Enter untuk melanjutkan...");
    let _ = term.read_line();
}

fn test_notification_interactive() {
    println!("🔔 Menjalankan Test Notifikasi...");

    let config = AppConfig::load().unwrap_or_default();

    let alert_msg = adzan_lib::helpers::quotes::get_random_message();
    adzan_lib::helpers::notification::play_adzan(config.sound_choice.clone(), alert_msg);

    println!("✅ Test selesai dikirim. (Tutup dialog putih untuk mematikan suara uji coba)");

    let term = Term::stdout();
    println!("\nTekan Enter untuk kembali...");
    let _ = term.read_line();
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
            eprintln!("Belum ada kota dipilih. Jalankan 'adzan set-city' dulu.");
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

async fn show_today_schedule() {
    let config = AppConfig::load().unwrap_or_default();

    match config.selected_city_id {
        Some(id) => {
            let service = PrayerService::new();
            let city_name = config
                .selected_city_name
                .as_deref()
                .unwrap_or("Kota tidak diketahui");

            let mut sp = spinners::Spinner::new(
                spinners::Spinners::Dots9,
                format!("Mengambil jadwal untuk {}...", city_name),
            );
            match service.get_today_schedule(id.as_str()).await {
                Ok(schedule) => {
                    let lokasi = &schedule.data.kabko;
                    sp.stop_with_message("✅ Jadwal berhasil dimuat!\n".to_string());

                    println!("Jadwal Sholat Hari Ini - {}", lokasi);
                    println!("──────────────────────────────");

                    // Ambil jadwal untuk hari ini (ambil yang pertama dari HashMap)
                    if let Some((_, jadwal_hari)) = schedule.data.jadwal.iter().next() {
                        println!("Tanggal : {}", jadwal_hari.tanggal);
                        println!("Imsak   : {}", jadwal_hari.imsak);
                        println!("Subuh   : {}", jadwal_hari.subuh);
                        println!("Terbit  : {}", jadwal_hari.terbit);
                        println!("Dhuha   : {}", jadwal_hari.dhuha);
                        println!("Dzuhur  : {}", jadwal_hari.dzuhur);
                        println!("Ashar   : {}", jadwal_hari.ashar);
                        println!("Maghrib : {}", jadwal_hari.maghrib);
                        println!("Isya    : {}", jadwal_hari.isya);

                        // --- Tambahkan Countdown ke Sholat Berikutnya ---
                        println!();
                        println!("⏰ Countdown ke Sholat Berikutnya:");
                        println!("──────────────────────────────");

                        let now = Local::now();
                        let prayer_list: Vec<(&str, &String)> = vec![
                            ("Subuh", &jadwal_hari.subuh),
                            ("Dzuhur", &jadwal_hari.dzuhur),
                            ("Ashar", &jadwal_hari.ashar),
                            ("Maghrib", &jadwal_hari.maghrib),
                            ("Isya", &jadwal_hari.isya),
                        ];

                        let mut found_next = false;

                        for (name, time_str) in &prayer_list {
                            if let Ok(parsed_time) = NaiveTime::parse_from_str(time_str, "%H:%M") {
                                let p_datetime = now
                                    .date_naive()
                                    .and_time(parsed_time)
                                    .and_local_timezone(Local)
                                    .unwrap();

                                if p_datetime > now {
                                    let duration = p_datetime.signed_duration_since(now);
                                    let hours = duration.num_hours();
                                    let minutes = duration.num_minutes() % 60;

                                    println!(
                                        "{} {} <- ({:02} jam {:02} menit lagi)",
                                        console::style(name).green().bold(),
                                        console::style(time_str).cyan(),
                                        hours,
                                        minutes
                                    );
                                    found_next = true;
                                } else {
                                    // Sholat yang sudah lewat
                                    println!(
                                        "{} {} ✓",
                                        console::style(name).dim(),
                                        console::style(time_str).dim()
                                    );
                                }
                            }
                        }

                        if !found_next {
                            println!("🌙 Semua jadwal hari ini sudah terlewati.");
                        }

                        // Cek reminder aktif
                        let prayer_times = PrayerTimes::from_schedule(&schedule);
                        if let Some(message) = prayer_times.check_reminder(config.notification_time)
                        {
                            println!();
                            println!("🔔 Reminder Aktif:");
                            println!("──────────────────────────────");
                            if message.contains("sekarang") {
                                println!(
                                    "{} {}",
                                    console::style("🕌 WAKTU SHOLAT TIBA!").red().bold(),
                                    console::style(&message).yellow()
                                );
                            } else if message.contains("menit lagi") {
                                println!(
                                    "{} {}",
                                    console::style(format!(
                                        "⚠️ {} MENIT LAGI!",
                                        config.notification_time
                                    ))
                                    .yellow()
                                    .bold(),
                                    console::style(&message).cyan()
                                );
                            }
                        }
                    } else {
                        println!("Tidak ada data jadwal tersedia");
                    }
                }
                Err(e) => {
                    sp.stop_with_message(format!("❌ Gagal fetch jadwal: {}\n", e));
                }
            }
        }
        None => {
            println!("Belum ada kota yang dipilih.");
            return;
        }
    }
}

async fn show_current_city() {
    let config = AppConfig::load().unwrap_or_default();
    match config.selected_city_id {
        Some(id) => {
            let name = config
                .selected_city_name
                .as_deref()
                .unwrap_or("Tidak diketahui");
            println!("Kota terpilih: {} ({})", name, id);
        }
        None => println!("Belum ada kota yang dipilih."),
    }
}

async fn set_city_interactive() {
    let mut sp = spinners::Spinner::new(
        spinners::Spinners::Dots,
        "Mengambil list kota dari API...".to_string(),
    );

    let service = PrayerService::new();
    let cities = match service.get_cities().await {
        Ok(c) => {
            sp.stop_with_message("✅ List kota berhasil dimuat!\n".to_string());
            c
        }
        Err(e) => {
            sp.stop_with_message(format!("❌ Gagal fetch list kota: {}\n", e));
            return;
        }
    };

    // Format items sebagai string sederhana (nama kota)
    let input_bytes: Vec<u8> = cities
        .iter()
        .map(|c| format!("{}\n", c.lokasi))
        .collect::<String>()
        .into_bytes();

    let options = SkimOptionsBuilder::default()
        .height("70%".into())
        .multi(false)
        .prompt("Cari kota: ".into())
        .build()
        .unwrap();

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input_bytes));

    let selected = Skim::run_with(&options, Some(items));

    if let Some(output) = selected {
        if output.is_abort {
            println!("Pemilihan dibatalkan.");
            return;
        }

        if let Some(selected_line) = output.selected_items.first() {
            let selected_name = selected_line.output().to_string();

            // Cari kota berdasarkan nama (karena output hanya nama)
            if let Some(chosen) = cities.iter().find(|c| c.lokasi == selected_name) {
                let mut config = AppConfig::load().unwrap_or_default();
                config.selected_city_id = Some(chosen.id.clone());
                config.selected_city_name = Some(chosen.lokasi.clone());

                if let Err(e) = config.save() {
                    eprintln!("Gagal simpan config: {}", e);
                } else {
                    println!("\n✅ Kota berhasil disimpan: {}", chosen.lokasi);
                }
            }
        }
    }
}

fn setup_autostart() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "linux")]
    {
        setup_systemd()?;
        println!("Linux (systemd) setup done!");
        println!("Run this command: \n systemctl --user enable --now adzan-reminder.service");
    }

    #[cfg(target_os = "macos")]
    {
        setup_launchd()?;
        println!("macos (launchd) setup done");
        println!(
            "Run this command: \n launchctl load ~/Library/LaunchAgents/com.adzan.reminder.plist"
        );
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        println!("Autostart setup not supported on this OS.");
        println!("Windows: Buat Task Scheduler dengan 'adzan daemon'")
    }

    println!("Setup done! Adzan Reminder akan jalan otomatis setiap boot.");
    Ok(())
}

#[cfg(target_os = "linux")]
fn setup_systemd() -> Result<(), Box<dyn std::error::Error>> {
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

    println!(
        "✅ Service systemd berhasil dibuat di: {}",
        service_path.display()
    );
    println!();
    println!("📋 Jalankan perintah berikut untuk mengaktifkan:");
    println!();
    println!("┌────────────────────────────────────────────────────────────────┐");
    println!("│ systemctl --user daemon-reload                                │");
    println!("│ systemctl --user enable --now adzan-reminder.service          │");
    println!("└────────────────────────────────────────────────────────────────┘");
    println!();
    println!("💡 Tips: Copy paste perintah di atas satu per satu ke terminal");

    Ok(())
}

#[cfg(target_os = "macos")]
fn setup_launchd() -> Result<(), Box<dyn std::error::Error>> {
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

    println!(
        "✅ Service launchd berhasil dibuat di: {}",
        plist_path.display()
    );
    println!();
    println!("📋 Jalankan perintah berikut untuk mengaktifkan:");
    println!();
    println!("┌─────────────────────────────────────────────────────────────────────────┐");
    println!(
        "│ launchctl load {}                        │",
        plist_path.display()
    );
    println!("└─────────────────────────────────────────────────────────────────────────┘");
    println!();
    println!("💡 Tips: Copy paste perintah di atas ke terminal");
    println!("📝 Log file akan tersimpan di: ~/Library/Logs/adzan.log");

    Ok(())
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
║  GitHub: github.com/ismailnuralam        ║
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
