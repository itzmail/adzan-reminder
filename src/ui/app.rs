use adzan_lib::{
    domain::entities::{JadwalResponse, Kota},
    AppConfig, PrayerService,
};
use crossterm::event::KeyCode;

#[derive(PartialEq)]
pub enum Tab {
    Dashboard,
    Countdown,
    Settings,
    About,
}

pub enum SettingState {
    Normal,
    SearchingCity {
        query: String,
        results: Vec<Kota>,
        selected_index: usize,
    },
    EditingSound {
        selected_index: usize,
    },
    EditingLanguage {
        selected_index: usize,
    },
    ManagingDaemon {
        selected_index: usize,
    },
    ShowingMessage(String),
}

pub struct App {
    pub active_tab: Tab,
    pub config: AppConfig,
    pub schedule: Option<JadwalResponse>,
    pub selected_menu_index: usize,
    pub selected_settings_index: usize,
    pub setting_state: SettingState,
    pub all_cities: Vec<Kota>,
    pub latest_version: Option<String>, // None = belum dicek, Some = versi terbaru di GitHub
}

impl App {
    pub async fn new() -> App {
        let config = AppConfig::load().unwrap_or_default();
        let mut schedule = None;

        let service = PrayerService::new();
        // Load all cities mapping
        let all_cities = service.get_cities().await.unwrap_or_default();

        if let Some(city_id) = &config.selected_city_id {
            if let Ok(sched) = service.get_today_schedule(city_id).await {
                schedule = Some(sched);
            }
        }

        // Ambil versi terbaru dari GitHub di background thread (non-blocking)
        let latest_version: Option<String> = std::thread::spawn(|| {
            let target = self_update::get_target();
            let bin_name = format!("adzan-{}", target);
            self_update::backends::github::Update::configure()
                .repo_owner("itzmail")
                .repo_name("adzan-reminder")
                .bin_name(&bin_name)
                .bin_path_in_archive("./adzan")
                .no_confirm(true)
                .show_output(false)
                .show_download_progress(false)
                .current_version(env!("CARGO_PKG_VERSION"))
                .build()
                .ok()
                .and_then(|u| u.get_latest_release().ok())
                .map(|r| r.version)
        })
        .join()
        .unwrap_or(None);

        App {
            active_tab: Tab::Dashboard,
            config,
            schedule,
            selected_menu_index: 0,
            selected_settings_index: 0,
            setting_state: SettingState::Normal,
            all_cities,
            latest_version,
        }
    }

