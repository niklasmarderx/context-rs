pub mod chunk;
pub mod error;
pub mod splitter;
pub mod tokenizer;
pub mod walker;

pub use chunk::{Chunk, FileChunk};
pub use error::ContextError;
pub use splitter::{ContextSplitter, SplitterConfig};
pub use tokenizer::TokenCounter;
pub use walker::WalkerConfig;

pub type Result<T> = std::result::Result<T, ContextError>;
