use crate::{walker, Chunk, FileChunk, Result, TokenCounter, WalkerConfig};
use std::path::Path;

/// Configuration for the context splitter.
pub struct SplitterConfig {
    /// Maximum tokens per context window (default: 100_000).
    pub max_tokens: usize,
    /// Reserve tokens for system prompt + response headroom (default: 8_000).
    pub headroom: usize,
    pub walker: WalkerConfig,
}

impl Default for SplitterConfig {
    fn default() -> Self {
        Self {
            max_tokens: 100_000,
            headroom: 8_000,
            walker: WalkerConfig::default(),
        }
    }
}

pub struct ContextSplitter {
    config: SplitterConfig,
    counter: TokenCounter,
}

impl ContextSplitter {
    pub fn new(config: SplitterConfig) -> Result<Self> {
        let counter = TokenCounter::new()?;
        Ok(Self { config, counter })
    }

    /// Split the codebase at `root` into context-window-sized chunks.
    pub fn split(&self, root: &Path) -> Result<Vec<Chunk>> {
        if !root.is_dir() {
            return Err(crate::ContextError::NotADirectory(
                root.display().to_string(),
            ));
        }

        let budget = self.config.max_tokens.saturating_sub(self.config.headroom);
        let files = walker::walk(root, &self.config.walker)?;

        let mut chunks: Vec<Chunk> = Vec::new();
        let mut current = Chunk::new(0);

        for (path, content) in files {
            let tokens = self.counter.count(&content);

            // File larger than one window: split by lines
            if tokens > budget {
                if !current.is_empty() {
                    let idx = chunks.len();
                    let mut done = Chunk::new(idx);
                    std::mem::swap(&mut done, &mut current);
                    current = Chunk::new(idx + 1);
                    chunks.push(done);
                }
                let line_chunks = split_large_file(&path, &content, budget, &self.counter);
                let base = chunks.len();
                for (i, (fragment, toks)) in line_chunks.into_iter().enumerate() {
                    let mut c = Chunk::new(base + i);
                    c.push(FileChunk {
                        path: path.clone(),
                        content: fragment,
                        tokens: toks,
                    });
                    chunks.push(c);
                }
                continue;
            }

            if current.total_tokens + tokens > budget && !current.is_empty() {
                let idx = chunks.len();
                let mut done = Chunk::new(idx);
                std::mem::swap(&mut done, &mut current);
                current = Chunk::new(idx + 1);
                chunks.push(done);
            }

            current.push(FileChunk {
                path,
                content,
                tokens,
            });
        }

        if !current.is_empty() {
            let idx = chunks.len();
            current.index = idx;
            chunks.push(current);
        }

        Ok(chunks)
    }
}

fn split_large_file(
    path: &Path,
    content: &str,
    budget: usize,
    counter: &TokenCounter,
) -> Vec<(String, usize)> {
    let lines: Vec<&str> = content.lines().collect();
    let mut result = Vec::new();
    let mut buf = String::new();
    let mut buf_tokens = 0;

    for line in &lines {
        let line_tokens = counter.count(line);
        if buf_tokens + line_tokens > budget && !buf.is_empty() {
            result.push((buf.clone(), buf_tokens));
            buf.clear();
            buf_tokens = 0;
        }
        buf.push_str(line);
        buf.push('\n');
        buf_tokens += line_tokens;
    }
    if !buf.is_empty() {
        result.push((buf, buf_tokens));
    }
    let _ = path;
    result
}
