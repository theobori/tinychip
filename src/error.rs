use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChipError {
    #[error("Unable to read this file: {0}")]
    ReadFile(String),
    #[error("Unknown error")]
    Unknown,
}
