/// All user-visible strings, available in Indonesian ("id") and English ("en").
/// Call `get(lang)` with the language code from `AppConfig.language`.
pub struct Lang {
    // ── Dashboard ──────────────────────────────────────────────────────────
    pub menu_countdown: &'static str,
    pub menu_settings: &'static str,
    pub menu_about: &'static str,
    pub menu_quit: &'static str,
    pub menu_title: &'static str,
    pub schedule_title: &'static str,
    pub schedule_empty: &'static str,
    pub label_location: &'static str,
    pub label_date: &'static str,

    // ── Settings ───────────────────────────────────────────────────────────
    pub settings_title: &'static str,
    pub settings_list_title: &'static str,
    pub settings_tooltip_title: &'static str,

    pub setting_city: &'static str,
    pub setting_notif_time: &'static str,
    pub setting_notif_time_unit: &'static str,
    pub setting_sound: &'static str,
    pub setting_test: &'static str,
    pub setting_daemon: &'static str,
    pub setting_update: &'static str,
    pub setting_language: &'static str,

    pub tooltip_city: &'static str,
    pub tooltip_notif: &'static str,
    pub tooltip_sound: &'static str,
    pub tooltip_test: &'static str,
    pub tooltip_daemon: &'static str,
    pub tooltip_update: &'static str,
    pub tooltip_language: &'static str,

    // ── Modals ─────────────────────────────────────────────────────────────
    pub modal_city_title: &'static str,
    pub modal_city_results: &'static str,
    pub modal_sound_title: &'static str,
    pub modal_daemon_title: &'static str,
    pub modal_daemon_install: &'static str,
    pub modal_daemon_uninstall: &'static str,
    pub modal_daemon_cancel: &'static str,
    pub modal_info_title: &'static str,
    pub modal_language_title: &'static str,

    // ── Countdown ──────────────────────────────────────────────────────────
    pub countdown_next: &'static str,
    pub countdown_unknown: &'static str,
    pub countdown_all_passed: &'static str,

    // ── About ──────────────────────────────────────────────────────────────
    pub about_title: &'static str,
    pub about_made_by: &'static str,
    pub about_thanks: &'static str,
    pub about_prayer: &'static str,

    // ── Footer ─────────────────────────────────────────────────────────────
    pub footer_general: &'static str,
    pub footer_about: &'static str,

    // ── Language names (shown in language picker) ──────────────────────────
    pub lang_id: &'static str,
    pub lang_en: &'static str,
}

static ID: Lang = Lang {
    // Dashboard
    menu_countdown: "1. Countdown Adzan",
    menu_settings: "2. Pengaturan",
    menu_about: "3. Tentang",
    menu_quit: "4. Keluar",
    menu_title: " Menu Utama ",
    schedule_title: " Jadwal Hari Ini ",
    schedule_empty: "Jadwal belum tersedia. Buka tab Pengaturan untuk set kota.",
    label_location: "Lokasi",
    label_date: "Tanggal",

    // Settings
    settings_title: "\n S E T T I N G S \n",
    settings_list_title: " Daftar Pengaturan ",
    settings_tooltip_title: " Keterangan ",

    setting_city: "1. Atur Kota Lokasi",
    setting_notif_time: "2. Waktu Peringatan Awal",
    setting_notif_time_unit: "Menit",
    setting_sound: "3. Suara Adzan",
    setting_test: "4. Test Notifikasi (Tekan Enter)",
    setting_daemon: "5. Setup Daemon Autostart",
    setting_update: "6. Cek Update Aplikasi (Tekan Enter)",
    setting_language: "7. Bahasa / Language",

    tooltip_city: "Mengubah kota tempat kamu berada.\nAPI akan mencarikan jadwal sholat akurat berdasarkan ID kota tersebut.",
    tooltip_notif: "Fitur pre-reminder.\nMengatur berapa menit aplikasi akan bunyi / memberi notifikasi SEBELUM waktu azan masuk.",
    tooltip_sound: "Pilih suara alarm yang kamu suka (Bedug / Adzan / Mute).",
    tooltip_test: "Mensimulasikan seolah-olah waktu Adzan sudah tiba (memutar audio test).",
    tooltip_daemon: "Mengonfigurasi service OS (Launchd/SystemD) agar reminder jalan sendiri setiap komputer nyala tanpa perlu buka terminal.",
    tooltip_update: "Mengecek pembaruan aplikasi dari GitHub dan langsung mendownload versi terbaru jika tersedia.",
    tooltip_language: "Ganti bahasa tampilan antarmuka.\nPilih antara Bahasa Indonesia atau English.",

    // Modals
    modal_city_title: " Cari Kota (Ketik & Enter) ",
    modal_city_results: " Hasil ",
    modal_sound_title: " Pilih Suara (Up/Down + Enter) ",
    modal_daemon_title: " Setup Background Daemon ",
    modal_daemon_install: "Instal & Start Daemon",
    modal_daemon_uninstall: "Uninstall & Stop Daemon",
    modal_daemon_cancel: "Batal",
    modal_info_title: " Peringatan / Informasi ",
    modal_language_title: " Pilih Bahasa (Up/Down + Enter) ",

    // Countdown
    countdown_next: "  S H O L A T   B E R I K U T N Y A :  ",
    countdown_unknown: "Belum Diketahui",
    countdown_all_passed: "Semua Sudah Lewat",

    // About
    about_title: "\n A B O U T \n",
    about_made_by: "Dibuat dengan ❤️ oleh:",
    about_thanks: "Terima kasih telah menggunakan app ini!",
    about_prayer: "Semoga menjadi amal jariyah 🤲",

    // Footer
    footer_general: "q: Keluar | Enter: Pilih | b/Esc: Kembali",
    footer_about: "q: Keluar | b/Esc: Kembali",

    // Language names
    lang_id: "Bahasa Indonesia",
    lang_en: "English",
};

