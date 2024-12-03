use clap::Parser;
use piet::piet::*;
use std::io::Read;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filepath: String,
}

fn main() {
    let args = Args::parse();

    let input = std::io::stdin().bytes().peekable();
    let output = std::io::stdout();

    interpret(args.filepath.as_str(), &mut Some(input), &mut Some(output));
}
