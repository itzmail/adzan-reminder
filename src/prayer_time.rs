use chrono::{Local, NaiveTime, Timelike};

use crate::domain::entities::JadwalSholat;

const PRAYER_TIMES: [&str; 5] = ["Subuh", "Dzuhur", "Ashar", "Maghrib", "Isya"];

#[derive(Debug, Clone)]
pub struct PrayerTimes {
    pub subuh: NaiveTime,
    pub dzuhur: NaiveTime,
    pub ashar: NaiveTime,
    pub maghrib: NaiveTime,
    pub isya: NaiveTime,
}

impl PrayerTimes {
    pub fn from_schedule(schedule: &crate::domain::entities::JadwalResponse) -> Self {
        // Ambil jadwal hari ini (key pertama dari HashMap)
        let jadwal_map = &schedule.data.jadwal;
        let default_jadwal = JadwalSholat::default();
        let today_jadwal = jadwal_map.values().next().unwrap_or(&default_jadwal);

        let parse = |time_str: &str| {
            NaiveTime::parse_from_str(time_str, "%H:%M")
                .unwrap_or(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
        };

        Self {
            subuh: parse(&today_jadwal.subuh),
            dzuhur: parse(&today_jadwal.dzuhur),
            ashar: parse(&today_jadwal.ashar),
            maghrib: parse(&today_jadwal.maghrib),
            isya: parse(&today_jadwal.isya),
        }
    }

    pub fn check_reminder(&self) -> Option<String> {
        let now = Local::now().time();

        for &prayer in &PRAYER_TIMES {
            let prayer_time = match prayer {
                "Subuh" => self.subuh,
                "Dzuhur" => self.dzuhur,
                "Ashar" => self.ashar,
                "Maghrib" => self.maghrib,
                "Isya" => self.isya,
                _ => continue,
            };

            // Konversi ke total menit dari tengah malam
            let prayer_minutes = prayer_time.hour() as i32 * 60 + prayer_time.minute() as i32;
            let now_minutes = now.hour() as i32 * 60 + now.minute() as i32;

            // Tepat waktu (toleransi 1 menit)
            if now_minutes == prayer_minutes {
                return Some(format!("Waktu {} sekarang! Ayo sholat 🕌", prayer));
            }

            // 5 menit sebelum
            if now_minutes == prayer_minutes - 5 {
                return Some(format!("{} 5 menit lagi! Siap-siap sholat yuk 🕌", prayer));
            }
        }

        None
    }
}
