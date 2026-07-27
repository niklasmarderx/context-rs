use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A single file's content with its path and token count.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChunk {
    pub path: PathBuf,
    pub content: String,
    pub tokens: usize,
}

/// A context window: a collection of file chunks that fit within a token budget.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub files: Vec<FileChunk>,
    pub total_tokens: usize,
    pub index: usize,
}

impl Chunk {
    pub fn new(index: usize) -> Self {
        Self {
            files: Vec::new(),
            total_tokens: 0,
            index,
        }
    }

    pub fn push(&mut self, file: FileChunk) {
        self.total_tokens += file.tokens;
        self.files.push(file);
    }

    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    /// Render the chunk as a single LLM-ready string with file headers.
    pub fn render(&self) -> String {
        let mut out = String::new();
        for f in &self.files {
            out.push_str(&format!("### {}\n", f.path.display()));
            out.push_str(&f.content);
            out.push_str("\n\n");
        }
        out
    }
}
