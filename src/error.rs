use thiserror::Error;

#[derive(Error, Debug)]
pub enum LrcLibError {
    #[error("Could not find track with name {name:?}")]
    TrackNotFound { name: String },
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error),
}
