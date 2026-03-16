#!/usr/bin/env bash

set -e

# Warna untuk output
RED='\033[0;31m'
GREEN='\033[0;32m'
CYAN='\033[0;36m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${CYAN}======================================${NC}"
echo -e "${CYAN}   Uninstalasi Adzan Reminder CLI     ${NC}"
echo -e "${CYAN}======================================${NC}"

# 1. Hentikan dan Hapus Daemon/Service Background
OS="$(uname -s)"
echo -e "Mendeteksi OS: ${YELLOW}${OS}${NC}"

if [ "$OS" = "Darwin" ]; then
    PLIST_PATH="$HOME/Library/LaunchAgents/com.adzan.reminder.plist"
    if [ -f "$PLIST_PATH" ]; then
        echo -e "Menghapus service LaunchAgents macOS..."
        launchctl unload "$PLIST_PATH" 2>/dev/null || true
        rm -f "$PLIST_PATH"
        echo -e "${GREEN}Service berhasil dihapus.${NC}"
    else
        echo -e "Service autostart tidak ditemukan, diloncati."
    fi
    # Confy di macOS menyimpan config umumnya di ~/Library/Application Support/
    CONFIG_DIR="$HOME/Library/Application Support/adzan"
elif [ "$OS" = "Linux" ]; then
    SERVICE_PATH="$HOME/.config/systemd/user/adzan-reminder.service"
    if [ -f "$SERVICE_PATH" ]; then
        echo -e "Menghapus service Systemd Linux..."
        systemctl --user stop adzan-reminder.service 2>/dev/null || true
        systemctl --user disable adzan-reminder.service 2>/dev/null || true
        rm -f "$SERVICE_PATH"
        systemctl --user daemon-reload 2>/dev/null || true
        echo -e "${GREEN}Service berhasil dihapus.${NC}"
    else
        echo -e "Service autostart tidak ditemukan, diloncati."
    fi
    CONFIG_DIR="$HOME/.config/adzan"
else
    echo -e "${RED}OS tidak didukung sepenuhnya untuk uninstalasi servis, diloncati.${NC}"
    CONFIG_DIR="$HOME/.config/adzan"
fi

# 2. Hapus Binary
BIN_PATH="$HOME/.local/bin/adzan"
if [ -f "$BIN_PATH" ]; then
    echo -e "Menghapus binary aplikasi..."
    rm -f "$BIN_PATH"
    echo -e "${GREEN}Binary adzan berhasil dihapus.${NC}"
else
    echo -e "Binary adzan tidak ditemukan di ~/.local/bin."
fi

# 3. Hapus Konfigurasi
if [ -d "$CONFIG_DIR" ]; then
    echo -e "Menghapus direktori konfigurasi pengguna..."
    rm -rf "$CONFIG_DIR"
    echo -e "${GREEN}Konfigurasi berhasil dihapus.${NC}"
fi

# 4. Hapus Database dan Assets
DATA_DIR="$HOME/.local/share/adzan"
if [ -d "$DATA_DIR" ]; then
    echo -e "Menghapus direktori data & assets (~/.local/share/adzan)..."
    rm -rf "$DATA_DIR"
    echo -e "${GREEN}Data & assets berhasil dihapus.${NC}"
fi

# Pastikan versi fallback linux hapus juga jika macOS ada config di ~/.config
if [ -d "$HOME/.config/adzan" ]; then
    rm -rf "$HOME/.config/adzan"
fi

echo -e "\n${CYAN}================================================================${NC}"
echo -e "${GREEN}Adzan Reminder CLI telah berhasil dihapus / di-uninstall bersih!${NC}"
echo -e "${CYAN}================================================================${NC}\n"
