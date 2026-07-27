use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContextError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("tokenizer error: {0}")]
    Tokenizer(String),

    #[error("path is not a directory: {0}")]
    NotADirectory(String),
}
