use clap::Parser as CliParser;

use std::fs;
use std::io::Read;
use std::collections::HashMap;

use piet::piet_stack::*;

#[derive(CliParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filepath: String,
}

fn main() {
    let args = Args::parse();

    let unparsed_file = fs::read_to_string(args.filepath).expect("cannot read file");

    let mut input = std::io::stdin().bytes().peekable();
    PietStackExecutor::interpret_from_string(unparsed_file.as_str(), &mut input);
}
