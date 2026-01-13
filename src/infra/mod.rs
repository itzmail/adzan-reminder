pub mod repository;

#[cfg(test)]
pub mod mock_repository;

pub use repository::{MyQuranRepository, PrayerRepository};
