use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(about, version, max_term_width = 80)]
struct Cli {
    /// Input file(s)
    #[arg(short, value_name = "PATH")]
    input_files: Vec<PathBuf>,

    /// Markdown string(s)
    #[arg(value_name = "STRING")]
    input_strings: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    for input in &cli.input_strings {
        println!("{}", markdown2unicode::convert(input));
    }

    for f in &cli.input_files {
        let input = if f.as_os_str() == "-" {
            std::io::read_to_string(std::io::stdin()).unwrap()
        } else {
            std::fs::read_to_string(f).unwrap()
        };

        print!("{}", markdown2unicode::convert(&input));
    }
}
