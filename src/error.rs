use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChipError {
    #[error("Unable to read this file: {0}")]
    ReadFile(String),
    #[error("Unable to load the program")]
    LoadProgram,
    #[error("Unable to use this API")]
    UseApi,
    #[error("Unable to use this interpreter")]
    UseIntepreter,
    #[error("Unknown error")]
    Unknown
}
