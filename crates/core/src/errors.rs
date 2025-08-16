use thiserror::Error;

#[derive(Error, Debug)]
pub enum AstralError {
    #[error("invalid dimension: expected {expected}, got {got}")]
    InvalidDim { expected: usize, got: usize },

    #[error("not found: {0}")]
    NotFound(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, AstralError>;
