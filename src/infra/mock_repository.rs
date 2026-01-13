use async_trait::async_trait;
use mockall::mock;

use crate::domain::entities::{JadwalResponse, Kota};
use crate::error::AppError;
use crate::infra::repository::PrayerRepository;

mock! {
  pub PrayerRepository {}
  #[async_trait]
  impl PrayerRepository for PrayerRepository {
    async fn get_all_cities(&self) -> Result<Vec<Kota>, AppError>;
    async fn get_today_schedule(&self, city_id: &str) -> Result<JadwalResponse, AppError>;
  }
}
