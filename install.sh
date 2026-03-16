#!/usr/bin/env bash

set -e

# Warna untuk output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Konfigurasi repositori
REPO="itzmail/adzan-reminder"
BIN_NAME="adzan"
INSTALL_DIR="$HOME/.local/bin"

echo -e "${CYAN}==================================${NC}"
echo -e "${CYAN}   Memulai Instalasi Adzan CLI    ${NC}"
echo -e "${CYAN}==================================${NC}"

# 1. Mendeteksi OS dan Arsitektur
OS="$(uname -s)"
ARCH="$(uname -m)"

map_arch() {
    local arch=$1
    case "$arch" in
        x86_64|amd64) echo "x86_64" ;;
        aarch64|arm64) echo "aarch64" ;;
        *) echo "unsupported"; return 1 ;;
    esac
}

map_os() {
    local os=$1
    case "$os" in
        Darwin) echo "apple-darwin" ;;
        Linux) echo "unknown-linux-gnu" ;;
        *) echo "unsupported"; return 1 ;;
    esac
}

check_linux_dependencies() {
    local missing=()
    if ! command -v notify-send &> /dev/null; then
        missing+=("libnotify")
    fi
    if ! command -v zenity &> /dev/null; then
        missing+=("zenity")
    fi

    if [ ${#missing[@]} -ne 0 ]; then
        echo -e "${YELLOW}Peringatan: Dependensi berikut belum terpasang: ${RED}${missing[*]}${NC}"
        echo -e "${YELLOW}Notifikasi dan alert mungkin tidak berfungsi dengan baik.${NC}"
        echo -e "${YELLOW}Silakan instal melalui package manager distro Anda (cek README.md).${NC}\n"
    fi
}

TARGET_ARCH=$(map_arch "$ARCH")
if [ "$TARGET_ARCH" = "unsupported" ]; then
    echo -e "${RED}Arsitektur sistem '$ARCH' belum didukung.${NC}"
    exit 1
fi

TARGET_OS=$(map_os "$OS")
if [ "$TARGET_OS" = "unsupported" ]; then
    echo -e "${RED}Sistem operasi '$OS' belum didukung.${NC}"
    exit 1
fi

# Format nama file dari github release yang kita buat di release.yml: adzan-<arch>-<os>.tar.gz
TARGET="${TARGET_ARCH}-${TARGET_OS}"
ASSET_NAME="${BIN_NAME}-${TARGET}.tar.gz"

echo -e "Platform terdeteksi: ${YELLOW}${OS} ${ARCH}${NC} (Target: ${TARGET})"

if [ "$OS" = "Linux" ]; then
    check_linux_dependencies
fi

# 2. Mendapatkan versi terbaru (Latest Tag)
echo -e "Mencari versi terbaru di GitHub..."
LATEST_RELEASE=$(curl -sfL "https://api.github.com/repos/$REPO/releases/latest")

if [ -z "$LATEST_RELEASE" ]; then
    echo -e "${RED}Gagal menghubungi GitHub API. Pastikan Anda terhubung ke internet.${NC}"
    exit 1
fi

# Ekstrak tag_name — pastikan hasilnya dimulai dengan 'v' agar tidak salah tangkap
TAG=$(echo "$LATEST_RELEASE" | grep '"tag_name"' | head -1 | sed -E 's/.*"tag_name"[[:space:]]*:[[:space:]]*"([^"]+)".*/\1/')

if [ -z "$TAG" ] || [[ "$TAG" != v* ]]; then
    echo -e "${RED}Gagal mendapatkan data rilis terbaru.${NC}"
    echo -e "${YELLOW}Kemungkinan penyebab:${NC}"
    echo -e "  - Belum ada rilis yang dipublikasikan di repositori ini"
    echo -e "  - GitHub API rate limit tercapai (coba beberapa menit lagi)"
    echo -e "  - Tidak ada koneksi internet"
    if [ -n "$LATEST_RELEASE" ]; then
        echo -e "${YELLOW}Response dari GitHub:${NC} $(echo "$LATEST_RELEASE" | head -3)"
    fi
    exit 1
fi

echo -e "Versi terbaru: ${GREEN}${TAG}${NC}"

DOWNLOAD_URL="https://github.com/$REPO/releases/download/$TAG/$ASSET_NAME"

# 3. Menyiapkan direktori instalasi
mkdir -p "$INSTALL_DIR"
ASSETS_DIR="$HOME/.local/share/adzan/assets"
mkdir -p "$ASSETS_DIR"

# 4. Unduh dan Ekstrak
TMP_DIR=$(mktemp -d)
echo -e "Mengunduh file release ${ASSET_NAME}..."
if curl --fail -sL "$DOWNLOAD_URL" -o "$TMP_DIR/$ASSET_NAME"; then
    echo -e "Berhasil diunduh. Mengekstrak..."
    tar -xzf "$TMP_DIR/$ASSET_NAME" -C "$TMP_DIR"
    
    echo -e "Memasang $BIN_NAME ke $INSTALL_DIR..."
    mv "$TMP_DIR/$BIN_NAME" "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/$BIN_NAME"
    
    # Copy assets jika ada
    if [ -d "$TMP_DIR/assets" ]; then
        echo -e "Memasang assets ke $ASSETS_DIR..."
        cp -r "$TMP_DIR/assets/"* "$ASSETS_DIR/"
        echo -e "${GREEN}Assets berhasil dipasang!${NC}"
    else
        echo -e "${YELLOW}Assets tidak ditemukan dalam package (menggunakan embedded assets).${NC}"
    fi
    
    rm -rf "$TMP_DIR"
else
    echo -e "${RED}Gagal mengunduh file release dari GitHub.${NC}"
    echo -e "File ${ASSET_NAME} mungkin tidak ada di rilis ${TAG}."
    rm -rf "$TMP_DIR"
    exit 1
fi

echo -e "\n${GREEN}Instalasi Selesai!${NC} 🚀"
echo -e "Aplikasi '${BIN_NAME}' telah dipasang di ${YELLOW}${INSTALL_DIR}${NC}"

# Cek apakah target ~/.local/bin sudah ada di PATH
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    echo -e "\n${RED}PERHATIAN:${NC} Direktori ${YELLOW}~/.local/bin${NC} belum ada di PATH Anda."
    echo -e "Silakan tambahkan perintah berikut ke file profile Anda (misal ~/.bashrc atau ~/.zshrc):"
    echo -e "  ${CYAN}export PATH=\"\$HOME/.local/bin:\$PATH\"${NC}"
    echo -e "Lalu jalankan ${CYAN}source ~/.bashrc${NC} atau buka terminal baru."
fi

echo -e "\n${CYAN}================================================================${NC}"
echo -e "Langkah Selanjutnya:"
echo -e "1. Jalankan aplikasi dengan perintah: ${YELLOW}${BIN_NAME}${NC}"
echo -e "2. Untuk mengatur kota dan notifikasi, masuk ke menu ${YELLOW}Settings${NC} di TUI."
echo -e "3. Agar alarm dan notifikasi berjalan terus di background setiap komputer nyala,"
echo -e "   jalankan di terminal: ${YELLOW}${BIN_NAME} setup-autostart${NC}"
echo -e "${CYAN}================================================================${NC}\n"
