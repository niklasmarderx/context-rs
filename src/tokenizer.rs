use crate::{ContextError, Result};
use tiktoken_rs::cl100k_base;

pub struct TokenCounter {
    bpe: tiktoken_rs::CoreBPE,
}

impl TokenCounter {
    pub fn new() -> Result<Self> {
        let bpe = cl100k_base().map_err(|e| ContextError::Tokenizer(e.to_string()))?;
        Ok(Self { bpe })
    }

    pub fn count(&self, text: &str) -> usize {
        self.bpe.encode_with_special_tokens(text).len()
    }
}

impl Default for TokenCounter {
    fn default() -> Self {
        Self::new().expect("failed to load cl100k_base tokenizer")
    }
}
