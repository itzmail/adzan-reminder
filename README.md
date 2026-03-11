# Adzan Reminder CLI 🕌

Reminder sholat otomatis untuk desktop (macOS, Linux, Windows).

Fitur:
- Jadwal akurat dari API MyQuran
- Notification desktop 5 menit sebelum & tepat waktu
- Suara adzan/bedug
- Daemon background
- Auto-start saat boot

## Install

### Quick Install (macOS / Linux)
Cara paling mudah untuk meng-install Adzan Reminder adalah dengan menjalankan script berikut di terminal:
```bash
curl -fsSL https://raw.githubusercontent.com/itzmail/adzan-reminder/main/install.sh | bash
```

Setelah berhasil terinstall, jalankan:
```bash
adzan
```

### Uninstallation (macOS / Linux)
Untuk menghapus aplikasi Adzan Reminder (termasuk mematikan daemon berjalan dan menghapus konfigurasi), jalankan:
```bash
curl -fsSL https://raw.githubusercontent.com/itzmail/adzan-reminder/main/uninstall.sh | bash
```

### macOS (Homebrew)
```bash
brew tap itzmail/adzan
brew install adzan-reminder
adzan set-city
adzan setup-autostart
```

### Running on Local Machine
1. Clone repository
    ```bash
    git clone https://github.com/itzmail/adzan-reminder
    cd adzan-reminder
    ```
2. Install dependencies
    ```bash
    cargo install --path .
    ```
3. Running from terminal
    ```bash
    cargo run
    ```
4. Build binary
    ```bash
    cargo build --release
    ```
5. Run binary
    ```bash
    ./target/release/adzan-reminder
    ```
