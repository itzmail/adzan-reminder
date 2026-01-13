use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub selected_city_id: Option<String>,
    pub selected_city_name: Option<String>,
}

impl AppConfig {
    pub fn load() -> Result<Self, confy::ConfyError> {
        confy::load("adzan", None)
    }

    pub fn save(&self) -> Result<(), confy::ConfyError> {
        confy::store("adzan", None, self)
    }
}
