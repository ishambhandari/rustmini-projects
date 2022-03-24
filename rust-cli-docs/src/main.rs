#![allow(unused)]
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}
fn main() {
    let args = Cli::parse();
    let result = std::fs::read_to_string(&args.path);
    let content = match result {
        Ok(content) => content,
        Err(error) => {
            panic!("Something went wrong!!{}", error)
        }
    };

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
