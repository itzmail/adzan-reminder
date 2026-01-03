use crate::domain::entities::{DaftarKotaResponse, JadwalResponse, Kota};
use crate::error::AppError;
use async_trait::async_trait;

#[async_trait]
pub trait PrayerRepository: Send + Sync {
    async fn get_all_cities(&self) -> Result<Vec<Kota>, AppError>;
    async fn get_today_schedule(&self, city_id: &str) -> Result<JadwalResponse, AppError>;
}

pub struct MyQuranRepository;

#[async_trait]
impl PrayerRepository for MyQuranRepository {
    async fn get_all_cities(&self) -> Result<Vec<Kota>, AppError> {
        let url = "https://api.myquran.com/v3/sholat/kabkota/semua";
        let response = reqwest::get(url)
            .await?
            .json::<DaftarKotaResponse>()
            .await?;

        if !response.status {
            return Err(AppError::Other(
                "API returned non-success status".to_string(),
            ));
        }

        Ok(response.data)
    }

    async fn get_today_schedule(&self, city_id: &str) -> Result<JadwalResponse, AppError> {
        let tz = "Asia/Jakarta";
        let url = format!(
            "https://api.myquran.com/v3/sholat/jadwal/{}/today?tz={}",
            city_id, tz
        );

        let response = reqwest::get(&url).await?.json::<JadwalResponse>().await?;

        if !response.status {
            return Err(AppError::Other(
                "API returned non-success status".to_string(),
            ));
        }

        Ok(response)
    }
}
