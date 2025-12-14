use clap::Parser;
use std::env;
// grep-like utility written in rust
// grep generally searches for a given string in a file

#[derive(Parser)]
struct Cli {
    // Pattern to look for, string
    pattern: String,
    // Path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}
