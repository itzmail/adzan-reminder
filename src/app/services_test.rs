#[cfg(test)]
mod tests {
    use super::super::super::domain::entities::*;
    use super::super::super::infra::mock_repository::MockPrayerRepository;
    use super::super::PrayerService;

    #[tokio::test]
    async fn test_get_cities_returns_mock_data() {
        let mut mock_repo = MockPrayerRepository::new();

        let expected_cities = vec![
            Kota {
                id: "1".to_string(),
                lokasi: "Jakarta".to_string(),
            },
            Kota {
                id: "2".to_string(),
                lokasi: "Bandung".to_string(),
            },
        ];

        mock_repo
            .expect_get_all_cities()
            .returning(move || Ok(expected_cities.clone()));

        let service = PrayerService::with_repo(Box::new(mock_repo));

        let result = service.get_cities().await;

        assert!(result.is_ok());
        let cities = result.unwrap();
        assert_eq!(cities.len(), 2);
        assert_eq!(cities[0].lokasi, "Jakarta");
    }
}
