use crate::Result;
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

pub struct WalkerConfig {
    /// Additional glob patterns to ignore (on top of .gitignore).
    pub extra_ignore: Vec<String>,
    /// Only include files with these extensions. Empty = all.
    pub extensions: Vec<String>,
    /// Maximum file size in bytes to include (default: 1 MB).
    pub max_file_bytes: u64,
}

impl Default for WalkerConfig {
    fn default() -> Self {
        Self {
            extra_ignore: vec![
                "*.lock".into(),
                "target/**".into(),
                "node_modules/**".into(),
                "dist/**".into(),
                ".git/**".into(),
            ],
            extensions: Vec::new(),
            max_file_bytes: 1024 * 1024,
        }
    }
}

/// Walk a directory respecting .gitignore, returning file paths and their contents.
pub fn walk(root: &Path, config: &WalkerConfig) -> Result<Vec<(PathBuf, String)>> {
    let mut builder = WalkBuilder::new(root);
    builder.git_ignore(true).hidden(true);

    for pat in &config.extra_ignore {
        builder.add_ignore(pat);
    }

    let mut files = Vec::new();

    for entry in builder.build().flatten() {
        let path = entry.path().to_path_buf();

        if !path.is_file() {
            continue;
        }

        if let Ok(meta) = std::fs::metadata(&path) {
            if meta.len() > config.max_file_bytes {
                continue;
            }
        }

        if !config.extensions.is_empty() {
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            if !config.extensions.iter().any(|e| e == ext) {
                continue;
            }
        }

        if let Ok(content) = std::fs::read_to_string(&path) {
            files.push((path, content));
        }
    }

    files.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(files)
}