static EN: Lang = Lang {
    // Dashboard
    menu_countdown: "1. Adzan Countdown",
    menu_settings: "2. Settings",
    menu_about: "3. About",
    menu_quit: "4. Quit",
    menu_title: " Main Menu ",
    schedule_title: " Today's Schedule ",
    schedule_empty: "No schedule yet. Open Settings to set your city.",
    label_location: "Location",
    label_date: "Date",

    // Settings
    settings_title: "\n S E T T I N G S \n",
    settings_list_title: " Settings List ",
    settings_tooltip_title: " Description ",

    setting_city: "1. Set City Location",
    setting_notif_time: "2. Early Reminder Time",
    setting_notif_time_unit: "Minutes",
    setting_sound: "3. Adzan Sound",
    setting_test: "4. Test Notification (Press Enter)",
    setting_daemon: "5. Setup Daemon Autostart",
    setting_update: "6. Check App Update (Press Enter)",
    setting_language: "7. Language / Bahasa",

    tooltip_city: "Change your current city.\nThe API will find the accurate prayer schedule based on the city ID.",
    tooltip_notif: "Pre-reminder feature.\nSet how many minutes before prayer time the app will ring / send a notification.",
    tooltip_sound: "Choose your preferred alarm sound (Bedug / Adzan / Mute).",
    tooltip_test: "Simulate as if prayer time has arrived (plays a test audio).",
    tooltip_daemon: "Configure the OS service (Launchd/SystemD) so the reminder runs automatically every time the computer starts.",
    tooltip_update: "Check for app updates from GitHub and directly download the latest version if available.",
    tooltip_language: "Change the display language of the interface.\nChoose between Bahasa Indonesia or English.",

    // Modals
    modal_city_title: " Search City (Type & Enter) ",
    modal_city_results: " Results ",
    modal_sound_title: " Choose Sound (Up/Down + Enter) ",
    modal_daemon_title: " Background Daemon Setup ",
    modal_daemon_install: "Install & Start Daemon",
    modal_daemon_uninstall: "Uninstall & Stop Daemon",
    modal_daemon_cancel: "Cancel",
    modal_info_title: " Warning / Information ",
    modal_language_title: " Choose Language (Up/Down + Enter) ",

    // Countdown
    countdown_next: "  T H E   N E X T   P R A Y E R :  ",
    countdown_unknown: "Unknown",
    countdown_all_passed: "All Prayers Passed",

    // About
    about_title: "\n A B O U T \n",
    about_made_by: "Made with ❤️ by:",
    about_thanks: "Thank you for using this app!",
    about_prayer: "May it be a charity that continues 🤲",

    // Footer
    footer_general: "q: Quit | Enter: Select | b/Esc: Back",
    footer_about: "q: Quit | b/Esc: Back",

    // Language names
    lang_id: "Bahasa Indonesia",
    lang_en: "English",
};

/// Returns the correct language struct based on the language code.
/// Falls back to Indonesian for any unknown code.
pub fn get(lang: &str) -> &'static Lang {
    match lang {
        "en" => &EN,
        _ => &ID,
    }
}
