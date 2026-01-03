use thiserror::Error;

/// Main application error type
#[derive(Error, Debug)]
pub enum AppError {
    #[error("HTTP request failed: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Failed to parse JSON response: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Failed to access persistent storage: {0}")]
    Store(String),

    #[error("Failed to send notification: {0}")]
    Notification(String),

    #[error("City not found or invalid city ID")]
    CityNotFound,

    #[error("Failed to play adzan audio: {0}")]
    AudioError(String),

    #[error("Unexpected error: {0}")]
    Other(String),
}

impl From<AppError> for String {
    fn from(err: AppError) -> Self {
        err.to_string()
    }
}

// Optional: convert from IO errors (e.g., when reading local files)
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Other(err.to_string())
    }
}
