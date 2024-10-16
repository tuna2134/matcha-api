use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("JPreprocess error: {0}")]
    JPreprocessError(#[from] jpreprocess::error::JPreprocessError),
    #[error("Ort error: {0}")]
    OrtError(#[from] ort::Error),
    #[error("Ndarray error: {0}")]
    NdArrayError(#[from] ndarray::ShapeError),
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Hound error: {0}")]
    HoundError(#[from] hound::Error),
    #[error("other")]
    OtherError(String),
}

pub type Result<T> = std::result::Result<T, Error>;
