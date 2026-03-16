# Adzan Reminder CLI 🕌

Automatic prayer time reminder for desktop (macOS, Linux, Windows).

Features:
- Accurate schedule from the MyQuran API
- Desktop notification 5 minutes before & at prayer time
- Adzan / bedug sound
- Background daemon
- Auto-start at boot
- Bilingual UI (English & Bahasa Indonesia)

## Prerequisites (Linux Only)

For Adzan Reminder to show notifications and alerts on Linux, you need to install `libnotify` and `zenity`.

**Debian / Ubuntu / Mint / PopOS:**
```bash
sudo apt update && sudo apt install libnotify-bin zenity alsa-utils
```

**Arch Linux / Manjaro:**
```bash
sudo pacman -S libnotify zenity alsa-utils
```

**Fedora:**
```bash
sudo dnf install libnotify zenity alsa-utils
```


## Install

### Quick Install (macOS / Linux)
The easiest way to install Adzan Reminder is by running the following script in your terminal:
```bash
curl -fsSL https://raw.githubusercontent.com/itzmail/adzan-reminder/main/install.sh | bash
```

After installation, simply run:
```bash
adzan
```

### Uninstallation (macOS / Linux)
To remove Adzan Reminder (including stopping the running daemon and deleting configuration), run:
```bash
curl -fsSL https://raw.githubusercontent.com/itzmail/adzan-reminder/main/uninstall.sh | bash
```

### Running on Local Machine
1. Clone the repository
    ```bash
    git clone https://github.com/itzmail/adzan-reminder
    cd adzan-reminder
    ```
2. Install dependencies
    ```bash
    cargo install --path .
    ```
3. Run from terminal
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