    pub async fn handle_key(&mut self, key: KeyCode) -> bool {
        match &mut self.setting_state {
            SettingState::SearchingCity {
                query,
                results,
                selected_index,
            } => {
                match key {
                    KeyCode::Esc => {
                        self.setting_state = SettingState::Normal;
                    }
                    KeyCode::Enter => {
                        if !results.is_empty() && *selected_index < results.len() {
                            let selected_city = &results[*selected_index];
                            self.config.selected_city_id = Some(selected_city.id.clone());
                            self.config.selected_city_name = Some(selected_city.lokasi.clone());
                            let _ = self.config.save();

                            // Reload schedule for new city
                            let service = PrayerService::new();
                            if let Ok(sched) = service.get_today_schedule(&selected_city.id).await {
                                self.schedule = Some(sched);
                            }
                        }
                        self.setting_state = SettingState::Normal;
                    }
                    KeyCode::Up => {
                        if *selected_index > 0 {
                            *selected_index -= 1;
                        } else if !results.is_empty() {
                            *selected_index = results.len() - 1;
                        }
                    }
                    KeyCode::Down => {
                        if !results.is_empty() {
                            *selected_index = (*selected_index + 1) % results.len();
                        }
                    }
                    KeyCode::Backspace => {
                        let mut q = query.clone();
                        q.pop();
                        self.update_city_search(q);
                    }
                    KeyCode::Char(c) => {
                        let mut q = query.clone();
                        q.push(c);
                        self.update_city_search(q);
                    }
                    _ => {}
                }
                return false;
            }
            SettingState::EditingSound { selected_index } => {
                let sounds = ["bedug", "adzan_mecca", "adzan_shubuh", "mute"];
                match key {
                    KeyCode::Esc => {
                        self.setting_state = SettingState::Normal;
                    }
                    KeyCode::Enter => {
                        self.config.sound_choice = sounds[*selected_index].to_string();
                        let _ = self.config.save();
                        self.setting_state = SettingState::Normal;
                    }
                    KeyCode::Up => {
                        if *selected_index > 0 {
                            *selected_index -= 1;
                        } else {
                            *selected_index = sounds.len() - 1;
                        }
                    }
                    KeyCode::Down => {
                        *selected_index = (*selected_index + 1) % sounds.len();
                    }
                    _ => {}
                }
                return false;
            }
            SettingState::EditingLanguage { selected_index } => {
                let langs = ["id", "en"];
                match key {
                    KeyCode::Esc => {
                        self.setting_state = SettingState::Normal;
                    }
                    KeyCode::Enter => {
                        self.config.language = langs[*selected_index].to_string();
                        let _ = self.config.save();
                        self.setting_state = SettingState::Normal;
                    }
                    KeyCode::Up => {
                        if *selected_index > 0 {
                            *selected_index -= 1;
                        } else {
                            *selected_index = langs.len() - 1;
                        }
                    }
                    KeyCode::Down => {
                        *selected_index = (*selected_index + 1) % langs.len();
                    }
                    _ => {}
                }
                return false;
            }
            SettingState::ShowingMessage(_) => {
                match key {
                    KeyCode::Esc | KeyCode::Enter | KeyCode::Char('b') => {
                        self.setting_state = SettingState::Normal;
                    }
                    _ => {}
                }
                return false;
            }
            SettingState::ManagingDaemon { selected_index } => {
                match key {
                    KeyCode::Esc | KeyCode::Char('b') => {
                        self.setting_state = SettingState::Normal;
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if *selected_index > 0 {
                            *selected_index -= 1;
                        } else {
                            *selected_index = 2; // 0: Start, 1: Stop, 2: Cancel
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        *selected_index = (*selected_index + 1) % 3;
                    }
                    KeyCode::Enter => {
                        let result = match selected_index {
                            0 => crate::setup_autostart(),
                            1 => crate::teardown_autostart(),
                            _ => Ok(String::new()), // Cancel
                        };

                        if *selected_index != 2 {
                            let msg = match result {
                                Ok(m) => m,
                                Err(e) => format!("Gagal: {}", e),
                            };
                            self.setting_state = SettingState::ShowingMessage(msg);
                        } else {
                            self.setting_state = SettingState::Normal;
                        }
                    }
                    _ => {}
                }
                return false;
            }
            SettingState::Normal => match key {
                KeyCode::Char('q') => return true,
                KeyCode::Char('b') | KeyCode::Esc => {
                    if self.active_tab == Tab::Dashboard {
                        return true; // Quit
                    } else {
                        self.active_tab = Tab::Dashboard; // Back
                    }
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    if self.active_tab == Tab::Settings && self.selected_settings_index == 1 {
                        self.settings_decrease();
                    }
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    if self.active_tab == Tab::Settings && self.selected_settings_index == 1 {
                        self.settings_increase();
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => self.next_item(),
                KeyCode::Up | KeyCode::Char('k') => self.previous_item(),
                KeyCode::Enter => return self.select_item().await,
                _ => {}
            },
        }
        false
    }

    fn update_city_search(&mut self, new_query: String) {
        let results = if new_query.is_empty() {
            vec![]
        } else {
            let lower_query = new_query.to_lowercase();
            self.all_cities
                .iter()
                .filter(|c| c.lokasi.to_lowercase().contains(&lower_query))
                .take(30)
                .cloned()
                .collect()
        };

        self.setting_state = SettingState::SearchingCity {
            query: new_query,
            results,
            selected_index: 0,
        };
    }

    pub fn next_item(&mut self) {
        match self.active_tab {
            Tab::Dashboard => {
                self.selected_menu_index = (self.selected_menu_index + 1) % 4;
            }
            Tab::Settings => {
                self.selected_settings_index = (self.selected_settings_index + 1) % 7;
            }
            _ => {}
        }
    }

    pub fn previous_item(&mut self) {
        match self.active_tab {
            Tab::Dashboard => {
                if self.selected_menu_index > 0 {
                    self.selected_menu_index -= 1;
                } else {
                    self.selected_menu_index = 3;
                }
            }
            Tab::Settings => {
                if self.selected_settings_index > 0 {
                    self.selected_settings_index -= 1;
                } else {
                    self.selected_settings_index = 6;
                }
            }
            _ => {}
        }
    }

    pub async fn select_item(&mut self) -> bool {
        match self.active_tab {
            Tab::Dashboard => match self.selected_menu_index {
                0 => self.active_tab = Tab::Countdown,
                1 => self.active_tab = Tab::Settings,
                2 => self.active_tab = Tab::About,
                3 => return true,
                _ => {}
            },
            Tab::Settings => {
                match self.selected_settings_index {
                    0 => {
                        self.setting_state = SettingState::SearchingCity {
                            query: String::new(),
                            results: vec![],
                            selected_index: 0,
                        };
                    }
                    1 => {
                        // Update daemon trigger
                        let result = crate::restart_daemon();
                        let msg = match result {
                        Ok(_) => "Notifikasi / Peringatan Awal berhasil diperbarui & service di-restart!".to_string(),
                        Err(e) => format!("Gagal memuat ulang service: {}", e),
                    };
                        self.setting_state = SettingState::ShowingMessage(msg);
                    }
                    2 => {
                        let sounds = ["bedug", "adzan_mecca", "adzan_shubuh", "mute"];
                        let current_pos = sounds
                            .iter()
                            .position(|&s| s == self.config.sound_choice)
                            .unwrap_or(0);
                        self.setting_state = SettingState::EditingSound {
                            selected_index: current_pos,
                        };
                    }
                    3 => {
                        let alert_msg = adzan_lib::helpers::quotes::get_random_message();
                        adzan_lib::helpers::notification::play_adzan(
                            self.config.sound_choice.clone(),
                            alert_msg,
                        );
                    }
                    4 => {
                        self.setting_state = SettingState::ManagingDaemon { selected_index: 0 };
                    }
                    5 => {
                        let t = crate::i18n::get(&self.config.language);
                        let msg = match std::thread::spawn(|| {
                            let target = self_update::get_target();
                            let bin_name = format!("adzan-{}", target);
                            self_update::backends::github::Update::configure()
                                .repo_owner("itzmail")
                                .repo_name("adzan-reminder")
                                .bin_name(&bin_name)
                                .bin_path_in_archive("./adzan")
                                .no_confirm(true)
                                .show_output(false)
                                .show_download_progress(false)
                                .current_version(env!("CARGO_PKG_VERSION"))
                                .build()
                                .and_then(|builder| builder.update())
                        })
                        .join()
                        .unwrap()
                        {
                            Ok(status) => {
                                if status.updated() {
                                    format!("🎉 Berhasil update ke versi {}!\nSilakan tutup (q) dan jalankan ulang aplikasi.", status.version())
                                } else {
                                    format!(
                                        "✅ Aplikasi sudah menggunakan versi terbaru ({}).",
                                        status.version()
                                    )
                                }
                            }
                            Err(e) => format!(
                                "❌ {}: {}",
                                if t.lang_en == "English" {
                                    "Update failed"
                                } else {
                                    "Gagal update"
                                },
                                e
                            ),
                        };
                        self.setting_state = SettingState::ShowingMessage(msg);
                    }
                    6 => {
                        let current_pos = ["id", "en"]
                            .iter()
                            .position(|&l| l == self.config.language)
                            .unwrap_or(0);
                        self.setting_state = SettingState::EditingLanguage {
                            selected_index: current_pos,
                        };
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        false
    }

    pub fn settings_increase(&mut self) {
        if self.active_tab == Tab::Settings {
            match self.selected_settings_index {
                1 => {
                    self.config.notification_time = (self.config.notification_time + 1) % 65;
                    let _ = self.config.save();
                }
                _ => {}
            }
        }
    }

    pub fn settings_decrease(&mut self) {
        if self.active_tab == Tab::Settings {
            match self.selected_settings_index {
                1 => {
                    if self.config.notification_time > 0 {
                        self.config.notification_time -= 1;
                        let _ = self.config.save();
                    }
                }
                _ => {}
            }
        }
    }
}
