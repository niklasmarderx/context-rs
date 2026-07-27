# context-rs

[![CI](https://github.com/niklasmarderx/context-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/niklasmarderx/context-rs/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/context-rs)](https://crates.io/crates/context-rs)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Rust stable](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org)

Split a codebase into LLM-friendly context windows — respecting `.gitignore`, counting tokens with `cl100k_base`, and rendering each chunk as a ready-to-paste prompt.

## Features

- **Token-accurate splitting** — uses tiktoken's `cl100k_base` (same as GPT-4 / Claude)
- **Respects `.gitignore`** — powered by the `ignore` crate, same as ripgrep
- **Large-file handling** — files bigger than your budget are split by line, not truncated
- **Sorted, reproducible output** — files are always processed in alphabetical order
- **Zero LLM dependency** — pure Rust library, bring your own API client

## Usage

```toml
[dependencies]
context-rs = "0.1"
```

```rust
use context_rs::{ContextSplitter, SplitterConfig};

let splitter = ContextSplitter::new(SplitterConfig {
    max_tokens: 100_000,
    headroom: 8_000,          // reserved for system prompt + response
    ..Default::default()
})?;

let chunks = splitter.split(std::path::Path::new("./my-project"))?;

for chunk in &chunks {
    println!("Chunk {}: {} files, {} tokens",
        chunk.index,
        chunk.files.len(),
        chunk.total_tokens,
    );
    // chunk.render() → single string with ### path/to/file.rs headers
    let prompt = chunk.render();
}
```

## Config

```rust
use context_rs::{SplitterConfig, WalkerConfig};

let config = SplitterConfig {
    max_tokens: 200_000,    // Claude's 200k window
    headroom: 10_000,
    walker: WalkerConfig {
        extensions: vec!["rs".into(), "toml".into()], // only Rust files
        max_file_bytes: 512 * 1024,                   // skip files > 512 KB
        ..Default::default()
    },
};
```

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
