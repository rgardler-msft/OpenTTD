use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("unexpected end of input")]
    UnexpectedEof,
    #[error("invalid data: {0}")]
    InvalidData(String),
}
