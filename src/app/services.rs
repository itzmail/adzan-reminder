use crate::domain::entities::{JadwalResponse, Kota};
use crate::error::AppError;
use crate::infra::repository::{MyQuranRepository, PrayerRepository};

pub struct PrayerService {
    repo: Box<dyn PrayerRepository>,
}

impl PrayerService {
    pub fn new() -> Self {
        Self {
            repo: Box::new(MyQuranRepository),
        }
    }

    #[cfg(test)]
    pub fn with_repo(repo: Box<dyn PrayerRepository>) -> Self {
        Self { repo }
    }

    pub async fn get_cities(&self) -> Result<Vec<Kota>, AppError> {
        self.repo.get_all_cities().await
    }

    pub async fn get_today_schedule(&self, city_id: &str) -> Result<JadwalResponse, AppError> {
        self.repo.get_today_schedule(city_id).await
    }
}
