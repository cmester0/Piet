use clap::Parser as CliParser;
use piet::smpl::SmplExecutor;
use std::fs;
use std::io::Read;

#[derive(CliParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filepath: String,
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();

    let unparsed_file = fs::read_to_string(args.filepath).expect("cannot read file");

    let input = std::io::stdin().bytes().peekable();
    let output = std::io::stdout();
    SmplExecutor::interpret_from_string(unparsed_file.as_str(), &mut Some(input), &mut Some(output));
}
