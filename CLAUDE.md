# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Role

Senior Rust developer. Idiomatic Rust: prefer `?` over `unwrap()`, `thiserror` for errors, no unnecessary clones, leverage ownership.

## Commands

```bash
# Build
cargo build
cargo build --release

# Run (opens TUI if terminal attached, daemon mode if not)
cargo run

# Run with subcommand
cargo run -- today
cargo run -- daemon
cargo run -- current-city
cargo run -- update
cargo run -- setup-autostart
cargo run -- teardown-autostart
cargo run -- about

# Test
cargo test
cargo test -- --nocapture                          # show println output
cargo test <test_name>                             # single test
cargo test -p adzan_lib                            # lib tests only

# Lint / Format
cargo clippy -- -D warnings
cargo fmt

# Cross-compile for Linux aarch64 (requires `cargo install cross`)
cross build --release --target aarch64-unknown-linux-gnu

# Linux: needs libasound2-dev for rodio (ALSA)
sudo apt-get install libasound2-dev
```

## Architecture

Dual-crate workspace in a single `Cargo.toml`:
- **`adzan_lib`** (`src/lib.rs`) — library crate, all core logic
- **`adzan`** binary (`src/main.rs`) — thin entry point + TUI module

```
src/
├── main.rs           # Entry: CLI dispatch → TUI | daemon based on args/tty
├── i18n.rs           # Static bilingual string structs (ID / EN)
├── lib.rs            # Re-exports public surface of adzan_lib
│
├── domain/
│   ├── entities.rs   # Serde structs: JadwalSholat, JadwalResponse, Kota, etc.
│   └── usecase.rs    # (placeholder) domain use-case traits
│
├── config.rs         # AppConfig via `confy` (city, sound, notification_time, language)
├── error.rs          # AppError enum via `thiserror`
├── prayer_time.rs    # PrayerTimes: parse schedule → next_prayer(), check_reminder()
│
├── app/
│   ├── services.rs   # PrayerService: async HTTP calls to MyQuran API
│   └── services_test.rs # Integration tests with mockall
│
├── helpers/
│   ├── notification.rs  # play_adzan() (rodio), macOS AppleScript alert, Linux zenity dialog
│   ├── quotes.rs        # Random Islamic quotes / motivational messages for notifications
│   └── serde_helpers.rs # string_or_null etc.
│
├── infra/
│   ├── http.rs           # reqwest HTTP client wrapper
│   ├── repository.rs     # Concrete API repository impl
│   ├── mock_repository.rs # mockall mock for tests
│   └── notifier.rs       # OS notification dispatch (notify-rust or system command)
│
└── ui/
    ├── mod.rs    # run_tui() entry, event loop (crossterm + tokio)
    ├── app.rs    # App state machine: Tab enum, SettingState enum, App struct
    └── render.rs # ratatui rendering logic
```

## Key Design Points

**Entry dispatch** (`main.rs`): no args + tty → TUI; no args + no tty → daemon (for launchd/systemd); args → `handle_command()`.

**Config** (`config.rs`): stored via `confy` (TOML). `AppConfig::load()` never panics — falls back to `Default`. Daemon hot-reloads config every loop tick to pick up TUI changes without restart.

**Prayer schedule**: fetched from MyQuran API per city ID. `PrayerTimes::check_reminder()` fires at exact prayer time (±1 min tolerance) and at `notification_time` minutes before. Uses `HashSet` dedup across daemon loop ticks to avoid double-triggering.

**Audio** (`helpers/notification.rs`): sound bytes embedded at compile time (`include_bytes!`). Three sounds: `bedug`, `adzan_mecca`, `adzan_shubuh`. `mute` = skip. macOS uses AppleScript dialog for stop control; Linux uses `zenity`.

**Self-update**: `self_update` crate pulls GitHub releases. Asset naming: `adzan-{target}` (e.g., `adzan-aarch64-apple-darwin`). Version check is non-blocking (spawned thread) on TUI init.

**TUI** (`ui/`): ratatui + crossterm. `App` owns all state. `SettingState` enum drives modal overlays (city search, sound picker, daemon manager, language picker). City search uses `fuzzy-matcher`.

**i18n** (`i18n.rs`): two static `Translations` structs (ID/EN). `get(lang)` returns the right one. No runtime allocation.

**Error type** (`error.rs`): `AppError` — from `reqwest::Error`, `serde_json::Error`, `std::io::Error`. Use `?` everywhere; only `eprintln!` at call sites in `main.rs`.

## CI / Release

`.github/workflows/` triggers on `v*.*.*` tags. Builds for:
- `x86_64-apple-darwin`, `aarch64-apple-darwin` (native cargo)
- `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu` (`cross`)

Release archive includes `adzan` binary + `assets/` directory (audio files).
