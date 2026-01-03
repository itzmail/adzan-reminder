# Adzan Reminder CLI 🕌

Reminder sholat otomatis untuk desktop (Linux, macOS, Windows).

Fitur:
- Jadwal akurat dari API MyQuran
- Notification desktop
- Suara adzan/bedug
- Daemon background
- Auto-start saat boot

## Install

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

### macOS (Homebrew) (On Process)
```bash
brew tap ismailnuralam/adzan
brew install adzan-reminder
adzan set-city
adzan setup-autostart
```
