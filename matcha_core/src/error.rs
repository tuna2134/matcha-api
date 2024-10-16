use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("JPreprocess error: {0}")]
    JPreprocessError(#[from] jpreprocess::error::JPreprocessError),
    #[error("Ort error: {0}")]
    OrtError(#[from] ort::Error),
    #[error("other")]
    OtherError(String),
}

pub type Result<T> = std::result::Result<T, Error>;
