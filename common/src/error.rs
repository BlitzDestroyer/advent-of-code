use thiserror::Error;

#[derive(Debug, Error)]
pub enum PuzzleError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON Error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    Custom(&'static str),
}