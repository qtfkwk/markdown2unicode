use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(about, version, max_term_width = 80)]
struct Cli {
    files: Vec<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    for f in &cli.files {
        let input = std::fs::read_to_string(f).unwrap();
        print!("{}", markdown2unicode::convert(&input));
    }
}
