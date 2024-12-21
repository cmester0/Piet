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
    #[arg(short, long)]
    steps_per_frame: Option<usize>,
}

fn main() {
    let args = Args::parse();
    let steps_per_frame = args.steps_per_frame.unwrap_or(1);

    handle_piet(open(args.filepath).unwrap(), None, true, args.gui, steps_per_frame);
}
