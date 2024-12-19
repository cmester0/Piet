use clap::Parser;
use piet::piet::*;
use image::open;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filepath: String,
    #[arg(short, long)]
    gui: bool,
}

fn main() {
    let args = Args::parse();

    handle_piet(open(args.filepath).unwrap(), None, true, args.gui);
}
