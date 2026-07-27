use context_rs::{ContextSplitter, SplitterConfig};
use std::fs;
use tempfile::TempDir;

fn make_repo(files: &[(&str, &str)]) -> TempDir {
    let dir = TempDir::new().unwrap();
    for (name, content) in files {
        let path = dir.path().join(name);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, content).unwrap();
    }
    dir
}

#[test]
fn test_single_file_single_chunk() {
    let repo = make_repo(&[("main.rs", "fn main() { println!(\"hello\"); }")]);
    let splitter = ContextSplitter::new(SplitterConfig::default()).unwrap();
    let chunks = splitter.split(repo.path()).unwrap();
    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0].files.len(), 1);
}

#[test]
fn test_multiple_files_fit_one_chunk() {
    let repo = make_repo(&[
        ("a.rs", "fn a() {}"),
        ("b.rs", "fn b() {}"),
        ("c.rs", "fn c() {}"),
    ]);
    let splitter = ContextSplitter::new(SplitterConfig::default()).unwrap();
    let chunks = splitter.split(repo.path()).unwrap();
    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0].files.len(), 3);
}

#[test]
fn test_chunk_splits_at_token_budget() {
    // Each file has ~20 tokens; budget is 50 → forces a split
    let content = "fn placeholder() { let x = 1 + 2; println!(\"{}\", x); }";
    let repo = make_repo(&[
        ("a.rs", content),
        ("b.rs", content),
        ("c.rs", content),
        ("d.rs", content),
    ]);
    let config = SplitterConfig {
        max_tokens: 60,
        headroom: 10,
        ..Default::default()
    };
    let splitter = ContextSplitter::new(config).unwrap();
    let chunks = splitter.split(repo.path()).unwrap();
    assert!(chunks.len() > 1, "expected multiple chunks");
    for chunk in &chunks {
        assert!(!chunk.is_empty());
    }
}

#[test]
fn test_chunk_render_contains_path_header() {
    let repo = make_repo(&[("src/lib.rs", "pub fn hello() {}")]);
    let splitter = ContextSplitter::new(SplitterConfig::default()).unwrap();
    let chunks = splitter.split(repo.path()).unwrap();
    let rendered = chunks[0].render();
    assert!(rendered.contains("###"), "expected file header");
    assert!(rendered.contains("lib.rs"));
}

#[test]
fn test_not_a_directory_error() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("file.rs");
    fs::write(&file, "fn x() {}").unwrap();
    let splitter = ContextSplitter::new(SplitterConfig::default()).unwrap();
    let result = splitter.split(&file);
    assert!(result.is_err());
}

#[test]
fn test_total_tokens_matches_sum() {
    let repo = make_repo(&[("a.rs", "fn a() {}"), ("b.rs", "fn b() {}")]);
    let splitter = ContextSplitter::new(SplitterConfig::default()).unwrap();
    let chunks = splitter.split(repo.path()).unwrap();
    for chunk in &chunks {
        let sum: usize = chunk.files.iter().map(|f| f.tokens).sum();
        assert_eq!(chunk.total_tokens, sum);
    }
}
