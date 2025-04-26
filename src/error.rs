use thiserror::Error;

#[derive(Error, Debug)]
pub enum ADHDError {
    #[error("invalid file extension at path {0}")]
    InvalidFileExtension(String)
}