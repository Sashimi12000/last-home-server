#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("{0}")]
    ConfyError(#[from] confy::ConfyError),
    #[error("{0}")]
    JsonParse(#[from] serde_json::Error),
    #[error("{0}")]
    JsonWebtoken(#[from] jsonwebtoken::errors::Error),

    #[error("Unexpected error")]
    Unexpected,
}