use clap::Parser;
use piet::piet::*;
use std::io::Read;
use image::open;

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

    interpret(open(args.filepath).unwrap(), &mut Some(input), &mut Some(output));
}
