use thiserror::Error;

#[derive(Error, Debug)]
pub enum TmfError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("API returned error {status}: {body}")]
    Api { status: u16, body: String },

    #[error("Failed to deserialize response: {0}")]
    Deserialize(String),

    #[error("Service unavailable: {0}")]
    Unavailable(String),

    #[error("Configuration error: {0}")]
    Config(String),
}

pub type Result<T> = std::result::Result<T, TmfError>;
