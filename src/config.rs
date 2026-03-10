use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub selected_city_id: Option<String>,
    pub selected_city_name: Option<String>,
    pub notification_time: u32,
    pub sound_choice: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            selected_city_id: None,
            selected_city_name: None,
            notification_time: 5,
            sound_choice: "bedug".to_string(),
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self, confy::ConfyError> {
        confy::load("adzan", None)
    }

    pub fn save(&self) -> Result<(), confy::ConfyError> {
        confy::store("adzan", None, self)
    }
}
