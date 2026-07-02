# Adzan Reminder CLI 🕌

Automatic prayer time reminder for desktop (macOS, Linux, Windows).

Features:
- Accurate schedule from the MyQuran API
- Desktop notification 5 minutes before & at prayer time
- Adzan / bedug sound
- Background daemon
- Auto-start at boot
- Bilingual UI (English & Bahasa Indonesia)
- macOS menu bar icon with live countdown

### macOS Menu Bar

When the daemon is running, a 🕌 icon appears in your menu bar. Click it to see:
- Countdown to the next prayer
- Your configured city
- A Settings shortcut to open the TUI

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
This opens the TUI (Text UI). Pick your city in the **Settings** menu, then press `q` to quit — your city & schedule are saved automatically.

## Getting Started (for beginners)

1. **Install** — run the script in the [Install](#install) section above, or `cargo install --path .` if building from source.
2. **Run `adzan`** — open the TUI, go to the **Settings** tab → search & select your city.
3. **Set up autostart** — so Adzan keeps running even after closing the terminal / restarting your computer, run:
   ```bash
   adzan setup-autostart
   ```
   This automatically creates a background service (launchd on macOS, systemd on Linux) and starts it right away. Only needs to be run **once**.
4. Done. Adzan will play automatically according to your city's schedule, even with the app/terminal closed.

Other useful commands:
```bash
adzan today          # show today's prayer schedule
adzan current-city   # show the currently selected city
adzan --help          # show all commands
```

## Setup Auto-Start on macOS (launchctl)

The Adzan daemon can run automatically in the background using `launchd` (macOS's built-in background service mechanism), without needing to keep a Terminal window open.

### Easiest way (recommended)
```bash
adzan setup-autostart
```
This command automatically:
- Creates a service file at `~/Library/LaunchAgents/com.adzan.reminder.plist`
- Runs `launchctl load` so the daemon starts right away
- Makes the daemon start automatically every time your Mac boots/restarts (`RunAtLoad`) and restart itself if it crashes (`KeepAlive`)

You can check the daemon log at:
```bash
cat ~/Library/Logs/adzan.log
```

### Check daemon status
```bash
launchctl list | grep com.adzan.reminder
```
If a line with `com.adzan.reminder` shows up, the daemon is running.

### Stop / remove autostart
Open the TUI (`adzan`) → go to the **Daemon Manager** menu → select **Stop & Remove Autostart**.

Or manually via terminal:
```bash
launchctl unload ~/Library/LaunchAgents/com.adzan.reminder.plist
rm ~/Library/LaunchAgents/com.adzan.reminder.plist
```

### Restart daemon manually (optional)
```bash
launchctl unload ~/Library/LaunchAgents/com.adzan.reminder.plist
launchctl load ~/Library/LaunchAgents/com.adzan.reminder.plist
```

> **Linux:** the same `adzan setup-autostart` command automatically uses `systemd` (user service) instead of `launchctl`.

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
