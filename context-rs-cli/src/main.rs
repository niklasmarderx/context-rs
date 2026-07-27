use clap::Parser;
use llm_window::{ContextSplitter, SplitterConfig, WalkerConfig};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "context-rs",
    about = "Split a codebase into LLM-friendly context windows",
    version
)]
struct Cli {
    /// Path to the codebase directory
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Maximum tokens per context window
    #[arg(short, long, default_value_t = 100_000)]
    max_tokens: usize,

    /// Token headroom reserved for system prompt + response
    #[arg(long, default_value_t = 8_000)]
    headroom: usize,

    /// Only include files with these extensions (e.g. --ext rs --ext toml)
    #[arg(long = "ext")]
    extensions: Vec<String>,

    /// Print each chunk's content instead of the summary
    #[arg(short, long)]
    render: bool,

    /// Output chunk metadata as JSON
    #[arg(short, long)]
    json: bool,
}

fn main() {
    let cli = Cli::parse();

    let config = SplitterConfig {
        max_tokens: cli.max_tokens,
        headroom: cli.headroom,
        walker: WalkerConfig {
            extensions: cli.extensions,
            ..Default::default()
        },
    };

    let splitter = match ContextSplitter::new(config) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    };

    let chunks = match splitter.split(&cli.path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    };

    if cli.json {
        println!("{}", serde_json::to_string_pretty(&chunks).unwrap());
        return;
    }

    println!(
        "{} chunk(s) from {} — budget: {} tokens\n",
        chunks.len(),
        cli.path.display(),
        cli.max_tokens - cli.headroom,
    );

    for chunk in &chunks {
        println!(
            "Chunk {:>2} │ {:>3} files │ {:>6} tokens",
            chunk.index,
            chunk.files.len(),
            chunk.total_tokens,
        );
        if !cli.render {
            for f in &chunk.files {
                println!("         │   {}", f.path.display());
            }
        }
    }

    if cli.render {
        println!();
        for chunk in &chunks {
            println!(
                "━━━ Chunk {} ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
                chunk.index
            );
            print!("{}", chunk.render());
        }
    }
}
